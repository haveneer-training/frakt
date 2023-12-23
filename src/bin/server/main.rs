use std::io::Read;
use std::{io, process};
use std::net::{TcpListener, TcpStream};
use clap::Parser;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{

    /// Specify address
    #[arg(long, default_value = "127.0.0.1")]
    server_address: String,

    /// Specify port 
    #[arg(short, long, default_value = "8787")]
    port: String,

    /// Use debug version
    #[arg(long)]
    debug: bool,
}

#[derive(Deserialize, Debug)]
struct FragmentRequest{
    worker_name: String,
    maximal_work_lead: u32
}

fn main() -> io::Result<()>{
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
                Ok(_) => {},
                Err(err) => println!("Data received fro mthe client has a probleme! {}", err)
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

    let mut lenbuf = [0; 4];
    stream.read_exact(&mut lenbuf)?;

    let len = u32::from_be_bytes(lenbuf);
    let mut sbuf = vec![0_u8; len as usize];

    stream.read(&mut sbuf)?;

    let s = String::from_utf8_lossy(&sbuf);

    let fragment_request : FragmentRequest = serde_json::from_str(&s)?;
    println!("{:?}", fragment_request);


    Ok(())
}
