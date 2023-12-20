
use serde::{Serialize, Deserialize};

//`PixelData`| `offset: u32`<br/>`count: u32`

pub struct PixelData {
    pub offset: u32,
    pub count: u32,
}

impl PixelData {
    pub fn new(offset: u32, count: u32) -> PixelData {
        PixelData { offset, count}
    }
}