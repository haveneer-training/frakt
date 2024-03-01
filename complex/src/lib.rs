use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul};


#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
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

    /// Multiplies two complex numbers.
    fn mul(self, other: Self) -> Self::Output {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

#[cfg(test)]
mod test1 {
    use super::*;

    #[test]
    fn test_norm() {
        let complex = Complex { re: 3.0, im: 4.0 };
        assert_eq!(complex.norm(), 5.0);
    }

    #[test]
    fn test_norm_sqr() {
        let complex = Complex { re: 3.0, im: 4.0 };
        assert_eq!(complex.norm_sqr(), 25.0);
    }

    #[test]
    fn complex_addition() {
        // Define two complex numbers to add
        let complex1 = Complex { re: 1.5, im: 2.5 };
        let complex2 = Complex { re: 3.0, im: 4.0 };

        // Perform the addition
        let result = complex1 + complex2;

        // Define the expected result
        let expected = Complex { re: 4.5, im: 6.5 };

        // Assert that the actual result matches the expected result
        assert_eq!(
            result, expected,
            "Complex addition did not produce the expected result."
        );
    }
    #[test]
    fn test_multiplication() {
        let complex1 = Complex { re: 1.0, im: 2.0 };
        let complex2 = Complex { re: 3.0, im: 4.0 };
        let result = complex1 * complex2;
        // Based on the formula: (a+bi)(c+di) = (ac-bd) + (ad+bc)i
        assert_eq!(result, Complex { re: -5.0, im: 10.0 });
    }
}
