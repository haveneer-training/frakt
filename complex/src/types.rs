pub mod CalculType{
    use serde::{Serialize, Deserialize};

    // Structure de nombres complexes
    #[derive(Debug, Serialize, PartialEq, Deserialize)]
    pub struct Complex {
        pub reel: f64,
        pub imaginaire: f64,
    }
}