#[cfg(test)]
use crate::Complex;


#[test]
fn test_add() {
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let c = a + b;
    assert_eq!(c.re, 4.0);
    assert_eq!(c.im, 6.0);
}

#[test]
fn test_add_f64() {
    let a = Complex::new(1.0, 2.0);
    let b = 3.0;
    let c = a + b;
    assert_eq!(c.re, 4.0);
    assert_eq!(c.im, 2.0);
}

#[test]
fn test_sub() {
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let c = a - b;
    assert_eq!(c.re, -2.0);
    assert_eq!(c.im, -2.0);
}

#[test]
fn test_sub_f64() {
    let a = Complex::new(1.0, 2.0);
    let b = 3.0;
    let c = a - b;
    assert_eq!(c.re, -2.0);
    assert_eq!(c.im, 2.0);
}

#[test]
fn test_mul() {
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let c = a * b;
    assert_eq!(c.re, -5.0);
    assert_eq!(c.im, 10.0);
}

#[test]
fn test_mul_f64() {
    let a = Complex::new(1.0, 2.0);
    let b = 3.0;
    let c = a * b;
    assert_eq!(c.re, 3.0);
    assert_eq!(c.im, 6.0);
}

#[test]
fn test_div() {
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let c = a / b;
    assert_eq!(c.re, 0.44);
    assert_eq!(c.im, 0.08);
}

#[test]
fn test_div_f64() {
    let a = Complex::new(1.0, 2.0);
    let b = 3.0;
    let c = a / b;
    assert_eq!(c.re, 1.0 / 3.0);
    assert_eq!(c.im, 2.0 / 3.0);
}

#[test]
fn test_arg() {
    let a = Complex::new(3.0, 4.0);
    let c = a.arg();
    assert_eq!(c, 0.9272952180016122);
}

#[test]
fn test_arg_sq() {
    let a = Complex::new(3.0, 4.0);
    let c = a.arg_sq();
    assert_eq!(c, 25.0);
}

#[test]
fn test_norm() {
    let a = Complex::new(3.0, 4.0);
    let c = a.norm();
    assert_eq!(c, 5.0);
}

#[test]
fn test_sin() {
    let a = Complex::new(3.0, 4.0);
    let c = a.sin();
    assert_eq!(c.re, 3.853738037919377);
    assert_eq!(c.im, -27.016813258003932);
}

#[test]
fn test_pow() {
    let a = Complex::new(3.0, 4.0);
    let c = a.pow(2);
    assert_eq!(c.re, -7.0);
    assert_eq!(c.im, 24.0);
}


