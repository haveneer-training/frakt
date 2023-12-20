// `Range`| `min: Point`<br/>`max: Point` 

use serde::{Serialize, Deserialize};
use crate::data_stuctures::point::Point;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub min: Point,
    pub max: Point,
}


impl Range {
    pub fn new(min: Point, max: Point) -> Range {
        Point { min: x, max: y }
    }
}