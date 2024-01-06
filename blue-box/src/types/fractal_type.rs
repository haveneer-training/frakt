use std::io::Write;

use cmplx_nbr::Complex;
use image::{ImageBuffer, Rgb};
use log::error;
use serde::{Deserialize, Serialize};

use super::{protocols::FragmentTask, desc::Range};

pub trait CalcFractal {
    fn determine_pixel_intensity(&self, x: f64, y: f64, max_iteration: &u32)-> (f32, f32);

    fn make_image(&self, fragment_task: &FragmentTask, data: &mut Vec<u8>) {
        let Range {min, max} = fragment_task.range;
        let max_iteration = (fragment_task.max_iteration) as u32;
        let width = fragment_task.resolution.nx as u32;
        let height = fragment_task.resolution.ny as u32;

        let mut image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

        for (x, y, _pixel) in image_buffer.enumerate_pixels_mut() {

            // debug!("x->{}; y->{}", x, y);

            let mapped_x = min.x + (x as f64 / width as f64) * (max.x - min.x);
            let mapped_y = min.y + (y as f64 / height as f64) * (max.y - min.y);

            let (zn , count) = self.determine_pixel_intensity(mapped_x, mapped_y, &max_iteration);

            if let Err(_) = data.write_all(&zn.to_be_bytes()){
                error!("Can't add to data")
            }

            if let Err(_) = data.write_all(&count.to_be_bytes()){
                error!("Can't add to data")
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct JuliaDescriptor {
    pub c: Complex,
    pub divergence_threshold_square: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Mandelbrot {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct IteratedSinZ {
    pub c: Complex,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct NewtonRaphsonZ3 {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct NewtonRaphsonZ4 {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum FractalDescriptor {
    Julia(JuliaDescriptor),
    Mandelbrot(Mandelbrot),
    IteratedSinZ(IteratedSinZ),
    NewtonRaphsonZ3(NewtonRaphsonZ3),
    NewtonRaphsonZ4(NewtonRaphsonZ4),
}

impl CalcFractal for JuliaDescriptor{
    fn determine_pixel_intensity(&self, x: f64, y: f64, max_iteration: &u32)-> (f32, f32){

        let mut z = Complex::new(x, y);

        let mut i = 0;
        while i < *max_iteration && z.norm() < self.divergence_threshold_square {
            z = z * z + self.c;
            i += 1;
        }
        (z.norm() as f32, (i as f32 / *max_iteration as f32))
    }
}

impl CalcFractal for Mandelbrot{
    fn determine_pixel_intensity(&self, x: f64, y: f64, max_iteration: &u32)-> (f32, f32){

        let c = Complex::new(x, y);
        let mut z = Complex::new(0_f64, 0_f64);

        let mut i = 0;
        while i < *max_iteration && z.norm() < 4.0{
            z = z * z + c;
            i += 1;
        }
        ((z.norm() as f32 / 4.0), (i as f32 / *max_iteration as f32))
    }
}

impl CalcFractal for IteratedSinZ{

    fn determine_pixel_intensity(&self, x: f64, y: f64, max_iteration: &u32) -> (f32, f32) {
        let c = Complex::new(x, y);
        let mut z = Complex::new(0_f64, 0_f64);

        let mut i = 0;
        while i < *max_iteration && z.norm() < 4.0 {
            z = z * z + c;
            i += 1;
        }

        ((z.norm() as f32), (i as f32 / *max_iteration as f32))
    }

}
