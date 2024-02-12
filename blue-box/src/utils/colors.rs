pub fn color_palette(t: f32) -> (u8, u8, u8) {
    const A: (f32, f32, f32) = (0.5, 0.5, 0.5);
    const B: (f32, f32, f32) = (0.5, 0.5, 0.5);
    const C: (f32, f32, f32) = (1.0, 1.0, 1.0);
    const D: (f32, f32, f32) = (0.0, 0.33, 0.67);

    let cos = |c, d, t| A.0 + B.0 * (((2.0 * std::f32::consts::PI * (t + d)) as f32).cos() * c);

    (
        (cos(C.0, D.0, t) * 255.0) as u8,
        (cos(C.1, D.1, t) * 255.0) as u8,
        (cos(C.2, D.2, t) * 255.0) as u8,
    )
}
