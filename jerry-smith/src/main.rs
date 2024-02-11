mod models;
mod utils;

use std::{process, thread };

use log::{error, warn};
use models::{server::Server, display::DisplayFractal};
use utils::config::Config;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();

    let config = Config::read();

    let display_fractal = DisplayFractal::new();

    let server = Server::new(config.server_address, config.port);
    let listener_result = server.start_server();

    let listener = match listener_result {
        Ok(r) => r,
        Err(err) => {
            error!("Server failed to start ... \n{err}");
            process::exit(1);
        }
    };

    let thread_test = thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(mut s) => match Server::handle_client(&mut s) {
                    Ok(_) => {}
                    Err(err) => warn!("Data received from the client has a probleme! {}", err),
                },
                Err(err) => {
                    warn!("Something went wrong with a stream ! {}", err)
                }
            }
        }
    });

    display_fractal.start();
}
