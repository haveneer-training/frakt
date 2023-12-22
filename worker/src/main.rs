extern crate num_complex;
extern crate image;

use num_complex::Complex;
use image::{Rgb, RgbImage};

fn julia_set(c: Complex<f64>, z_0: Complex<f64>, divergence_threshold_square: f64, max_iterations: usize) -> usize {
    let mut z = z_0;
    for i in 0..max_iterations {
        z = z * z + c;
        if z.norm_sqr() > divergence_threshold_square {
            return i;
        }
    }
    max_iterations
}

fn main() {
    // Paramètres
    let range = 2.0;
    let divergence_threshold_square = 4.0;
    let max_iterations = 1000;

    // Dimensions de l'image
    let image_width = 800;
    let image_height = 600;

    // Création de l'image avec la bibliothèque `image`
    let mut img = RgbImage::new(image_width, image_height);

    // Parcours des pixels dans l'espace physique
    for y in 0..image_height {
        for x in 0..image_width {
            // Conversion des coordonnées du pixel à un nombre complexe
            let z_0 = Complex::new(
                x as f64 / image_width as f64 * range - range / 2.0,
                y as f64 / image_height as f64 * range - range / 2.0,
            );

            // Appel de la fonction pour déterminer la divergence
            let iterations = julia_set(
                Complex::new(-0.8, 0.156), // Exemple de nombre complexe c
                z_0,
                divergence_threshold_square,
                max_iterations,
            );

            // Utilisation du nombre d'itérations pour la couleur
            let color = (iterations % 256) as u8;

            // Affectation de la couleur au pixel de l'image
            img.put_pixel(x, y, Rgb([color, color, color]));
        }
    }

    // Sauvegarde de l'image dans un fichier PNG
    img.save("fractale_julia.png").unwrap();
}
