use serde::{Deserialize, Serialize};

use crate::messages::{
    complementary_types::pixelintensity::PixelIntensity, fragment_task::FragmentTask,
};

use super::fractal_types::{
    iterated_sin_z::IteratedSinZ, julia_descriptor::JuliaDescriptor, mandelbrot::Mandelbrot,
};

pub trait GetDatas {
    fn get_datas(&self, task: &FragmentTask) -> Vec<PixelIntensity>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FractalDescriptor {
    Julia(JuliaDescriptor),
    Mandelbrot(Mandelbrot),
    IteratedSinZ(IteratedSinZ),
}
