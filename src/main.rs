use std::{io, process};
use std::net::TcpStream;
use std::io::Write;
use serde::Serialize;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{

    /// Specify server address
    #[arg(long, default_value = "127.0.0.1")]
    server_address: String,

    /// Specify server port 
    #[arg(short, long, default_value = "8787")]
    port: String,

    /// Use debug version
    #[arg(long)]
    debug: bool,

    /// Use GPU
    #[arg(long)]
    gpu_use: bool
}

#[derive(Serialize)]
struct FramentRequest{
    worker_name: String,
    maximal_work_lead: u32
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let stream_result = TcpStream::connect(format!("{}:{}", args.server_address, args.port));
    let mut stream = match stream_result {
        Ok(rep) => rep,
        Err(err) => {
            // TODO: Try serveral times 
            println!("The server unreachable! {}", err);
            process::exit(1)
        }
    };

    let work_request = FramentRequest {worker_name: String::from("Communication test"), maximal_work_lead: 13};
    
    let work_serialized = serde_json::to_string(&work_request).unwrap();
    println!("work_request = {}", work_serialized);

    let message_size = work_serialized.len() as u32;
    stream.write(&message_size.to_be_bytes())?;
    stream.write(work_serialized.as_bytes())?;

    Ok(())
}
