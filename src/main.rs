extern crate image;
mod client;
mod fractal;
mod messages;

mod client_calcul {
    pub mod libs;
}

use std::env;

use crate::client_calcul::libs::fractal_lib;
use crate::fractal::fractal::FractalDescriptor;
use crate::{client::client_services::ClientServices, messages::fragment_result::FragmentResult};
fn main() {
    let mut host = String::from("localhost");
    let mut port = 8787;

    let args: Vec<String> = env::args().collect();

    // Récupérer le nom de l'exécutable
    let elements: Vec<&str> = args[0].split('/').collect();
    let exec = elements.last().unwrap();

    match args.len() {
        1 => {
            // Utiliser les valeurs par défaut de host et port
        }
        2 => {
            // Changer pour "--help" quand possible de lancer en exécutable
            if args[1] == "help" {
                println!("Usage : ./{} <name> <port>", exec);
                // Terminer le programme
                std::process::exit(0);
            }
        }
        3 => {
            // Récupérer les arguments valides
            host = args[1].clone();
            port = args[2].clone().parse().unwrap();
        }
        _ => {
            // Nombres d'arguments incorrects
            eprintln!("Erreur : Nombre incorrect d'arguments !");
            eprintln!("Usage : ./{} <name> <port>", exec);
            // Terminer le programme avec un code d'erreur
            std::process::exit(1);
        }
    }

    let mut client = ClientServices::new(&host, &port);
    let request = messages::fragment_request::FragmentRequest::new(String::from("worker"), 10);
    let (task, id) = client.request_task(request);
    println!("{}", task.serialize());

    let _result = FragmentResult::create(&task);
    println!("{}", _result.serialize());

    let datas = FractalDescriptor::get_datas(&task);
    //fractal_lib::create_image(&task, &datas);

    //make loop here so when a FragmentResult is sent, the worker takes another task
    client = ClientServices::new(&host, &port);
    client.send_result(_result, datas, id);

    loop {
        let (task, id) = client.read_task_response();
        println!("{}", task.serialize());

        let _result = FragmentResult::create(&task);
        println!("{}", _result.serialize());

        let datas = FractalDescriptor::get_datas(&task);
        // fractal_lib::create_image(&task, &datas);

        client = ClientServices::new(&host, &port);
        client.send_result(_result, datas, id);
    }
}
