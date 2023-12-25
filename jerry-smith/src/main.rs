use clap::Parser;
use network::Network;
use std::net::TcpStream;
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

fn main() -> io::Result<()> {
    let args = Args::parse();

    let server = Network::new(args.server_address, args.port);
    let listener_result = server.start_server();
    let listener = match listener_result {
        Ok(r) => r,
        Err(error) => {
            println!("Serevr failed to start! {}", error);
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

    let fragment_request = Network::get_work_request(stream)?;
    println!("Asked for: {:?}", fragment_request);

    Network::send_work(stream)?;

    Ok(())
}
