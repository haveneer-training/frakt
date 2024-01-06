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

    while true{
        let fragment_result = Fractal::run(&fragment_task, &mut data);
        
        // for &i in &data{
        //     debug!("{:02x}", i);
        // }
        // println!("Size {}", data.len());

        fragment_task = match client.send_work_done(fragment_result, &mut data){
            Ok(fragment) => {
                // panic!();
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
    }
}
