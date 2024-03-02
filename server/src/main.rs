use std::env::args;
use std::net::TcpListener;

use server::{receive_request, send_task, validate_server_argument};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hola, soy el server.");

    let arguments: Vec<String> = args().collect();
    let server_address = validate_server_argument(arguments)?;

    let listener = TcpListener::bind(server_address)?;

    for stream in listener.incoming() {
        let mut stream = stream?;
        println!("Incoming connection {stream:?}");

        let fragment = receive_request(&mut stream)?;

        println!("Received fragment: {fragment:?}");

        send_task(&mut stream)?;

        break;
    }

    println!("\n\nEnd of server");
    Ok(())
}
