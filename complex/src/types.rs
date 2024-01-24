pub mod calcul_types{
    use serde::{Serialize, Deserialize};
    use std::ops::{Add, Mul, Sub, Div};

    // Structure de nombres complexes
    #[derive(Debug, Serialize, PartialEq, Deserialize, Clone, Copy)]
    pub struct Complex {
        pub reel: f64,
        pub imaginaire: f64,
    }

    impl Add<Complex> for Complex {
        type Output = Complex;

        fn add(self, rhs : Self) -> Self::Output{   //self = complex type
            Complex {
                reel: self.reel + rhs.reel,
                imaginaire: self.imaginaire + rhs.imaginaire
            }
        }
    }

    impl Sub<Complex> for Complex {
        type Output = Complex;
    
        fn sub(self, rhs: Self) -> Self::Output {
            Complex {
                reel: self.reel - rhs.reel,
                imaginaire: self.imaginaire - rhs.imaginaire,
            }
        }
    }

    impl Mul<Complex> for Complex {
        type Output = Complex;
    
        fn mul(self, rhs: Self) -> Self::Output {
            Complex {
                reel: self.reel * rhs.reel - self.imaginaire * rhs.imaginaire,
                imaginaire: self.reel * rhs.imaginaire + self.imaginaire * rhs.reel,
            }
        }
    }

    impl Div<Complex> for Complex {
        type Output = Complex;
    
        fn div(self, rhs: Self) -> Self::Output {
            let denominator: f64 = rhs.reel * rhs.reel + rhs.imaginaire * rhs.imaginaire;
    
            Complex {
                reel: (self.reel * rhs.reel + self.imaginaire * rhs.imaginaire) / denominator,
                imaginaire: (self.imaginaire * rhs.reel - self.reel * rhs.imaginaire) / denominator,
            }
        }
    }

    impl Complex {

        pub fn new(reel: f64, imaginaire: f64) -> Complex {
            Complex { reel, imaginaire }
        }
        // Fonction pour calculer la norme au carré
        pub fn norm_sqr(&self) -> Complex {
            Complex {
                reel: self.reel * self.reel - self.imaginaire * self.imaginaire,
                imaginaire: 2.0 * self.reel * self.imaginaire,
            }
        }
    }
}