mod models;
mod utils;

use std::{process, thread, sync::mpsc::{self, Sender, Receiver} };

use blue_box::types::desc::PixelIntensity;
use log::{error, warn};
use models::{server::Server, display::DisplayFractal};
use utils::config::Config;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let config = Config::read();

    let (tx, rx): (Sender<Vec<PixelIntensity>>, Receiver<Vec<PixelIntensity>>) = mpsc::channel();

    let display_fractal = DisplayFractal::new(config.width, config.height);

    let server = Server::new(config.server_address, config.port, config.width, config.height);
    let listener_result = server.start_server();

    let listener = match listener_result {
        Ok(r) => r,
        Err(err) => {
            error!("Server failed to start ... \n{err}");
            process::exit(1);
        }
    };

    let _communication_thread = thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(mut s) => match Server::handle_client(&mut s, &tx) {
                    Ok(_) => {}
                    Err(err) => warn!("Data received from the client has a probleme! {}", err),
                },
                Err(err) => {
                    warn!("Something went wrong with a stream ! {}", err)
                }
            }
        }
    });

    display_fractal.start(rx);
}
