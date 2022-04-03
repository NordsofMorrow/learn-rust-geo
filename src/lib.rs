//! # Polygonify
//!
//! A library for creating random polygons

extern crate geo;
extern crate rand;

use geo::algorithm::convex_hull::ConvexHull;
use geo::*;
use num_traits::Num;
use rand::prelude::*;

pub use geo::Polygon;

pub trait Sane<Bool> {
    fn build_polygon(self) -> bool;
}

// Basic polygon type
pub struct Framework<T>
where
    T: std::ops::Mul<Output = T> + Clone + std::convert::Into<f64> + Copy,
{
    pub lon_min: T,
    pub lon_max: T,
    pub lat_min: T,
    pub lat_max: T,
    pub vertices: i32,
}

impl<T> Framework<T>
where
    T: std::ops::Mul<Output = T> + Clone + std::convert::Into<f64> + Copy,
{
    pub fn new(lon_min: T, lon_max: T, lat_min: T, lat_max: T, vertices: i32) -> Framework<T>
    where
        T: Num + std::convert::Into<f64> + Copy,
    {
        Framework {
            lon_min,
            lon_max,
            lat_min,
            lat_max,
            vertices,
        }
    }

    pub fn build_polygon(&self, hull: bool) -> Polygon<f64> {
        if self.vertices < 3 {
            panic!("Minimum vertices is 3")
        }

        let mut coords = Vec::new();
        let mut rng = thread_rng();
        for _ in 0..self.vertices {
            let lon: f64 = rng.gen_range(self.lon_min.clone().into()..self.lon_max.clone().into());
            let lat: f64 = rng.gen_range(self.lat_min.clone().into()..self.lat_max.clone().into());
            let c = Coordinate { x: lon, y: lat };
            coords.push(c)
        }

        let ls: LineString<f64> = LineString(coords);
        let p: Polygon<f64> = Polygon::new(ls, vec![]);

        if hull {
            p.convex_hull()
        } else {
            p
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
