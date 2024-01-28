use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

