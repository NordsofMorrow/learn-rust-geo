use geo::{CoordNum, GeometryCollection, Polygon};
use num_traits::{Num, NumCast};
use std::fmt;

#[derive(Debug)]
pub enum GeoType<T>
where
    T: Num + NumCast + CoordNum + fmt::Debug,
{
    Polygon(Polygon<T>),
    GeometryCollection(GeometryCollection<T>),
}
