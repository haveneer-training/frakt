mod models;
mod utils;

use std::{process, net::TcpStream, io };

use blue_box::types::protocols::Fragment;
use log::{error, info, warn, debug};
use models::server::Server;
use utils::config::Config;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();

    let config = Config::read();

    let server = Server::new(config.server_address, config.port);
    let listener_result = server.start_server();

    let listener = match listener_result {
        Ok(r) => r,
        Err(err) => {
            error!("Server failed to start ... \n{err}");
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
}

fn handle_client(stream: &mut TcpStream) -> Result<(), io::Error> {
    info!("Incoming connection {stream:?}");

    match Server::read_messge_from_client(stream){
        Ok((Fragment::FragmentRequest(fragment), _)) => {
            debug!("Work request received");
            Server::send_work(stream, &fragment);
        },
        Ok((Fragment::FragmentResult(fragment), data)) => {
            debug!("Work done");
            Server::get_work_done(stream, &fragment, data);
            process::exit(0);
        },
        Ok((Fragment::FragmentTask(_), _)) => {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "The worker send a task",
            ));
        },
        Err(err) => {
            return Err(err);
        }
    };

    Ok(())
}
