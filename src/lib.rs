//! # Polygonify
//!
//! A library for creating random polygons

extern crate geo;
extern crate rand;

use geo::algorithm::convex_hull::ConvexHull;
use geo::*;
// use geojson::Geometry;
// use geojson::Geometry;
use num_traits::{Num, NumCast};
use rand::prelude::*;

pub use geo::{Geometry, GeometryCollection, Polygon};

pub trait Sane<Bool> {
    fn build_polygon(self) -> bool;
}

// Basic polygon type
#[derive(Debug)]

pub struct Framework<T>
where
    T: std::ops::Mul<Output = T> + Num + NumCast + Copy + PartialOrd + Clone + std::fmt::Display,
{
    pub lon_min: T,
    pub lon_max: T,
    pub lat_min: T,
    pub lat_max: T,
    pub vertices: usize,
}

impl<'a, T> Framework<T>
where
    T: std::ops::Mul<Output = T>
        + Clone
        + Num
        + NumCast
        + Copy
        + PartialOrd
        + GeoNum
        + std::fmt::Display
        + std::fmt::Debug
        + rand::distributions::uniform::SampleUniform,
{
    pub fn new(lons: Vec<T>, lats: Vec<T>, vertices: usize) -> Framework<T> {
        let (lon_min, lon_max) = (lons[0], lons[1]);
        let (lat_min, lat_max) = (lats[0], lats[1]);
        Framework {
            lon_min,
            lon_max,
            lat_min,
            lat_max,
            vertices,
        }
    }

    pub fn describe(&self) {
        println!("lon_min: {}", &self.lon_min);

        println!("Value for vertices: {}", &self.vertices);
    }

    pub fn build(self, gc: bool, convex_hull: bool) -> Option<GeoType<T>> {
        let geo_var = match gc {
            true => Some(GeoType::GeometryCollection(
                self.build_geometrycollection(convex_hull),
            )),
            false => Some(GeoType::Polygon(self.build_polygon(convex_hull))),
        };
        geo_var
    }

    fn build_polygon(&self, convex_hull: bool) -> Polygon<T> {
        if self.vertices < 3 {
            panic!("Minimum vertices is 3")
        }

        let mut coords = Vec::new();
        let mut rng = thread_rng();
        for _ in 0..self.vertices {
            let lon = rng.gen_range(self.lon_min..self.lon_max);
            let lat = rng.gen_range(self.lat_min..self.lat_max);
            let c = Coordinate { x: lon, y: lat };
            coords.push(c)
        }

        let ls: LineString<T> = LineString(coords);
        let mut p: Polygon<T> = Polygon::new(ls, vec![]);

        p = if convex_hull { p.convex_hull() } else { p };
        p
    }

    fn build_geometrycollection(&self, convex_hull: bool) -> GeometryCollection<T> {
        let g = GeometryCollection(vec![geo::Geometry::Polygon(
            self.build_polygon(convex_hull),
        )]);
        g
    }
}

#[derive(Debug)]
pub enum GeoType<T>
where
    T: Num + NumCast + Clone + PartialOrd + Copy + std::fmt::Display + std::fmt::Debug,
{
    Polygon(Polygon<T>),
    // Geometry(Geometry<f64>),
    GeometryCollection(GeometryCollection<T>),
}

// pub mod logic () {
//     // todo
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use geo::algorithm::bounding_rect::BoundingRect;
//     use geo::coords_iter::CoordsIter;

//     #[test]
//     fn make_poly() {
//         let p: Framework<f64> = Framework::new(5.5, 6.5, 1.2, 1000., 10);

//         let poly: Polygon<f64> = p.build_polygon(true);
//         let vertices: bool = poly.coords_count() <= p.vertices as usize;
//         assert!(vertices);
//         let bounds = poly.bounding_rect().unwrap();

//         assert!(p.lon_min <= bounds.min().x);
//         assert!(bounds.max().x <= p.lon_max);
//         assert!(p.lat_min <= bounds.min().y);
//         assert!(bounds.max().y <= p.lat_max);
//     }
// }
