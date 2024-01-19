//!```
//!pub struct Complex{
//!pub re: f64,
//!pub im: f64
//!}
//!```
//!
//!A complex number in f64 form.

mod complex_test;

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

    /// It is possible to obtain the sine of a complex number, which returns a new complex number
    pub fn sin(&self)-> Complex{
        Complex{
            re:(self.re.sin() * self.im.cosh()),
            im:(self.re.cos() * self.im.sinh())
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

/// You can mult complex numbers together
impl ops::Mul for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let re = self.re * other.re - self.im * other.im;
        let im = self.re * other.im + self.im * other.re;
        Self { re, im }
    }
}

