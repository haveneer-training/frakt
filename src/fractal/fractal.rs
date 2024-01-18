use serde::{Deserialize, Serialize};

use crate::messages::{
    complementary_types::pixelintensity::PixelIntensity, fragment_task::FragmentTask,
};

use super::fractal_types::{
    iterated_sin_z::IteratedSinZ, julia_descriptor::JuliaDescriptor, mandelbrot::Mandelbrot,
    newton_raphson_z_3::NewtonRaphsonZ3, newton_raphson_z_4::NewtonRaphsonZ4,
    nova_newton_raphson_z_3::NovaNewtonRaphsonZ3, nova_newton_raphson_z_4::NovaNewtonRaphsonZ4,
};

pub trait GetDatas {
    fn get_datas(&self, task: &FragmentTask) -> Vec<PixelIntensity>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FractalDescriptor {
    Julia(JuliaDescriptor),
    Mandelbrot(Mandelbrot),
    IteratedSinZ(IteratedSinZ),
    NewtonRaphsonZ3(NewtonRaphsonZ3),
    NewtonRaphsonZ4(NewtonRaphsonZ4),
    NovaNewtonRaphsonZ3(NovaNewtonRaphsonZ3),
    NovaNewtonRaphsonZ4(NovaNewtonRaphsonZ4),
}
