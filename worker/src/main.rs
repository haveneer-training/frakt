use shared::initate_request;
use std::env::args;
use std::io::Read;
use std::mem::size_of;
use std::net::TcpStream;
use worker::validate_worker_argument;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hola, soy el worker");

    let arguments: Vec<String> = args().collect();
    let server_address = validate_worker_argument(arguments)?;

    println!("Connecting to server on address {}...", server_address);
    let mut stream = TcpStream::connect(server_address)?;
    println!("Connected to server on address {}", server_address);
    let mut buffer: Vec<u8> = Vec::new();

    initate_request(&mut buffer, &mut stream)?;

    let size_written = stream.read_to_end(&mut buffer)?;
    println!("Size written: {}", size_written);

    let size_bytes = &buffer[0..4];
    let size_array: [u8; 4] = [size_bytes[0], size_bytes[1], size_bytes[2], size_bytes[3]];
    let total_message_size = u32::from_be_bytes(size_array);
    println!("Size of total message: {}", total_message_size);

    let size_bytes = &buffer[4..8];
    let size_array = [size_bytes[0], size_bytes[1], size_bytes[2], size_bytes[3]];
    let json_message_size = u32::from_be_bytes(size_array);
    println!("Size of json message: {}", json_message_size);
    let json = String::from_utf8_lossy(&buffer[8..(8 + json_message_size as usize)]);
    print!("Json: {}", json);

    println!("End of worker");
    return Ok(());
}