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
    #[arg(long, default_value = "localhost")]
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

        Fractal::run(&fragment_task, &fragment_result, &mut data);
        
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
                match client.ask_for_work(&args.worker_name, args.max_work) {
                    Ok((fragment, new_data)) => {
                        data = new_data;
                        fragment
                    },
                    Err(_) => {
                        error!("The server must be switched off");
                        process::exit(1);
                    }
                }
            }
        };
    }
}
