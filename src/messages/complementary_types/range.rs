use crate::messages::complementary_types::point::Point;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Range {
    pub min: Point,
    pub max: Point,
}

impl Range {
    pub fn new(min: Point, max: Point) -> Range {
        Range { min, max }
    }
}
