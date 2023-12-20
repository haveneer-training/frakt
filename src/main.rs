

//use data_stuctures::point::Point;
//use data_stuctures::complex::Complex;
//use data_stuctures::range::Range;
//use data_stuctures::resolution::Resolution;
//use data_stuctures::u8_data::U8Data;
//use data_stuctures::pixel_data::PixelData;
//use data_stuctures::pixel_intensity::PixelIntensity;



fn main() {
    // Créez une instance de Point
    let point = Point { x: 1.0, y: 2.0 };
    println!("{:?}", point);

    // Créez une instance de Complex
    let complex = Complex { re: 0.5, im: -0.3 };
    println!("{:?}", complex);

    // Créez une instance de Range
    let range = Range {
        min: Point { x: 0.0, y: 0.0 },
        max: Point { x: 1.0, y: 1.0 },
    };
    println!("{:?}", range);

    // Créez une instance de Resolution
    let resolution = Resolution { nx: 800, ny: 600 };
    println!("{:?}", resolution);

    // Créez une instance de U8Data
    let u8_data = U8Data { offset: 0, count: 100 };
    println!("{:?}", u8_data);

    // Créez une instance de PixelData
    let pixel_data = PixelData { offset: 0, count: 19200 };
    println!("{:?}", pixel_data);

    // Créez une instance de PixelIntensity
    let pixel_intensity = PixelIntensity { zn: 0.5, count: 0.8 };
    println!("{:?}", pixel_intensity);
}