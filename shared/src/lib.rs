use serde::{Deserialize, Serialize};
use std::{error::Error, io::Write, net::TcpStream};

#[derive(Debug, Serialize, Deserialize)]
pub struct FragmentRequest {
    FragmentRequest: FragmentBody,
}

impl FragmentRequest {
    pub fn new(worker_name: String, maximal_work_load: i64) -> Self {
        FragmentRequest {
            FragmentRequest: FragmentBody {
                worker_name,
                maximal_work_load,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]

struct FragmentBody {
    worker_name: String,
    maximal_work_load: i64,
}

pub fn initate_request(buffer: &mut Vec<u8>, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let fragment_request = FragmentRequest::new("fractal_painter".to_string(), 2000);
    let fragment_request_str = serde_json::to_string(&fragment_request)?;
    let fragment_request_bytes = fragment_request_str.as_bytes();
    let json_message_size: u32 = fragment_request_bytes.len() as u32;
    buffer.write_all(&json_message_size.to_be_bytes())?;
    buffer.write_all(&json_message_size.to_be_bytes())?;
    buffer.write_all(fragment_request_bytes)?;
    stream.write_all(&buffer)?;
    Ok(())
}
