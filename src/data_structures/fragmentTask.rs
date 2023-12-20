
use serde::{Serialize, Deserialize};
use crate::fractals::{Resolution, Range, U8Data};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FragmentTask {
    pub id: U8Data,
    pub max_iteration: u16,
    pub resolution: Resolution,
    pub range: Range,
}