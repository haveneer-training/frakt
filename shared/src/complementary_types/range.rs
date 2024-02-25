use serde::{Deserialize, Serialize};

use super::point::Point;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Range {
    pub min: Point,
    pub max: Point,
}

impl Range {
    pub fn new(min: Point, max: Point) -> Range {
        Range { min, max }
    }
}
