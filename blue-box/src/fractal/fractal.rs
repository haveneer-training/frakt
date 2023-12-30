use log::warn;

use crate::network::communication_types::{FragmentResult, FragmentTask};

use super::fractal_types::{FractalDescriptor, JuliaDescriptor};

impl JuliaDescriptor {
    pub fn run(&self) {
        warn!("Try to calculate julia's fractal but it's not implemented");
        // TODO: Faire ici le calcule pour la fractale de Julia
        todo!()
    }
}

#[derive(Debug)]
pub struct Fractal {}

impl Fractal {
    pub fn run(
        fragment_task: &FragmentTask,
        fragment_result: &mut FragmentResult,
        data: &mut Vec<u8>,
    ) {
        match &fragment_task.fractal {
            FractalDescriptor::Julia(julia) => julia.run(),
            FractalDescriptor::Mandelbrot(mandelbrot) => todo!(),
            FractalDescriptor::IteratedSinZ(iter) => todo!(),
            FractalDescriptor::NewtonRaphsonZ3(newton) => todo!(),
            FractalDescriptor::NewtonRaphsonZ4(newton) => todo!(),
        }
    }
}
