extern crate num_complex;
extern crate image;

use num_complex::Complex;
use image::{Rgb, RgbImage};
// use shared

pub mod fractals {


    // use shared::messages::Complex;
    // use serde::{Serialize, Deserialize};
    // use crate::

    use serde::{Serialize, Deserialize};
    use complex::CalculType::Complex;

    #[derive(Debug, Serialize, PartialEq, Deserialize)]
    pub struct Julia {
        pub c: Complex,
        pub divergence_threshold_square: f64,
    }
}

fn julia_set(c: Complex<f64>, z_0: Complex<f64>, divergence_threshold_square: f64, max_iterations: usize) -> usize {
    let mut z = z_0;
    for i in 0..max_iterations {
        z = z * z + c;
        if z.norm_sqr() > divergence_threshold_square {
            return i;
        }
    }
    max_iterations
}

fn main() {

    let range = 2.0;
    let divergence_threshold_square = 4.0;
    let max_iterations = 1000;

    let image_width = 800;
    let image_height = 600;

    let mut img = RgbImage::new(image_width, image_height);

    for y in 0..image_height {
        for x in 0..image_width {
            let z_0 = Complex::new(
                x as f64 / image_width as f64 * range - range / 2.0,
                y as f64 / image_height as f64 * range - range / 2.0,
            );

            let iterations = julia_set(
                Complex::new(-0.8, 0.156),
                z_0,
                divergence_threshold_square,
                max_iterations,
            );
            let color = (iterations % 256) as u8;

            img.put_pixel(x, y, Rgb([color, color, color]));
        }
    }
    img.save("fractale_julia.png").unwrap();
}
