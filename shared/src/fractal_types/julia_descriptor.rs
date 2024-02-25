use std::fmt::{Display, Error, Formatter};

use serde::{Deserialize, Serialize};

use complex_math::Complex;

use crate::{complementary_types::pixelintensity::PixelIntensity, fractal_implementation::{fractal::GetDatas, fractal_calcul::julia}, messages::fragment_task::FragmentTask};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JuliaDescriptor {
    pub divergence_threshold_square: f64,
    pub c: Complex,
}

impl GetDatas for JuliaDescriptor {
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
                let fractal_result = julia(
                    pixel_complexe,
                    self.c,
                    self.divergence_threshold_square,
                    max_iteration,
                );
                datas.push(PixelIntensity::new(fractal_result.0, fractal_result.1));
                x += x_step;
            }
            x = x_start;
            y += y_step;
        }

        return datas;
    }
}

impl Display for JuliaDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Julia")
    }
}
