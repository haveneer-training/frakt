use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PixelData {
    pub offset: u32,
    pub count: u32,
}

impl PixelData {
    pub fn new(offset: u32, count: u32) -> PixelData {
        PixelData { offset, count }
    }
}
