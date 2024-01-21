use std::env::args;
use std::net::TcpStream;
use worker::validate_worker_argument;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hola, soy el worker");

    let arguments: Vec<String> = args().collect();
    let server_address = validate_worker_argument(arguments)?;


    return Ok(());
}
