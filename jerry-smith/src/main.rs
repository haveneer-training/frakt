mod models;
mod utils;

use std::{process, net::TcpStream, io};

use log::{error, info, warn};
use models::server::Server;
use utils::config::Config;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

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

    let _fragment_request = Server::get_work_request(stream)?;

    Server::send_work(stream)?;

    let fragment_result_result = Server::get_work_done(stream);

    // let data: Vec<u8>;
    // let fragment_result = match fragment_result_result {
    //     Ok((fragment, data_in)) => {
    //         data = data_in;
    //         fragment
    //     },
    //     Err(err) => return Err(err),
    // };

    Ok(())
}
