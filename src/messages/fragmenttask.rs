//TODO : add fractal
use super::complementary_types::u8data::U8Data;
use super::complementary_types::resolution::Resolution;
use super::complementary_types::range::Range;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FragmentTask {
    id: U8Data,
    //fractal: Fractal,
    max_iteration: u16,
    resolution: Resolution,
    range: Range,
}

impl FragmentTask {
    pub fn new(id: U8Data, max_iteration: u16, resolution: Resolution, range: Range) -> FragmentTask {
        FragmentTask { id, max_iteration, resolution, range }
    }
}