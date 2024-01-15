use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct U8Data {
    pub offset: u32,
    pub count: u32,
}

impl U8Data {
    pub fn new(offset: u32, count: u32) -> U8Data {
        U8Data { offset, count }
    }
}
