use serde::{Serialize, Deserialize}; 

use crate::fractal::fractal_types::FreactalDescriptor;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FragmentRequest {
    pub worker_name: String,
    pub maximal_work_load: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct FragmentTask {
    pub id: U8Data,
    pub fractal: FreactalDescriptor,
    pub max_iteration: u16,
    pub resolution: Resolution,
    pub range: Range
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FragmentResult {
    pub id: U8Data,
    pub resolution: Resolution,
    pub range: Range,
    pub pixels: PixelData
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum NetworkProtocoles {
    FragmentRequest(FragmentRequest),
    FragmentTask(FragmentTask),
    FragmentResult(FragmentResult)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct U8Data {
    pub offset: u32,
    pub count: u32
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Complex {
    pub re: f64,
    pub im: f64
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Range {
    pub min: Point,
    pub max: Point
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Resolution {
    pub nx: u16,
    pub ny: u16
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PixelData {
    pub offset: u32,
    pub count: u32
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PixelIntensity {
    pub zn: f32,
    pub count: f32
}
