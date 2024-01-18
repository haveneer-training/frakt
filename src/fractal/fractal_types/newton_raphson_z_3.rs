use serde::{Deserialize, Serialize};

use crate::{
    client_calcul::libs::fractal_lib,
    fractal::fractal::GetDatas,
    messages::{
        complementary_types::{complex::Complex, pixelintensity::PixelIntensity},
        fragment_task::FragmentTask,
    },
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewtonRaphsonZ3 {}

impl GetDatas for NewtonRaphsonZ3 {
    fn get_datas(&self, task: &FragmentTask) -> Vec<PixelIntensity> {
        let x_start = task.range.min.x;
        let x_end = task.range.max.x;
        let y_start = task.range.min.y;
        let y_end = task.range.max.y;

        let x_step = ((&x_start - &x_end) / task.resolution.nx as f64).abs();
        let y_step = ((&y_start - &y_end) / task.resolution.ny as f64).abs();

        let mut pixel_intensity_vec: Vec<PixelIntensity> = Vec::new();

        let max_iteration = task.max_iteration;
        let mut x = x_start;
        let mut y = y_start;

        while y < y_end {
            while x < x_end {
                let pixel_complexe = Complex::new(x, y);
                let fractal_result = fractal_lib::newton_raphson_z_3(pixel_complexe, max_iteration);
                pixel_intensity_vec.push(PixelIntensity::new(fractal_result.0, fractal_result.1));
                x += x_step;
            }
            x = x_start;
            y += y_step;
        }

        return pixel_intensity_vec;
    }
}
