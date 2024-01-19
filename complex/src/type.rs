pub mod CalculType{
    // Structure de nombres complexes
    #[derive(Debug, Serialize, PartialEq, Deserialize)]
    pub struct Complex {
        pub reel: f64,
        pub imaginaire: f64,
    }
}