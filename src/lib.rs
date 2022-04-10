mod geo_enums;
pub use geo_enums::GeoType;

use anyhow::{anyhow, Result};

use num_traits::{Num, NumCast};
use std::{fmt, ops, vec};

use geo::algorithm::convex_hull::ConvexHull;
use geo::{Coordinate, Geometry, GeometryCollection, LineString, Polygon};
use rand::prelude::*;

// Basic polygon type
pub struct Framework<T>
where
    T: ops::Mul<Output = T> + fmt::Display + Num + NumCast + Copy + PartialOrd + Clone,
{
    pub lon_min: T,
    pub lon_max: T,
    pub lat_min: T,
    pub lat_max: T,
    pub vertices: usize,
    pub convex_hull: bool,
    pub collection: bool,
}

impl Framework<f64> {
    fn new(
        lons: Vec<f64>,
        lats: Vec<f64>,
        vertices: usize,
        convex_hull: bool,
        collection: bool,
    ) -> Framework<f64> {
        let (lon_min, lon_max) = (lons[0], lons[1]);
        let (lat_min, lat_max) = (lats[0], lats[1]);
        Framework {
            lon_min,
            lon_max,
            lat_min,
            lat_max,
            vertices,
            convex_hull,
            collection,
        }
    }

    pub fn clap_constructor(matches: clap::ArgMatches) -> Framework<f64> {
        let lons = matches.values_of_t("x").expect("Needs lon boundaries!");
        let lats = matches.values_of_t("y").expect("Needs lat boundaries!");
        let vertices = matches.value_of_t("z").expect("Need vertices!");
        let convex_hull = matches.is_present("convex_hull");
        let collection = matches.is_present("collection");
        let f = Framework::new(lons, lats, vertices, convex_hull, collection);
        f
    }

    pub fn describe(&self) {
        println!("lon_min: {}", &self.lon_min);
        println!("Value for vertices: {}", &self.vertices);
    }

    pub fn build(&mut self) -> Result<geo_enums::GeoType<f64>> {
        if self.vertices < 3 {
            return Err(anyhow!("Minimum vertices is 3"));
        }

        let mut coords = Vec::new();
        let mut rng = thread_rng();
        for _ in 0..self.vertices {
            let lon = rng.gen_range(self.lon_min..self.lon_max);
            let lat = rng.gen_range(self.lat_min..self.lat_max);
            let c = Coordinate { x: lon, y: lat };
            coords.push(c)
        }

        let ls = LineString(coords);
        let mut p = Polygon::new(ls, vec![]);
        p = if self.convex_hull { p.convex_hull() } else { p };

        if !self.collection {
            Ok(geo_enums::GeoType::Polygon(p))
        } else {
            let gc = GeometryCollection::new_from(vec![Geometry::Polygon(p)]);
            Ok(geo_enums::GeoType::GeometryCollection(gc))
        }
    }
}

// pub mod logic () {
//     // todo
// }

#[cfg(test)]
mod tests {
    use super::*;
    use geo::algorithm::bounding_rect::BoundingRect;
    use geo::coords_iter::CoordsIter;

    #[test]
    fn make_poly() {
        let p: Framework<f64> = Framework::new(5.5, 6.5, 1.2, 1000., 10);

        let poly: Polygon<f64> = p.build_polygon(true);
        let vertices: bool = poly.coords_count() <= p.vertices as usize;
        assert!(vertices);
        let bounds = poly.bounding_rect().unwrap();

        assert!(p.lon_min <= bounds.min().x);
        assert!(bounds.max().x <= p.lon_max);
        assert!(p.lat_min <= bounds.min().y);
        assert!(bounds.max().y <= p.lat_max);
    }
}
