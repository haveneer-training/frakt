mod models;
mod utils;

use std::{process, io::Write};

use blue_box::{types::{protocols::FragmentResult, desc::PixelData}, models::fractal::Fractal};
use clap::Parser;
use env_logger::Env;
use log::{warn, info, debug, error};
use models::client::Client;
use utils::start_util;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specify server address
    #[arg(long, default_value = "127.0.0.1")]
    server_address: String,

    /// Specify server port
    #[arg(short, long, default_value = "8787")]
    port: String,

    /// Specify server port
    #[arg(short, long, default_value = "Groupe 7")]
    worker_name: String,

    /// Specify server port
    #[arg(short, long, default_value_t = 100)]
    max_work: u32,

    /// Use debug version
    #[arg(long)]
    debug: bool,

    /// Use GPU
    #[arg(long)]
    gpu_use: bool,
}

fn main(){
    start_util::start_message();

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    // INFO: Log levels: 
    //  error
    //  warn
    //  info
    //  debug
    //  trace

    let args = Args::parse();

    let client = Client::new(args.server_address, args.port);

    let fragment_task_result = client.ask_for_work(&args.worker_name, args.max_work);
    let (mut fragment_task, mut data) = match fragment_task_result {
        Ok((fragment, data)) => (fragment, data),
        Err(err) => {
            warn!("There was a probleme connecting to the server ... {err}");
            process::exit(1)
        }
    };
    
    // TODO: From here, you have everything you need to calculate the fractals
    
    while true{
        let pixels = PixelData{
            offset: fragment_task.id.offset, 
            count: fragment_task.resolution.nx as u32 * fragment_task.resolution.ny as u32
        };
        let fragment_result : FragmentResult = FragmentResult::new(
            fragment_task.id.clone(),
            fragment_task.resolution.clone(),
            fragment_task.range.clone(),
            pixels
        );

        // Fractal::run(&fragment_task, &mut fragment_result, &mut data);

        let fake_zn: f32 = 0.018979378;
        let fake_data: f32 = 1.0;
        data.write_all(&fake_zn.to_be_bytes());
        data.write_all(&fake_data.to_be_bytes());

        debug!("fake data = {data:?}");

        fragment_task = match client.send_work_done(fragment_result, &mut data){
            Ok(fragment) => fragment,
            Err(_) => {
                match client.ask_for_work(&args.worker_name, args.max_work) {
                    Ok((fragment, new_data)) => {
                        data = new_data;
                        fragment
                    },
                    Err(_) => {
                        error!("Can't get new work from the server");
                        process::exit(1);
                    }

                }
            }
        };
    }
}
