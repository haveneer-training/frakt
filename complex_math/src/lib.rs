mod test_complex;

use std::ops::{Add, Div, Mul, Sub};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Copy, Debug,PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Add<f64> for Complex {
    type Output = Self;

    fn add(self, num: f64) -> Self {
        Complex {
            re: self.re + num,
            im: self.im,
        }
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Sub<f64> for Complex {
    type Output = Self;

    fn sub(self, num: f64) -> Self {
        Complex {
            re: self.re - num,
            im: self.im,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Mul<f64> for Complex {
    type Output = Self;

    fn mul(self, num: f64) -> Self {
        Complex {
            re: self.re * num,
            im: self.im * num,
        }
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Complex {
            re: (self.re * rhs.re + self.im * rhs.im) / (rhs.re * rhs.re + rhs.im * rhs.im),
            im: (self.im * rhs.re - self.re * rhs.im) / (rhs.re * rhs.re + rhs.im * rhs.im),
        }
    }
}

impl Div<f64> for Complex {
    type Output = Self;

    fn div(self, num: f64) -> Self {
        Complex {
            re: self.re / num,
            im: self.im / num,
        }
    }
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Complex {
        Complex { re, im }
    }
    pub fn arg_sq(self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn norm(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    pub fn abs(self) -> Self {
        Complex {
            re: self.re.abs(),
            im: self.im.abs(),
        }
    }

    pub fn sin(&self) -> Complex {
        Complex {
            re: (self.re.sin() * self.im.cosh()),
            im: (self.re.cos() * self.im.sinh()),
        }
    }

    pub fn arg(self) -> f64 {
        self.im.atan2(self.re)
    }

    pub fn pow(self, num: u32) -> Self {
        let mut result = self;
        for _ in 1..num {
            result = result * self;
        }
        result
    }
}
