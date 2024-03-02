use complex::Complex;

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
