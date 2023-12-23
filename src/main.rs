use std::io;
use std::net::TcpStream;
use std::io::Write;

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


fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut stream = TcpStream::connect(format!("{}:{}", args.server_address, args.port))?;

    let message = "Hello from the client";
    let message_size = message.len() as u32;
    stream.write(&message_size.to_be_bytes())?;
    stream.write(message.as_bytes())?;

    Ok(())
}
