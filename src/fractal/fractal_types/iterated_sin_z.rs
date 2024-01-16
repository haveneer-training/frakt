use serde::{Deserialize, Serialize};

use crate::{
    fractal::fractal::GetDatas,
    messages::{
        complementary_types::{complex::Complex, pixelintensity::PixelIntensity},
        fragment_task::FragmentTask,
    },
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IteratedSinZ {
    c: Complex,
}

impl GetDatas for IteratedSinZ {
    fn get_datas(&self, task: &FragmentTask) -> Vec<PixelIntensity> {
        return Vec::new();
    }
}
