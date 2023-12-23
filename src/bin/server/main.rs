use std::{io::{self, Read}, net::TcpListener, process};

use clap::Parser;

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
        let mut stream = stream?;
        println!("Incoming connection {stream:?}");

        let mut lenbuf = [0;4];
        stream.read_exact(&mut lenbuf)?;
        let len = u32::from_be_bytes(lenbuf);
        let mut sbuf = vec![0_u8; len as usize];
        stream.read(&mut sbuf)?;
        // println!("{sbuf:?}");
        let s = String::from_utf8_lossy(&sbuf);
        println!("{s}")
    }
        
    Ok(())
}
