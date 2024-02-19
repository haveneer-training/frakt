use std::env::args;
use std::io::{Read, Write};
use std::mem::size_of;
use std::net::TcpStream;

use shared::networking::{
    create_result_message, decode_message, initate_request, Fragment, ID_SIZE,
};
use worker::validate_worker_argument;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hola, soy el worker");

    let arguments: Vec<String> = args().collect();
    let server_address = validate_worker_argument(arguments)?;

    println!("\n Connecting to server on address {}...", server_address);
    let mut stream = TcpStream::connect(server_address)?;
    println!("Connected to server on address {}", server_address);
    let mut buffer: Vec<u8> = Vec::new();

    initate_request(&mut buffer, &mut stream)?;

    loop {
        buffer.clear();

        println!("\nWaiting for a Task...\n");
        let size_written = stream.read_to_end(&mut buffer)?;
        println!("Received {} bytes", size_written);
        if size_written < (size_of::<u32>() * 2) {
            println!("Error: no data received");
            return Ok(());
        }
        drop(stream);

        let (fragment, data) = decode_message(&buffer)?;
        match fragment {
            Fragment::FragmentTask(task) => {
                let mut buffer = Vec::new();
                let task_id = &data[..ID_SIZE].try_into()?;
                println!("Preparing result...");
                create_result_message(task, task_id, &mut buffer)?;
                // In the same stream, we send the result then receive a new Task
                println!("Reconnecting to server on address {}...", server_address);
                stream = TcpStream::connect(server_address)?;
                println!("Sending result...");
                stream.write(&buffer)?;
            }
            _ => {
                println!("Error: unexpected fragment");
                println!("Exiting worker...");
                break;
            }
        }
    }

    println!("\n\nEnd of worker");
    return Ok(());
}
