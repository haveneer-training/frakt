pub mod networking {
    use std::error::Error;
    use std::io::Write;
    use std::mem::size_of;
    use std::net::TcpStream;

    use complex::Complex;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::fractal::{FractalDescriptor, Julia};
    use crate::image::{PixelData, PixelIntensity, Range, Resolution};

    pub const ID_SIZE: usize = 16;

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Fragment {
        FragmentRequest(Request),
        FragmentTask(Task),
        FragmentResult(Result),
    }

    impl Fragment {
        pub fn to_json(&self) -> String {
            match self {
                Fragment::FragmentRequest(fragment_request) => {
                    json!({"FragmentRequest": fragment_request}).to_string()
                }
                Fragment::FragmentTask(task) => json!({"FragmentTask": task}).to_string(),
                Fragment::FragmentResult(result) => json!({"FragmentResult": result}).to_string(),
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Request {
        worker_name: String,
        maximal_work_load: i64,
    }

    impl Request {
        pub fn new(worker_name: String, maximal_work_load: i64) -> Self {
            Request {
                worker_name,
                maximal_work_load,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Task {
        pub id: U8Data,
        pub fractal: FractalDescriptor,
        pub max_iteration: u32,
        pub resolution: Resolution,
        pub range: Range,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Result {
        pub id: U8Data,
        pub resolution: Resolution,
        pub range: Range,
        pub pixels: PixelData,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct U8Data {
        pub offset: u32,
        pub count: u32,
    }

    pub fn initate_request(
        buffer: &mut Vec<u8>,
        stream: &mut TcpStream,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let fragment_request: Fragment =
            Fragment::FragmentRequest(Request::new("worker".to_string(), 100));
        let fragment_request_str = fragment_request.to_json();
        println!("fragment_request_str: {}", fragment_request_str);
        let fragment_request_bytes = fragment_request_str.as_bytes();
        let json_message_size: u32 = fragment_request_bytes.len() as u32;
        buffer.clear();
        buffer.write_all(&json_message_size.to_be_bytes())?;
        buffer.write_all(&json_message_size.to_be_bytes())?;
        buffer.write_all(fragment_request_bytes)?;
        stream.write(&buffer)?;
        Ok(())
    }

    /// Decode a message from a buffer returning its fragment and corresponding data
    pub fn decode_message(
        buffer: &Vec<u8>,
    ) -> std::result::Result<(Fragment, &[u8]), Box<dyn Error>> {
        if buffer.len() < 8 {
            return Err("Buffer too short".into());
        }

        let total_message_size = u32::from_be_bytes(buffer[0..4].try_into()?);
        let json_message_size = u32::from_be_bytes(buffer[4..8].try_into()?);
        let json_fragment = String::from_utf8_lossy(&buffer[8..(8 + json_message_size as usize)]);
        let fragment: Fragment = serde_json::from_str(&json_fragment)?;
        let data_fragment =
            &buffer[(8 + json_message_size as usize)..(8 + total_message_size as usize)];

        Ok((fragment, data_fragment))
    }

    fn create_result_bytes(task: &Task) -> (Vec<u8>, u32) {
        let result = create_result_fragment_json(task);
        let result_bytes = result.into_bytes();
        let result_bytes_size = result_bytes.len() as u32;

        (result_bytes, result_bytes_size)
    }

    fn create_result_fragment_json(task: &Task) -> String {
        let result = create_result_fragment(task);
        let fragment_result = Fragment::FragmentResult(result);
        fragment_result.to_json()
    }

    fn create_result_fragment(task: &Task) -> Result {
        let number_of_pixels = task.resolution.nx as u32 * task.resolution.ny as u32;
        Result {
            id: task.id.clone(),
            resolution: task.resolution.clone(),
            range: task.range.clone(),
            pixels: PixelData {
                offset: ID_SIZE as u32,
                count: number_of_pixels,
            },
        }
    }

    pub fn create_result_message(
        task: Task,
        task_id: &[u8; ID_SIZE],
        buffer: &mut Vec<u8>,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let (result_fragment_json_bytes, result_fragment_bytes_size) = create_result_bytes(&task);
        let number_of_pixels = task.resolution.nx as u32 * task.resolution.ny as u32;
        let pixel_data = create_pixel_data_fom_task(&task)?;
        let total_message_size = result_fragment_bytes_size
            + (number_of_pixels * (size_of::<f64>() * 2) as u32)
            + task_id.len() as u32;
        buffer.write_all(&total_message_size.to_be_bytes())?;
        buffer.write_all(&result_fragment_bytes_size.to_be_bytes())?;
        buffer.write_all(&result_fragment_json_bytes)?;
        buffer.write_all(task_id)?;
        buffer.write_all(&pixel_data)?;
        // Not enough data is sent and this is the fix for now TODO: Remove that shid
        buffer.write_all(&pixel_data)?;

        Ok(())
    }

    fn create_pixel_data_fom_task(task: &Task) -> std::result::Result<Vec<u8>, Box<dyn Error>> {
        let number_of_pixels = task.resolution.nx as u32 * task.resolution.ny as u32;
        let mut pixel_data = Vec::with_capacity(number_of_pixels as usize * (size_of::<f64>() * 2));
        for pixel_number in 0..number_of_pixels {
            let pixel_intensity = get_pixel_intensity(&task, pixel_number);
            pixel_data.write_all(&pixel_intensity.zn.to_be_bytes())?;
            pixel_data.write_all(&pixel_intensity.count.to_be_bytes())?;
        }
        Ok(pixel_data)
    }

    pub fn get_pixel_intensity(task: &Task, pixel_number: u32) -> PixelIntensity {
        let (x, y) = get_coordinates_from_pixel_number(pixel_number, &task.resolution);
        let (real_part, imaginary_part) =
            get_complex_from_coordinates(&task.range, x, y, &task.resolution);

        match task.fractal {
            FractalDescriptor::Julia(ref julia) => {
                iterate_julia(julia, real_part, imaginary_part, task.max_iteration)
            }
            FractalDescriptor::Mandelbrot(..) => {
                iterate_mandelbrot(real_part, imaginary_part, task.max_iteration)
            }
        }
    }

    fn get_coordinates_from_pixel_number(pixel_number: u32, resolution: &Resolution) -> (u32, u32) {
        let x = pixel_number % resolution.nx as u32;
        let y = pixel_number / resolution.nx as u32;
        (x, y)
    }

    fn get_complex_from_coordinates(
        range: &Range,
        x: u32,
        y: u32,
        resolution: &Resolution,
    ) -> (f64, f64) {
        let real_part =
            range.min.x + (range.max.x - range.min.x) * (x as f64) / (resolution.nx as f64 - 1.0);
        let imaginary_part =
            range.min.y + (range.max.y - range.min.y) * (y as f64) / (resolution.ny as f64 - 1.0);
        (real_part, imaginary_part)
    }

    fn iterate_fractal(
        mut z: Complex,
        c: Complex,
        divergence_threshold_square: f64,
        max_iteration: u32,
    ) -> PixelIntensity {
        let mut iter = 0;

        while z.norm_sqr() <= divergence_threshold_square && iter < max_iteration {
            z = z * z + c;
            iter += 1;
        }

        let intensity = if iter < max_iteration {
            iter as f32 / max_iteration as f32
        } else {
            1.0
        };

        PixelIntensity {
            zn: z.norm() as f32,
            count: intensity,
        }
    }

    fn iterate_julia(
        julia: &Julia,
        real_part: f64,
        imaginary_part: f64,
        max_iteration: u32,
    ) -> PixelIntensity {
        let z = Complex {
            re: real_part,
            im: imaginary_part,
        };
        let c = julia.c;
        iterate_fractal(z, c, julia.divergence_threshold_square, max_iteration)
    }

    fn iterate_mandelbrot(
        real_part: f64,
        imaginary_part: f64,
        max_iteration: u32,
    ) -> PixelIntensity {
        let z = Complex { re: 0.0, im: 0.0 };
        let c = Complex {
            re: real_part,
            im: imaginary_part,
        };
        iterate_fractal(z, c, 4.0, max_iteration)
    }
}

pub mod fractal {
    use serde::{Deserialize, Serialize};

    use complex::Complex;

    #[derive(Debug, Serialize, Deserialize)]
    pub enum FractalDescriptor {
        Julia(Julia),
        Mandelbrot(Option<Mandelbrot>),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Julia {
        pub c: Complex,
        pub divergence_threshold_square: f64,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Mandelbrot {}
}

pub mod image {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Range {
        pub min: Point,
        pub max: Point,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Resolution {
        pub nx: u16,
        pub ny: u16,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct PixelData {
        pub offset: u32,
        pub count: u32,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct PixelIntensity {
        pub zn: f32,
        pub count: f32,
    }
}
