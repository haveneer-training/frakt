use std::ops;

use crate::network::communication_types::Complex;

impl Complex {
    pub fn new(re: f64, im: f64) -> Complex {
        Complex { re, im }
    }

    pub fn norm(&self) -> f64 {
        (self.re.powi(2) + self.im.powi(2)).sqrt()
    }
}

impl ops::Add for Complex {
    type Output = Self;
    fn add(self, ot: Self) -> Self {
        Self {
            re: self.re + ot.re,
            im: self.im + ot.im,
        }
    }
}

impl ops::Mul for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let re = self.re * other.re - self.im * other.im;
        let im = self.re * other.im + self.im * other.re;
        Self { re, im }
    }
}
