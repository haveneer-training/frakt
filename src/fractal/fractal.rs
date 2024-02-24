use std::fmt::Error;
use std::fmt::{Display, Formatter};

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

impl FractalDescriptor {
    pub fn get_datas(task: &FragmentTask) -> Vec<PixelIntensity> {
        return match &task.fractal {
            FractalDescriptor::Julia(julia) => julia.get_datas(&task),
            FractalDescriptor::Mandelbrot(mandelbrot) => mandelbrot.get_datas(&task),
            FractalDescriptor::IteratedSinZ(iterated_sin_z) => iterated_sin_z.get_datas(&task),
            FractalDescriptor::NewtonRaphsonZ3(newton_raphson_z_3) => {
                newton_raphson_z_3.get_datas(&task)
            }
            FractalDescriptor::NewtonRaphsonZ4(newton_raphson_z_4) => {
                newton_raphson_z_4.get_datas(&task)
            }
            FractalDescriptor::NovaNewtonRaphsonZ3(nova_newton_raphson_z_3) => {
                nova_newton_raphson_z_3.get_datas(&task)
            }
            FractalDescriptor::NovaNewtonRaphsonZ4(nova_newton_raphson_z_4) => {
                nova_newton_raphson_z_4.get_datas(&task)
            }
        };
    }
}

impl Display for FractalDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            FractalDescriptor::Julia(descriptor) => write!(f, "{}", descriptor),
            FractalDescriptor::Mandelbrot(descriptor) => write!(f, "{}", descriptor),
            FractalDescriptor::IteratedSinZ(descriptor) => write!(f, "{}", descriptor),
            FractalDescriptor::NewtonRaphsonZ3(descriptor) => write!(f, "{}", descriptor),
            FractalDescriptor::NewtonRaphsonZ4(descriptor) => write!(f, "{}", descriptor),
            FractalDescriptor::NovaNewtonRaphsonZ3(descriptor) => write!(f, "{}", descriptor),
            FractalDescriptor::NovaNewtonRaphsonZ4(descriptor) => write!(f, "{}", descriptor),
        }
    }
}
