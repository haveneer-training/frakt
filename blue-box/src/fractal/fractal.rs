use log::warn;

use crate::network::communication_types::{FragmentTask, FragmentResult};

use super::fractal_types::{JuliaDescriptor, FreactalDescriptor};

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

    pub fn run(fragment_task: &FragmentTask, fragment_result: &mut FragmentResult, data: &mut Vec<u8>){
        match &fragment_task.fractal {
            FreactalDescriptor::Julia(julia) => julia.run(),
            FreactalDescriptor::Mandelbrot(mandelbrot) => todo!(),
            FreactalDescriptor::IteratedSinZ(iter) => todo!(),
            FreactalDescriptor::NewtonRaphsonZ3(newton) => todo!(),
            FreactalDescriptor::NewtonRaphsonZ4(newton) => todo!(),
        }
    }
}
