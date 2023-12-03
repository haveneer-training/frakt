use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FragmentRequest {
    worker_name: String,
    maximal_work_load: u32,
}

impl FragmentRequest {
    pub fn new(worker_name: String, maximal_work_load: u32) -> FragmentRequest {
        FragmentRequest {
            worker_name,
            maximal_work_load,
        }
    }
}