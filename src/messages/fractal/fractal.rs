use serde::{Deserialize, Serialize};

use super::fractal_types::{julia::Julia, mandelbrot::Mandelbrot};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fractal {
    pub Julia: Julia, //laisser en majuscule, très important
                      // pub Mandelbrot: Mandelbrot,
}

impl Fractal {}
