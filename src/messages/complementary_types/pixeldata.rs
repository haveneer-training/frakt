use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PixelData {
    offset: u32,
    count: u32,
}

impl PixelData {
    pub fn new(offset: u32, count: u32) -> PixelData {
        PixelData { offset, count }
    }
}