use clap::Parser;
use network::{models::commmunication::FragmentTask, Network};
use std::{io, process};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
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
    gpu_use: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let network = Network::new(args.server_address, args.port);

    let stream_result = network.get_server_connection();

    let mut stream = match stream_result {
        Ok(rep) => rep,
        Err(err) => {
            // TODO: Try serveral times
            println!("The server unreachable! {}", err);
            process::exit(1)
        }
    };

    let fractal_request_request = network.ask_for_work(&mut stream, "Groupe 7".to_string(), 100);
    let fragment_request: FragmentTask = match fractal_request_request {
        Ok(fragment) => fragment,
        Err(err) => {
            println!("Something went wrong: {}", err);
            process::exit(1)
        }
    };

    println!("Your work: {:?}", fragment_request);

    Ok(())
}
