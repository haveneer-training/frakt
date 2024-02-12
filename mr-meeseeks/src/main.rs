//! This is a course project for 4AL2
//! This code is a worker, its aim is to calculate fractals for a server. 
//! (This code can't work alone)
//! 
//! ## Introduction
//!
//! This Rust module initializes a client-server interaction for distributed computing tasks, 
//! specifically focused on fractal generation. 
//! The module is structured into two separate sub-modules: `models` and `utils`. 
//! The main functionality revolves around creating a `Client` instance, which connects to a server specified in the `Config`. 
//! It retrieves work tasks (fragment tasks) and processes them using the `Fractal` model.
//!
//! The logging is handled by `env_logger`, and the log level can be adjusted (error, warn, info, debug, trace). 
//! The client continuously requests work from the server, processes each fragment task by running the `Fractal::run` method,
//! and then sends the completed work back to the server.
//!
//! In case of errors while connecting to the server or sending the completed work,
//! the program logs appropriate warnings or errors and exits or continues based on the nature of the error.
//! The loop continues as long as there are fragment tasks to process, making it suitable for continuous, distributed computation tasks.
//!
//!## Fractals
//!
//! With this worker, you can calculate various fractals such as :
//!
//! - [x] Julia 
//! - [x] Mandelbrot
//! - [x] IteratedSinZ
//! - [ ] NewtonRaphsonZ3
//! - [ ] NewtonRaphsonZ4

mod models;
mod utils;

use std::process;

use blue_box::models::fractal::Fractal;
use env_logger::Env;
use log::{warn, error};
use models::client::Client;
use utils::{start_util, config::Config};

fn main(){
    start_util::start_message();

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    // INFO: Log levels: 
    //  error
    //  warn
    //  info
    //  debug
    //  trace

    let config = Config::read();

    let client = Client::new(config.server_address, config.port);

    let fragment_task_result = client.ask_for_work(&config.worker_name, config.max_work);
    let (mut fragment_task, mut data) = match fragment_task_result {
        Ok((fragment, data)) => (fragment, data),
        Err(err) => {
            warn!("There was a probleme connecting to the server ... {err}");
            process::exit(1)
        }
    };

    // while true{
        let fragment_result = Fractal::run(&fragment_task, &mut data);
        
        fragment_task = match client.send_work_done(fragment_result, &mut data){
            Ok(fragment) => {
                fragment
            },
            Err(_) => {
                match client.ask_for_work(&config.worker_name, config.max_work) {
                    Ok((fragment, new_data)) => {
                        data = new_data;
                        fragment
                    },
                    Err(_) => {
                        error!("The server must be switched off");
                        return ();
                    }
                }
            }
        };
    // 
}
