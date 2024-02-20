#![cfg(test)]

use super::colors::color_palette;


#[test]
fn test_get_correct_color() {
    let color = color_palette(0.5_f32);
    assert_eq!(color, (0, 188, 188));
}
