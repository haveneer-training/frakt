pub mod pixel_management_types {
    use serde::{Serialize, Deserialize};

    
    // Structure de coordonnées
    #[derive(Debug, Serialize , PartialEq, Deserialize)]
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }

    // Structure de plage (range)
    #[derive(Debug, Serialize, PartialEq, Deserialize)]
    pub struct Range {
        pub min: Point,
        pub max: Point,
    }

    // Structure de résolution
    #[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
    pub struct Resolution {
        pub nx: u16,
        pub ny: u16,
    }

    // Structure représentant l'intensité d'un pixel
    #[derive(Debug, Serialize, PartialEq, Deserialize)]
    pub struct PixelIntensity {
        pub zn: f32,
        pub count: f32,
    }

    // Structure de données pour les pixels
    #[derive(Debug, Serialize, PartialEq, Deserialize)]
    pub struct PixelData {
        pub offset: u32,
        pub count: u32,
    }
}

pub mod messages_types {
    use serde::{Serialize, Deserialize};

    use crate::pixel_management_types::{Resolution, Range, PixelData};
  
     

    // Structure de données pour identifier une tâche
    #[derive(Debug, Serialize, PartialEq, Deserialize)]
    pub struct U8Data {
        pub offset: u32,
        pub count: u32,
    }

    // Structure de demande de fragment
    #[derive(Debug, Serialize, PartialEq, Deserialize)]
    pub struct FragmentRequest {
        pub worker_name: String,
        pub maximal_work_load: u32,
    }

    // Structure de tâche de fragment
    #[derive(Debug, Serialize, PartialEq, Deserialize)]
    pub struct FragmentTask {
        pub id: U8Data,
        // pub fractal: Julia, // TO SET
        pub max_iteration: u16,
        pub resolution: Resolution,
        pub range: Range,
    }

    // Structure de résultat de fragment
    #[derive(Debug, Serialize, PartialEq, Deserialize)]
    pub struct FragmentResult {
        pub id: U8Data,
        pub resolution: Resolution,
        pub range: Range,
        pub pixels: PixelData,
    }

}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::types::messages::{Complex, PixelIntensity};


//     #[test]
//     fn test_serialize_deserialize_julia_fractal() {
//         // Vos tests de sérialisation et désérialisation pour la fractale Julia
//         let julia_fractal = fractals::Julia {
//             c: Complex { reel: 0.0, imaginaire: 0.1 },
//             divergence_threshold_square: 0.0,
//         };

//         let serialized = serde_json::to_string(&julia_fractal).unwrap();
//         let deserialized: fractals::Julia = serde_json::from_str(&serialized).unwrap();

//         assert_eq!(julia_fractal, deserialized);
//     }

//     #[test]
//     fn test_pixel_intensity_serialization() {
//         // Teste la sérialisation et la désérialisation pour PixelIntensity
//         let pixel_intensity : PixelIntensity = PixelIntensity {
//             zn: 1.5,
//             count: 1.0,
//         };

//         let serialized = serde_json::to_string(&pixel_intensity).unwrap();
//         let deserialized: PixelIntensity = serde_json::from_str(&serialized).unwrap();

//         assert_eq!(pixel_intensity, deserialized);
//     }

//     #[test]
//     fn test_fragment_request_serialization() {
//         // Teste la sérialisation et la désérialisation pour FragmentRequest
//         let fragment_request = messages::FragmentRequest {
//             worker_name: String::from("test_worker"),
//             maximal_work_load: 1000,
//         };

//         let serialized = serde_json::to_string(&fragment_request).unwrap();
//         let deserialized: messages::FragmentRequest = serde_json::from_str(&serialized).unwrap();

//         assert_eq!(fragment_request, deserialized);
//     }

//     // Ajoutez d'autres tests selon les besoins
// }