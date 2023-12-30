use blue_box::network::communication_types::FragmentResult;
use blue_box::network::server::Server;
use clap::Parser;
use log::{info, warn, error};
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

    /// Use debug version
    #[arg(long)]
    debug: bool,
}

fn main() -> io::Result<()> {

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let args = Args::parse();

    let server = Server::new(args.server_address, args.port);
    let listener_result = server.start_server();
    let listener = match listener_result {
        Ok(r) => r,
        Err(error) => {
            error!("Serevr failed to start! {}", error);
            process::exit(1);
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => match handle_client(&mut s) {
                Ok(_) => {}
                Err(err) => warn!("Data received from the client has a probleme! {}", err),
            },
            Err(err) => {
                warn!("Something went wrong with a stream ! {}", err)
            }
        }
    }

    Ok(())
}

fn handle_client(stream: &mut TcpStream) -> Result<(), io::Error> {
    info!("Incoming connection {stream:?}");

    let _fragment_request = Server::get_work_request(stream)?;

    Server::send_work(stream)?;

    let fragment_result_result = Server::get_work_done(stream);

    let data: Vec<u8>;
    let fragment_result = match fragment_result_result {
        Ok((fragment, data_in)) => {
            data = data_in;
            fragment
        },
        Err(err) => return Err(err),
    };
    

    Ok(())
}
