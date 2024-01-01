mod models;
mod utils;

use std::process;

use blue_box::{types::{protocols::FragmentResult, desc::PixelData}, models::fractal::Fractal};
use clap::Parser;
use env_logger::Env;
use log::{warn, info};
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

    let fragment_task_result = client.ask_for_work("Groupe 7".to_string(), 100);
    let (fragment_task, mut data) = match fragment_task_result {
        Ok((fragment, data)) => (fragment, data),
        Err(err) => {
            warn!("There was a probleme connecting to the server ... {err}");
            process::exit(1)
        }
    };
    
    // TODO: From here, you have everything you need to calculate the fractals
    

    let pixels = PixelData{
        offset: fragment_task.id.offset, 
        count: fragment_task.resolution.nx as u32 * fragment_task.resolution.ny as u32
    };
    let mut fragment_result : FragmentResult = FragmentResult::new(
        fragment_task.id.clone(),
        fragment_task.resolution.clone(),
        fragment_task.range.clone(),
        pixels
    );

    Fractal::run(&fragment_task, &mut fragment_result, &mut data);

    match client.send_work_done(fragment_result, data){
        Ok(_) => info!("Result sent to server"),
        Err(err) => warn!("There was a problem sendin the result to the server ... {err}"),
    };
}
