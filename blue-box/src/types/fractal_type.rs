use std::cmp::max;
use std::io::Write;
use std::ops::{Div, Sub};

use cmplx_nbr::Complex;
use image::{ImageBuffer, Rgb};
use log::error;
use serde::{Deserialize, Serialize};
use crate::utils::colors::color_palette;

use super::{protocols::FragmentTask, desc::Range};

pub trait CalcFractal {


    fn determine_pixel_intensity(&self, x: f64, y: f64, max_iteration: &u32)-> (f32, f32);

    fn make_image(&self, fragment_task: &FragmentTask, data: &mut Vec<u8>) {
        let Range {min, max} = fragment_task.range;
        let max_iteration = (fragment_task.max_iteration) as u32;
        let width = fragment_task.resolution.nx as u32;
        let height = fragment_task.resolution.ny as u32;

        let mut image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

        for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {

            let mapped_x = min.x + (x as f64 / width as f64) * (max.x - min.x);
            let mapped_y = min.y + (y as f64 / height as f64) * (max.y - min.y);

            let (zn , count) = self.determine_pixel_intensity(mapped_x, mapped_y, &max_iteration);

            if let Err(_) = data.write_all(&zn.to_be_bytes()){
                error!("Can't add to data")
            }

            if let Err(_) = data.write_all(&count.to_be_bytes()){
                error!("Can't add to data")
            }

            // let (red, green, blue) = Rgb{0: color_palette(count) as [u8; 3]};
            // pixel[0] = red;
            // pixel[1] = green;
            // pixel[2] = blue;
        }


        image_buffer.save("./last_image_client.png");
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
        ((z.norm() as f32), (i as f32 / *max_iteration as f32))
    }
}

impl CalcFractal for IteratedSinZ{

    fn determine_pixel_intensity(&self, x: f64, y: f64, max_iteration: &u32) -> (f32, f32) {
        let mut z = Complex::new(x, y);

        let mut i = 0;
        while i < *max_iteration && z.norm() < 50.0 {
            z = self.c * z.sin();
            i += 1;
        }

        ((z.norm() as f32), (i as f32 / *max_iteration as f32))
    }

}
impl NewtonRaphsonZ3{
    pub fn new()->Self {
        Self {}
    }
    fn fz(&self, z:Complex)-> Complex{
        z*z*z-Complex::new(1.0,0.0)
    }
    fn dfz(&self, z:Complex)-> Complex{
        Complex::new(3.0,0.0) * z * z
    }
}


impl CalcFractal for NewtonRaphsonZ3 {

    fn determine_pixel_intensity(&self, x: f64, y: f64, max_iteration: &u32) -> (f32, f32) {
        let mut z = Complex::new(x, y);
        let mut zn_next;
        let delta = 1e-6;
        let mut i = 0;

        loop {
            zn_next = z - (self.fz(z) / self.dfz(z));
            if (zn_next - z).norm_square() < delta || i >= *max_iteration {
                break;
            }
            z = zn_next;
            i += 1;
        }

        let zn = z.angle();
        let count = if i < *max_iteration {
            Complex::convergence_value(z.norm_square() as f32, delta, i, *max_iteration)
        } else {
            1.0
        };

        return (zn as f32, i as f32 * count / *max_iteration as f32);
    }
}







