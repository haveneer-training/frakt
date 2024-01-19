#![cfg(test)]

use cmplx_nbr::Complex;

use crate::types::fractal_type::{
    JuliaDescriptor,
    CalcFractal,
    Mandelbrot, IteratedSinZ
};

#[test]
fn test_julia() {
    let julia :JuliaDescriptor = JuliaDescriptor {
        c: Complex::new(-0.9, 0.27015),
        divergence_threshold_square: 4.0 
    };

    let calc_pixel = julia.determine_pixel_intensity(0_f64, 0_f64, &150);
    assert_eq!(calc_pixel, (4.4671035, 0.3));
}

#[test]
fn test_julia_second_params() {
    let julia :JuliaDescriptor = JuliaDescriptor {
        c: Complex::new(0.285, 0.013),
        divergence_threshold_square: 4.0 
    };

    let calc_pixel = julia.determine_pixel_intensity(0_f64, 0_f64, &150);
    assert_eq!(calc_pixel, (0.16961907, 1.0));
}

#[test]
fn test_mandelbrot() {
    let mandelbrot: Mandelbrot = Mandelbrot{};

    let calc_pixel = mandelbrot.determine_pixel_intensity(0_f64, 0_f64, &64);
    assert_eq!(calc_pixel, (0.0, 1.0));
}

#[test]
fn test_iterated_sin_z() {
    let iter: IteratedSinZ = IteratedSinZ{
        c: Complex::new(1.0, 0.3),
    };

    let calc_pixel = iter.determine_pixel_intensity(0_f64, 0_f64, &64);
    assert_eq!(calc_pixel, (0.0, 1.0));
}
