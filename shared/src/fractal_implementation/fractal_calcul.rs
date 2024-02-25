  use std::f64::consts::PI;

use complex_math::Complex;

use crate::{complementary_types::pixelintensity::PixelIntensity, messages::fragment_task::FragmentTask};

  pub fn julia(z: Complex, c: Complex, max_divergence: f64, max_iter: u16) -> (f32, f32) {
      let mut zn = z;
      let mut count = 0;

      while count < max_iter && zn.arg_sq() < max_divergence {
          zn = zn.pow(2) + c;
          count += 1;
      }
      (
          zn.arg_sq() as f32 / max_divergence as f32,
          count as f32 / max_iter as f32,
      )
  }

  pub fn mandelbrot(pixel_complexe: Complex, max_iter: u16) -> (f32, f32) {
      let c = pixel_complexe;
      let mut zn = Complex::new(0 as f64, 0 as f64);
      let mut count = 0;

      while zn.arg_sq() < 4 as f64 && count < max_iter {
          zn = zn.pow(2) + c;
          count += 1;
      }
      (
          zn.arg_sq() as f32 / 4 as f32,
          count as f32 / max_iter as f32,
      )
  }

  pub fn iterated_sin_z(z: Complex, c: Complex, max_iter: u16) -> (f32, f32) {
      let mut zn = z;
      let mut count = 0;

      while zn.arg_sq() < 50 as f64 && count < max_iter {
          zn = zn.sin() * c;
          count += 1;
      }
      (
          zn.arg_sq() as f32 / 4 as f32,
          count as f32 / max_iter as f32,
      )
  }

  pub fn newton_raphson_z_3(z: Complex, max_iter: u16) -> (f32, f32) {
      let mut zn = z;
      let mut previous_zn = Complex::new(0.0, 0.0);
      let mut count = 0;

      while (zn - previous_zn).arg_sq() > 10.0_f64.powi(-6) && count < max_iter {
          previous_zn = zn;
          zn = zn - (zn.pow(3) - 1.0) / (zn.pow(2) * 3.0);
          count += 1;
      }

      (
          0.5 + zn.arg() as f32 / (2.0 * PI) as f32,
          count as f32 / max_iter as f32,
      )
  }

  pub fn newton_raphson_z_4(z: Complex, max_iter: u16) -> (f32, f32) {
      let mut zn = z;
      let mut previous_zn = Complex::new(0.0, 0.0);
      let mut count = 0;

      while (zn - previous_zn).arg_sq() > 10.0_f64.powi(-6) && count < max_iter {
          previous_zn = zn;
          zn = zn - (zn.pow(4) - 1.0) / (zn.pow(3) * 4.0);
          count += 1;
      }

      (
          0.5 + zn.arg() as f32 / (2.0 * PI) as f32,
          count as f32 / max_iter as f32,
      )
  }

  pub fn nova_newton_raphson_z_3(pixel_complexe: Complex, max_iter: u16) -> (f32, f32) {
      let mut zn = Complex::new(1.0, 0.0);
      let c = pixel_complexe;
      let mut previous_zn = Complex::new(0.0, 0.0);
      let mut count = 0;

      while (zn - previous_zn).arg_sq() > 10.0_f64.powi(-6) && count < max_iter {
          previous_zn = zn;
          zn = c + zn - (zn.pow(3) - 1.0) / (zn.pow(2) * 3.0);
          count += 1;
      }

      (0 as f32, count as f32 / max_iter as f32)
  }

  pub fn nova_newton_raphson_z_4(pixel_complexe: Complex, max_iter: u16) -> (f32, f32) {
      let mut zn = Complex::new(1.0, 0.0);
      let c = pixel_complexe;
      let mut previous_zn = Complex::new(0.0, 0.0);
      let mut count = 0;

      while (zn - previous_zn).arg_sq() > 10.0_f64.powi(-6) && count < max_iter {
          previous_zn = zn;
          zn = c + zn - (zn.pow(4) - 1.0) / (zn.pow(3) * 4.0);
          count += 1;
      }

      (0 as f32, count as f32 / max_iter as f32)
  }

  pub fn color(t: f64) -> [u8; 3] {
      let a = (0.5, 0.5, 0.5);
      let b = (0.5, 0.5, 0.5);
      let c = (1.0, 1.0, 1.0);
      let d = (0.0, 0.10, 0.20);
      let r = b.0 * (6.28318 * (c.0 * t + d.0)).cos() + a.0;
      let g = b.1 * (6.28318 * (c.1 * t + d.1)).cos() + a.1;
      let b = b.2 * (6.28318 * (c.2 * t + d.2)).cos() + a.2;
      [(255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8]
  }

  pub fn create_image(task: &FragmentTask, pixel_intensity_vec: &Vec<PixelIntensity>) {
      let image_width = task.resolution.nx as u32;
      let image_height = task.resolution.ny as u32;

      let mut image_buffer = image::ImageBuffer::new(image_width, image_height);

      let mut count = 0;
      for (_x, _y, pixel) in image_buffer.enumerate_pixels_mut() {
          let t = pixel_intensity_vec[count].zn as f64;

          *pixel = image::Rgb(color((2.0 * t + 0.5) % 1.0));
          count += 1;
      }

      image_buffer.save(format!("{}.png", task.fractal)).unwrap();
  }
