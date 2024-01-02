use crate::types::{
    fractal_type::{
        FractalDescriptor, CalcFractal
    },
    protocols::{
        FragmentTask,
        FragmentResult
    }, desc::PixelData
};

#[derive(Debug)]
pub struct Fractal {}

// TODO: Delete the "_" character once the variables are used 
impl Fractal {
    pub fn run(
        fragment_task: &FragmentTask,
        data: &mut Vec<u8>
    ) -> FragmentResult {
        let pixels = PixelData{
            offset: fragment_task.id.count, 
            count: fragment_task.resolution.nx as u32 * fragment_task.resolution.ny as u32 
        };

        let fragment_result : FragmentResult = FragmentResult::new(
            fragment_task.id.clone(),
            fragment_task.resolution.clone(),
            fragment_task.range.clone(),
            pixels
        );

        match &fragment_task.fractal {
            FractalDescriptor::Julia(julia) => julia.make_image(&fragment_task, data),
            FractalDescriptor::Mandelbrot(mandelbrot) => mandelbrot.make_image(&fragment_task, data),
            FractalDescriptor::IteratedSinZ(_iter) => todo!(),
            FractalDescriptor::NewtonRaphsonZ3(_newton) => todo!(),
            FractalDescriptor::NewtonRaphsonZ4(_newton) => todo!(),
        };

        fragment_result
    }

}
