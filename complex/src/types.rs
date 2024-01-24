pub mod calcul_types{
    use serde::{Serialize, Deserialize};
    use std::{cmp::Ordering, ops::{Add, Mul, Sub, Div}};

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

    impl PartialOrd for Complex {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            let norm_sqr_self: f64 = self.norm_sqr().reel;
            let norm_sqr_other: f64 = other.norm_sqr().reel;
    
            norm_sqr_self.partial_cmp(&norm_sqr_other)
        }
    }
    
    impl Ord for Complex {
        fn cmp(&self, other: &Self) -> Ordering {
            let norm_sqr_self: f64 = self.norm_sqr().reel;
            let norm_sqr_other: f64 = other.norm_sqr().reel;
    
            norm_sqr_self
                .partial_cmp(&norm_sqr_other)
                .unwrap_or(Ordering::Equal)
        }
    }
    
    impl PartialEq for Complex {
        fn eq(&self, other: &Self) -> bool {
            self.reel == other.reel && self.imaginaire == other.imaginaire
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


#[cfg(test)]
mod tests {
    use super::calcul_types::*;

    #[test]
    fn test_constructeur() {
        let z = Complex::new(3.0, 4.0);
        assert_eq!(z.reel, 3.0);
        assert_eq!(z.imaginaire, 4.0);
    }

    #[test]
    fn test_addition() {
        let z1 = Complex::new(3.0, 4.0);
        let z2 = Complex::new(1.0, 2.0);
        let resultat = z1 + z2;

        assert_eq!(resultat.reel, 4.0);
        assert_eq!(resultat.imaginaire, 6.0);
    }

    #[test]
    fn test_soustraction() {
        let z1 = Complex::new(3.0, 4.0);
        let z2 = Complex::new(1.0, 2.0);
        let resultat = z1 - z2;

        assert_eq!(resultat.reel, 2.0);
        assert_eq!(resultat.imaginaire, 2.0);
    }

    #[test]
    fn test_multiplication() {
        let z1 = Complex::new(3.0, 4.0);
        let z2 = Complex::new(1.0, 2.0);
        let resultat = z1 * z2;

        assert_eq!(resultat.reel, -5.0);
        assert_eq!(resultat.imaginaire, 10.0);
    }

    #[test]
    fn test_division() {
        let z1 = Complex::new(3.0, 4.0);
        let z2 = Complex::new(1.0, 2.0);
        let resultat = z1 / z2;

        assert_eq!(resultat.reel, 2.0);
        assert_eq!(resultat.imaginaire, 0.0);
    }

    #[test]
    fn test_norm_sqr() {
        let z = Complex::new(3.0, 4.0);
        let norm_sqr = z.norm_sqr();

        assert_eq!(norm_sqr.reel, -7.0);
        assert_eq!(norm_sqr.imaginaire, 24.0);
    }
}