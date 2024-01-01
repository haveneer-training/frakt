use std::io::Write;

use cmplx_nbr::Complex;
use log::{warn, debug, error};

use crate::types::{
    fractal_type::{
        JuliaDescriptor, FractalDescriptor
    },
    protocols::{
        FragmentTask,
        FragmentResult
    }, desc::{Resolution, Point}
};

// TODO: Do the same for all fractals
impl JuliaDescriptor {
    pub fn run(&self, fragment_result: &FragmentResult ,max_iteration: &u32, data: &mut Vec<u8>) {
        self.make_image(
            &fragment_result.resolution,
            &fragment_result.range.max,
            &fragment_result.range.min,
            max_iteration, 
            data
        )
    }

    fn make_image(&self, resolution: &Resolution, max: &Point, min: &Point, max_iteration: &u32, data: &mut Vec<u8>) {
        for offset in 0..(resolution.nx as u32 * resolution.ny as u32) {
            let x = offset % resolution.nx as u32;
            let y = offset / resolution.nx as u32;

            let mapped_x = min.x + (x as f64 / resolution.nx as f64) * (max.x - min.x);
            let mapped_y = min.y + (y as f64 / resolution.ny as f64) * (max.y - min.y);

            let (zn , count) = self.make_pixel(mapped_x, mapped_y, max_iteration);

            if let Err(_) = data.write_all(&zn.to_be_bytes()){
                error!("Can't add to data")
            }
            if let Err(_) = data.write_all(&count.to_be_bytes()){
                error!("Can't add to data")
            }
        }
    }

    fn make_pixel(&self, x: f64, y: f64, max_iteration: &u32)-> (f32, f32){

        let mut z = Complex::new(x, y);

        let mut i = 0;
        while i < *max_iteration && z.norm() < self.divergence_threshold_square {
            z = z * z + self.c;
            i += 1;
        }
        (z.norm() as f32, (i as f32 / *max_iteration as f32))
    }
}

#[derive(Debug)]
pub struct Fractal {}

// TODO: Delete the "_" character once the variables are used 
impl Fractal {
    pub fn run(
        fragment_task: &FragmentTask,
        fragment_result: &mut FragmentResult,
        data: &mut Vec<u8>,
    ) {
        match &fragment_task.fractal {
            FractalDescriptor::Julia(julia) => julia.run(fragment_result, &(fragment_task.max_iteration as u32), data),
            FractalDescriptor::Mandelbrot(_mandelbrot) => todo!(),
            FractalDescriptor::IteratedSinZ(_iter) => todo!(),
            FractalDescriptor::NewtonRaphsonZ3(_newton) => todo!(),
            FractalDescriptor::NewtonRaphsonZ4(_newton) => todo!(),
        }
    }
}
