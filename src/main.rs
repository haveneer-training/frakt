extern crate image;
mod client;
mod messages;

mod client_calcul {
    pub mod libs;
}

use crate::{
    client::client_services::ClientServices,
    messages::complementary_types::{complex::Complex, pixelintensity::PixelIntensity},
};
use client_calcul::libs::fractal_lib;
use serde::{Deserialize, Serialize};

fn main() {
    // let image_width = 1920;
    // let image_height = 1080;

    // let mut image_buffer = image::ImageBuffer::new(image_width, image_height);

    // for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
    //     let u = x as f64 / image_height as f64;
    //     let v = y as f64 / image_height as f64;
    //     let t = fractal_lib::mandelbrot(2.5 * (u - 0.5) - 1.4, 2.5 * (v - 0.5));
    //     *pixel = image::Rgb(fractal_lib::color((2.0 * t + 0.5) % 1.0));
    //     let t = fractal_lib::mandelbrot(2.5 * (u - 0.5) - 1.4, 2.5 * (v - 0.5));
    //     *pixel = image::Rgb(fractal_lib::color((2.0 * t + 0.5) % 1.0));
    // }

    // image_buffer.save("Mandelbrot.png").unwrap();

    // from Jules
    let mut client = ClientServices::new(String::from("localhost"), 8787);
    let request = messages::fragment_request::FragmentRequest::new(String::from("worker"), 10);
    let task = client.request_task(request);
    println!("{}", task.serialize());
    //TODO: calculer la fractale

    let pixelData = messages::complementary_types::pixeldata::PixelData::new(
        task.id.offset + task.id.count,
        task.resolution.nx as u32 * task.resolution.ny as u32,
    );
    let _result = messages::fragment_result::FragmentResult::new(
        task.id,
        task.resolution,
        task.range,
        pixelData,
    );

    println!("{}", _result.serialize());

    let x_start = task.range.min.x;
    let x_end = task.range.max.x;
    let y_start = task.range.min.y;
    let y_end = task.range.max.y;

    let x_step = ((&x_start - &x_end) / task.resolution.nx as f64).abs();
    let y_step = ((&y_start - &y_end) / task.resolution.ny as f64).abs();

    let mut pixel_intensity_vec: Vec<PixelIntensity> = Vec::new();

    let julia_complexe = task.fractal.Julia.c;
    let max_divergence = task.fractal.Julia.divergence_threshold_square;
    let max_iteration = 1000;
    let mut x = x_start;
    let mut y = y_start;

    let mut previous_y = x_start;
    let mut previous_x = y_start;
    println!("Start!");
    println!("y: {}, y_end: {}, y_step: {}", y, y_end, y_step);
    println!("x: {}, x_end: {}, x_step: {}", x, x_end, x_step);

    while y < y_end {
        // if (y - previous_y).abs() >= 0.1 {
        //     println!("CHANGING LINE ! \ny: {}", y);
        //     previous_y = y;
        // }
        while x < x_end {
            // if (x - previous_x).abs() >= 0.1 {
            //     println!("x: {}", x);
            //     previous_x = x;
            // }
            let pixel_complexe = Complex::new(x, y);
            let fractal_result = fractal_lib::julia(
                pixel_complexe,
                julia_complexe,
                max_divergence,
                max_iteration,
            );
            pixel_intensity_vec.push(PixelIntensity::new(fractal_result.0, fractal_result.1));
            x += x_step;
        }
        x = x_start;
        y += y_step;
    }

    let image_width = task.resolution.nx as u32;
    let image_height = task.resolution.ny as u32;

    let mut image_buffer = image::ImageBuffer::new(image_width, image_height);

    let mut count = 0;
    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let t = pixel_intensity_vec[count].zn as f64;
        *pixel = image::Rgb(fractal_lib::color((2.0 * t + 0.5) % 1.0));
        count += 1;
    }

    image_buffer.save("Julia.png").unwrap();

    // println!("{:?}", pixel_intensity_vec);

    println!(
        "nombre de PixelIntensity calculated: {}",
        pixel_intensity_vec.len()
    );
    client = ClientServices::new(String::from("localhost"), 8787);
    client.send_result(_result, pixel_intensity_vec);
}
