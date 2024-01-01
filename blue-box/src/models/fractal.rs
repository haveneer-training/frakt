use log::warn;

use crate::types::{
    fractal_type::{
        JuliaDescriptor, FractalDescriptor
    },
    protocols::{
        FragmentTask,
        FragmentResult
    }
};

// TODO: Do the same for all fractals
impl JuliaDescriptor {
    pub fn run(&self) {
        warn!("Try to calculate julia's fractal but it's not implemented");
        // TODO: Calculate the Julia fractal here
        todo!()
    }
}

#[derive(Debug)]
pub struct Fractal {}

// TODO: Delete the "_" character once the variables are used 
impl Fractal {
    pub fn run(
        fragment_task: &FragmentTask,
        _fragment_result: &mut FragmentResult,
        _data: &mut Vec<u8>,
    ) {
        match &fragment_task.fractal {
            FractalDescriptor::Julia(julia) => julia.run(),
            FractalDescriptor::Mandelbrot(_mandelbrot) => todo!(),
            FractalDescriptor::IteratedSinZ(_iter) => todo!(),
            FractalDescriptor::NewtonRaphsonZ3(_newton) => todo!(),
            FractalDescriptor::NewtonRaphsonZ4(_newton) => todo!(),
        }
    }
}
