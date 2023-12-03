use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PixelIntensity {
    zn: f32,
    count: f32,
}

impl PixelIntensity {
    pub fn new(zn: f32, count: f32) -> PixelIntensity {
        PixelIntensity { zn, count }
    }
}