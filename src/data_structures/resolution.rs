
use serde::{Serialize, Deserialize};

//`nx: u16`<br/>`ny: u16`

pub struct Resolution {
    pub nx: u16,
    pub ny: u16,
}

impl Resolution {
    pub fn new(nx: u16, ny: u16) -> Resolution {
        Resolution { nx: im, ny: im }
    }
}