//!```
//!pub struct Complex{
//!pub re: f64,
//!pub im: f64
//!}
//!```
//!
//!A complex number in f64 form.

mod complex_test;

use std::f64::consts::PI;
use std::ops;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Complex {
        Complex { re, im }
    }

    /// It is possible to calculate the norm of a complex number
    pub fn norm(&self) -> f64 {
        self.re.powi(2) + self.im.powi(2)
    }

    pub fn angle(&self) -> f64 {
        (self.im.atan2(self.re) / (2.0 * PI)).rem_euclid(1.0)

    }

    pub fn norm_square(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    /// It is possible to obtain the sine of a complex number, which returns a new complex number
    pub fn sin(&self)-> Complex{
        Complex{
            re:(self.re.sin() * self.im.cosh()),
            im:(self.re.cos() * self.im.sinh())
        }
    }

    pub fn convergence_value(pzn: f32, threshold: f64, count: u32, nmax: u32) -> f32 {
        let accuracy = f32::log10(threshold as f32);
        if count < nmax {
            0.5 - 0.5 * f32::cos(0.1 * (count as f32 - (f32::log10(pzn) / accuracy)))
        } else {
            1.
        }
    }

}

/// You can add compelx numbers together
impl ops::Add for Complex {
    type Output = Self;
    fn add(self, ot: Self) -> Self {
        Self {
            re: self.re + ot.re,
            im: self.im + ot.im,
        }
    }
}

impl ops::Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            re : self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }

}

/// You can mult complex numbers together
impl ops::Mul for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let re = self.re * other.re - self.im * other.im;
        let im = self.re * other.im + self.im * other.re;
        Self { re, im }
    }
}

impl std::ops::Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let denom = rhs.re * rhs.re + rhs.im * rhs.im;
        Self {
            re: (self.re * rhs.re + self.im * rhs.im) / denom,
            im: (self.im * rhs.re - self.re * rhs.im) / denom,
        }
    }
}





