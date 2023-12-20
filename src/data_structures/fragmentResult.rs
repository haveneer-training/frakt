
use serde::{Serialize, Deserialize};
use crate::data_stuctures::{Resolution, Range, U8Data, PixelData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FragmentResult {
    pub id: U8Data,
    pub resolution: Resolution,
    pub range: Range,
    pub pixels: PixelData,
}