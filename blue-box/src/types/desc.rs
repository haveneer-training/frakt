use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct U8Data {
    pub offset: u32,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Range {
    pub min: Point,
    pub max: Point,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Resolution {
    pub nx: u16,
    pub ny: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PixelData {
    pub offset: u32,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PixelIntensity {
    pub zn: f32,
    pub count: f32,
}
