pub struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Complex {
        Complex { re, im }
    }
}