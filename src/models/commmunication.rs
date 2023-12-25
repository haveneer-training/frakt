use serde::{Serialize, Deserialize};

use super::fractal::FreactalDescriptor;

#[derive(Serialize, Deserialize, Debug)]
pub struct FragmentRequest {
    pub worker_name: String,
    pub maximal_work_load: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FragmentTask {
    pub id: U8Data,
    pub fractal: FreactalDescriptor,
    pub max_iteration: u16,
    pub resolution: Resolution,
    pub range: Range
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkProtocoles {
    FragmentRequest(FragmentRequest),
    FragmentTask(FragmentTask)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct U8Data {
    offset: u32,
    count: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Complex {
    re: f64,
    im: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    x: f64,
    y: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Range {
    min: Point,
    max: Point
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resolution {
    nx: u16,
    ny: u16
}

#[derive(Serialize, Deserialize, Debug)]
struct PixelData {
    offset: u32,
    count: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PixelIntensity {
    zn: f32,
    count: f32
}
