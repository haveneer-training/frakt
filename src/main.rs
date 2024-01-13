extern crate image;
mod client;
mod messages;

mod client_calcul {
    pub mod libs;
}

use crate::client::client_services::ClientServices;
use client_calcul::libs::fractal_lib;
use serde::{Deserialize, Serialize};

fn main() {
    let image_width = 1920;
    let image_height = 1080;

    let mut image_buffer = image::ImageBuffer::new(image_width, image_height);

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let u = x as f64 / image_height as f64;
        let v = y as f64 / image_height as f64;
        let t = fractal_lib::mandelbrot(2.5 * (u - 0.5) - 1.4, 2.5 * (v - 0.5));
        *pixel = image::Rgb(fractal_lib::color((2.0 * t + 0.5) % 1.0));
        let t = fractal_lib::mandelbrot(2.5 * (u - 0.5) - 1.4, 2.5 * (v - 0.5));
        *pixel = image::Rgb(fractal_lib::color((2.0 * t + 0.5) % 1.0));
    }

    image_buffer.save("Mandelbrot.png").unwrap();

    // from Jules
    let mut client = ClientServices::new(String::from("localhost"), 8787);
    let request = messages::fragment_request::FragmentRequest::new(String::from("worker"), 10);
    let task = client.request_task(request);
    println!("{}", task.serialize());
    //TODO: calculer la fractale
}
