
use serde::{Serialize, Deserialize};

// `PixelIntensity` | `zn: f32`<br/>`count: f32

pub struct PixelIntensity {
    pub zn: f32,
    pub count: f32,
}

impl PixelIntensity {
    pub fn new(zn: f32, count: f32) -> PixelIntensity {
        PixelIntensity { zn, count }
    }
}