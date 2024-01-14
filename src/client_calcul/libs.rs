pub mod fractal_lib {
    use crate::messages::complementary_types::complex::Complex;

    pub fn mandelbrot(x: f64, y: f64) -> f64 {
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
        for i in 0..max_iter {
            count = i;
            zn = zn * zn + c;

            if zn.arg_sq() > max_divergence {
                break;
            }
        }
        (
            (zn.arg_sq() / max_divergence) as f32,
            (count / max_iter) as f32,
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
}
