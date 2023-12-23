use serde::{Serialize, Deserialize};

use super::fractal_types::julia::Julia;

#[derive(Serialize, Deserialize)]
pub struct Fractal {
    Julia: Julia, //laisser en majuscule, très important
}

impl Fractal {
    
}