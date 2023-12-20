
use serde::{Serialize, Deserialize};

// `U8Data`| `offset: u32`<br/>`count: u32`

pub struct U8Data {
    pub offset: u32,
    pub count: u32,
}

impl U8Data {
    pub fn new(offset: u32, count: u32) -> U8Data {
        U8Data { offset, count}
    }
}