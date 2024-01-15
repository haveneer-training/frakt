use super::complementary_types::range::Range;
use super::complementary_types::resolution::Resolution;
use super::complementary_types::u8data::U8Data;
use super::{complementary_types::pixeldata::PixelData, fragment_task::FragmentTask};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FragmentResult {
    pub id: U8Data,
    pub resolution: Resolution,
    pub range: Range,
    pub pixels: PixelData,
}

impl FragmentResult {
    pub fn new(
        id: U8Data,
        resolution: Resolution,
        range: Range,
        pixels: PixelData,
    ) -> FragmentResult {
        FragmentResult {
            id,
            resolution,
            range,
            pixels,
        }
    }

    pub fn create(task: &FragmentTask) -> FragmentResult {
        let pixel_data = PixelData::new(
            task.id.offset + task.id.count,
            task.resolution.nx as u32 * task.resolution.ny as u32,
        );
        let id = task.id;
        let resolution = task.resolution;
        let range = task.range;
        return FragmentResult::new(id, resolution, range, pixel_data);
    }

    pub fn serialize(&self) -> String {
        let mut serialized = String::from("{\"FragmentResult\":");
        serialized.push_str(&serde_json::to_string(&self).expect("Could not serialize request"));
        serialized.push('}');
        serialized
    }
}
