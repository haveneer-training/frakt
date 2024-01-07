use serde::{Deserialize, Serialize};

use super::{
    desc::{
        U8Data,
        Resolution,
        Range,
        PixelData
    }, 
    fractal_type::FractalDescriptor,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Fragment {
    FragmentRequest(FragmentRequest),
    FragmentTask(FragmentTask),
    FragmentResult(FragmentResult),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FragmentRequest {
    pub worker_name: String,
    pub maximal_work_load: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct FragmentTask {
    pub id: U8Data,
    pub fractal: FractalDescriptor,
    pub max_iteration: u32,
    pub resolution: Resolution,
    pub range: Range,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FragmentResult {
    pub id: U8Data,
    pub resolution: Resolution,
    pub range: Range,
    pub pixels: PixelData,
}

impl FragmentResult {
    pub fn new(id: U8Data, resolution: Resolution, range: Range, pixels: PixelData) -> FragmentResult{
        FragmentResult {
            id,
            resolution,
            range,
            pixels
        }
    }
}
