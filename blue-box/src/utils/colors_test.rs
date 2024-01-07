#![cfg(test)]

use super::colors::get_color;

#[test]
fn test_get_correct_color() {
    get_color(10_f32);
}
