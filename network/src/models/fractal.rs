use serde::{Serialize, Deserialize}; 
use super::commmunication::Complex;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct JuliaDescriptor {
    pub c: Complex,
    pub divergence_threshold_square: f64
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum FreactalDescriptor {
    Julia(JuliaDescriptor)
}
