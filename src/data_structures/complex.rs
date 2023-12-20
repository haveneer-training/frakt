//`Complex`        | `re: f64`<br/>`im: f64` 

use serde::{Serialize, Deserialize};

pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Complex {
        Complex{ re, im }
    }
}