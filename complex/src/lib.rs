use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    /// Calculates the norm of the complex number
    pub fn norm(&self) -> f64 {
        (self.re.powi(2) + self.im.powi(2)).sqrt()
    }

    /// Calculates the square of the norm of the complex number, without taking the square root
    pub fn norm_sqr(&self) -> f64 {
        self.re.powi(2) + self.im.powi(2)
    }

    /// Calculates the sine of the complex number
    pub fn sin(&self) -> Self {
        Complex {
            re: self.re.sin() * self.im.cosh(),
            im: self.re.cos() * self.im.sinh(),
        }
    }

    /// Calculates the cosine of the complex number
    pub fn cos(&self) -> Self {
        Complex {
            re: self.re.cos() * self.im.cosh(),
            im: -self.re.sin() * self.im.sinh(),
        }
    }
}

impl Add for Complex {
    type Output = Self;

    /// Adds two complex numbers
    fn add(self, other: Self) -> Self::Output {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    /// Multiplies two complex numbers
    fn mul(self, other: Self) -> Self::Output {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}
