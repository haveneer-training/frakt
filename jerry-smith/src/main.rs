use clap::Parser;
// use serde::Deserialize;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::{io, process};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specify address
    #[arg(long, default_value = "127.0.0.1")]
    server_address: String,

    /// Specify port
    #[arg(short, long, default_value = "8787")]
    port: String,

    /// Use println version
    #[arg(long)]
    println: bool,
}

// #[derive(Deserialize, Debug)]
// struct FragmentRequest {
//     worker_name: String,
//     maximal_work_lead: u32,
// }

fn main() -> io::Result<()> {
    let args = Args::parse();

    let listener_result = TcpListener::bind(format!("{}:{}", args.server_address, args.port));
    let listener = match listener_result {
        Ok(r) => r,
        Err(error) => {
            println!("Something went wrong! {}", error);
            process::exit(1);
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => match handle_client(&mut s) {
                Ok(_) => {}
                Err(err) => println!("Data received from the client has a probleme! {}", err),
            },
            Err(err) => {
                println!("Something went wrong with a stream ! {}", err)
            }
        }
    }

    Ok(())
}

fn handle_client(stream: &mut TcpStream) -> Result<(), io::Error> {
    println!("Incoming connection {stream:?}");

    // Get Total message size
    let mut lenbuf = [0; 4];
    stream.read_exact(&mut lenbuf)?;
    let total_message_size = u32::from_be_bytes(lenbuf);
    println!("Total message size: {}", total_message_size);

    // Get JSON message size
    let mut json_len_buf = [0; 4];
    stream.read_exact(&mut json_len_buf)?;
    let json_message_size = u32::from_be_bytes(json_len_buf);
    println!("JSON Message size: {}", json_message_size);

    if total_message_size < json_message_size {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Json message size if bigger than total message size",
        ));
    }

    // JSON message
    let mut sbuf = vec![0_u8; total_message_size as usize];
    stream.read(&mut sbuf)?;
    let s = String::from_utf8_lossy(&sbuf);
    println!("String: {}", s);

    // let fragment_request : FragmentRequest = serde_json::from_str(&s)?;
    // println!("Fragment: {:?}", fragment_request);

    // Data
    // TODO:

    Ok(())
}
