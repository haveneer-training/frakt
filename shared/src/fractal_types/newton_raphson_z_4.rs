use std::fmt::{Display, Error, Formatter};

use serde::{Deserialize, Serialize};

use complex_math::Complex;

use crate::{complementary_types::pixelintensity::PixelIntensity, fractal_implementation::{fractal::GetDatas, fractal_calcul::newton_raphson_z_4}, messages::fragment_task::FragmentTask};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewtonRaphsonZ4 {}

impl GetDatas for NewtonRaphsonZ4 {
    fn get_datas(&self, task: &FragmentTask) -> Vec<PixelIntensity> {
        let x_start = task.range.min.x;
        let x_end = task.range.max.x;
        let y_start = task.range.min.y;
        let y_end = task.range.max.y;

        let x_step = ((&x_start - &x_end) / task.resolution.nx as f64).abs();
        let y_step = ((&y_start - &y_end) / task.resolution.ny as f64).abs();

        let mut datas: Vec<PixelIntensity> = Vec::new();

        let max_iteration = task.max_iteration;
        let mut x = x_start;
        let mut y = y_start;

        while y < y_end {
            while x < x_end {
                let pixel_complexe = Complex::new(x, y);
                let fractal_result = newton_raphson_z_4(pixel_complexe, max_iteration);
                datas.push(PixelIntensity::new(fractal_result.0, fractal_result.1));
                x += x_step;
            }
            x = x_start;
            y += y_step;
        }

        return datas;
    }
}

impl Display for NewtonRaphsonZ4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "NewtonRaphsonZ4")
    }
}
