use serde::{Deserialize, Serialize};

use super::fractal_types::julia::Julia;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fractal {
    Julia: Julia, //laisser en majuscule, très important
}

impl Fractal {}
