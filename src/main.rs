extern crate image;
mod client;
mod messages;

use crate::client::client_services::ClientServices;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl std::ops::Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Complex {
    fn arg_sq(self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    fn abs(self) -> Self {
        Complex {
            re: self.re.abs(),
            im: self.im.abs(),
        }
    }
}

fn mandelbrot(x: f64, y: f64) -> f64 {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex { re: x, im: y };
    let max = 256;
    let mut i = 0;
    while i < max && z.arg_sq() < 32.0 {
        z = z * z + c;
        i += 1;
    }
    return (i as f64 - z.arg_sq().log2().log2()) / (max as f64);
}

fn julia(x: f64, y: f64) -> f64 {
    let mut z = Complex { re: x, im: y };
    let c = Complex { re: 0.38, im: 0.28 };
    let max = 256;
    let mut i = 0;
    while i < max && z.arg_sq() < 32.0 {
        z = z * z + c;
        i += 1;
    }
    return (i as f64 - z.arg_sq().log2().log2()) / (max as f64);
}

fn color(t: f64) -> [u8; 3] {
    let a = (0.5, 0.5, 0.5);
    let b = (0.5, 0.5, 0.5);
    let c = (1.0, 1.0, 1.0);
    let d = (0.0, 0.10, 0.20);
    let r = b.0 * (6.28318 * (c.0 * t + d.0)).cos() + a.0;
    let g = b.1 * (6.28318 * (c.1 * t + d.1)).cos() + a.1;
    let b = b.2 * (6.28318 * (c.2 * t + d.2)).cos() + a.2;
    [(255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8]
}

fn main() {
    let image_width = 1920;
    let image_height = 1080;

    let mut image_buffer = image::ImageBuffer::new(image_width, image_height);

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let u = x as f64 / image_height as f64;
        let v = y as f64 / image_height as f64;
        let t = mandelbrot(2.5 * (u - 0.5) - 1.4, 2.5 * (v - 0.5));
        *pixel = image::Rgb(color((2.0 * t + 0.5) % 1.0));
    }

    image_buffer.save("Mandelbrot.png").unwrap();

    // from Jules

    let mut client = ClientServices::new(String::from("localhost"), 8787);
    let request = messages::fragmentrequest::FragmentRequest::new(String::from("w1"), 10);
    client.request_task(request);
}
