
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FragmentRequest {
    pub worker_name: String,
    pub maximal_work_load: u32,
}