use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct U8Data {
    offset: u32,
    count: u32,
}

impl U8Data {
    pub fn new(offset: u32, count: u32) -> U8Data {
        U8Data { offset, count }
    }
}