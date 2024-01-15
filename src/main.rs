extern crate image;
mod client;
mod messages;

mod client_calcul {
    pub mod libs;
}

use crate::{
    client::client_services::ClientServices,
    messages::{
        complementary_types::{complex::Complex, pixelintensity::PixelIntensity},
        fractal::fractal_types::julia::Julia,
        fragment_result::FragmentResult,
    },
};
use client_calcul::libs::fractal_lib;
use serde::{Deserialize, Serialize};

fn main() {
    let mut client = ClientServices::new(String::from("localhost"), 8787);
    let request = messages::fragment_request::FragmentRequest::new(String::from("worker"), 10);
    let (task, id) = client.request_task(request);
    println!("{}", task.serialize());
    //TODO: calculer la fractale

    let _result = FragmentResult::create(&task);

    println!("{}", _result.serialize());

    let pixel_intensity_vec = Julia::get_datas(&task);

    fractal_lib::create_image(&task, &pixel_intensity_vec);

    println!("{:?}", &pixel_intensity_vec[0]);

    // println!(
    //     "nombre de PixelIntensity calculated: {}",
    //     pixel_intensity_vec.len()
    // );

    //make loop here so when a FragmentResult is sent, the worker takes another task
    client = ClientServices::new(String::from("localhost"), 8787);
    client.send_result(_result, pixel_intensity_vec, id);

    while true {
        let (task, id) = client.readTaskResponse();
        let _result = FragmentResult::create(&task);

        println!("{}", _result.serialize());

        let pixel_intensity_vec = Julia::get_datas(&task);

        fractal_lib::create_image(&task, &pixel_intensity_vec);

        println!("{:?}", &pixel_intensity_vec[0]);

        // println!(
        //     "nombre de PixelIntensity calculated: {}",
        //     pixel_intensity_vec.len()
        // );

        //make loop here so when a FragmentResult is sent, the worker takes another task
        client = ClientServices::new(String::from("localhost"), 8787);
        client.send_result(_result, pixel_intensity_vec, id);
    }
}
