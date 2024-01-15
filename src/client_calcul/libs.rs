pub mod fractal_lib {
    use crate::messages::{
        complementary_types::{complex::Complex, pixelintensity::PixelIntensity},
        fragment_task::FragmentTask,
    };

    // pub fn mandelbrot(x: f64, y: f64) -> f64 {
    //     let mut z = Complex { re: 0.0, im: 0.0 };
    //     let c = Complex { re: x, im: y };
    //     let max = 256;
    //     let mut i = 0;
    //     while i < max && z.arg_sq() < 32.0 {
    //         z = z * z + c;
    //         i += 1;
    //     }
    //     return (i as f64 - z.arg_sq().log2().log2()) / (max as f64);
    // }

    // pub fn julia(x: f64, y: f64) -> f64 {
    //     let mut z = Complex { re: x, im: y };
    //     let c = Complex { re: 0.38, im: 0.28 };
    //     let max = 256;
    //     let mut i = 0;
    //     while i < max && z.arg_sq() < 32.0 {
    //         z = z * z + c;
    //         i += 1;
    //     }
    //     return (i as f64 - z.arg_sq().log2().log2()) / (max as f64);
    // }

    pub fn julia(z: Complex, c: Complex, max_divergence: f64, max_iter: u16) -> (f32, f32) {
        let mut zn = z;
        let mut count = 0;

        while count < max_iter && zn.arg_sq() < max_divergence {
            zn = zn * zn + c;
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

        while zn.arg_sq() < 4 as f64 {
            zn = zn * zn + c;
            count += 1;
        }
        (
            zn.arg_sq() as f32 / 4 as f32,
            count as f32 / max_iter as f32,
        )
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
        for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
            let t = pixel_intensity_vec[count].zn as f64;
            *pixel = image::Rgb(color((2.0 * t + 0.5) % 1.0));
            count += 1;
        }

        image_buffer.save("Julia.png").unwrap();
    }
}
