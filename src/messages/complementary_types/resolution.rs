use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Resolution {
    pub nx: u16,
    pub ny: u16,
}

impl Resolution {
    pub fn new(nx: u16, ny: u16) -> Resolution {
        Resolution { nx, ny }
    }
}
