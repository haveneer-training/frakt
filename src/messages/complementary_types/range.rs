use crate::messages::complementary_types::point::Point;

pub struct Range {
    min: Point,
    max: Point,
}

impl Range {
    pub fn new(min: Point, max: Point) -> Range {
        Range { min, max }
    }
}