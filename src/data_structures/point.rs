//`Point` | `x: f64`<br/>`y: f64`  

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    x: f64,
    y: f64
}

impl Point {
    pub fn new(x: f64, y:f64) -> Point {
        Point { x: x, y: y }
    }
}