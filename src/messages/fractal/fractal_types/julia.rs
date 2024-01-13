use serde::{Deserialize, Serialize};

use super::super::super::complementary_types::complex::Complex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Julia {
    pub divergence_threshold_square: f64,
    pub c: Complex,
}
