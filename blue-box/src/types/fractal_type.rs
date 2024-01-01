use cmplx_nbr::Complex;
use serde::{Deserialize, Serialize};

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
