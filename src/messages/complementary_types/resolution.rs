pub struct Resolution {
    nx: u16,
    ny: u16,
}

impl Resolution {
    pub fn new(nx: u16, ny: u16) -> Resolution {
        Resolution { nx, ny }
    }
}