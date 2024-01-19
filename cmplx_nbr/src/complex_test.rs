#![cfg(test)]

use crate::Complex;

#[test]
fn test_add() {
    let _5_4i = Complex::new(5_f64, 4_f64);
    let _7_2i = Complex::new(7_f64, 2_f64);

    let _12_6i = Complex::new(12_f64, 6_f64);

    assert_eq!(_5_4i + _7_2i, _12_6i);
}

#[test]
fn test_mul() {
    let _3_2i = Complex::new(3_f64, 2_f64);
    let _1_7i = Complex::new(1_f64, 7_f64);

    let _1_1i = Complex::new(1_f64, 1_f64);
    let _1_1i_2 = Complex::new(1_f64, 1_f64);

    let __11_23i = Complex::new(-11_f64, 23_f64);
    let _0_2i = Complex::new(0_f64, 2_f64);

    assert_eq!(_3_2i * _1_7i, __11_23i);
    assert_eq!(_1_1i * _1_1i_2, _0_2i);
}

#[test]
fn cal_norm() {
    let _3_2i = Complex::new(3_f64, 2_f64);
    let _1_7i = Complex::new(1_f64, 7_f64);
    let __11_23i = Complex::new(-11_f64, 23_f64);

    assert_eq!(13.0, _3_2i.norm());
    assert_eq!(50.0, _1_7i.norm());
    assert_eq!(650.0, __11_23i.norm());

}
