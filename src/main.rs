mod messages;
mod client;

use crate::client::client_services::ClientServices;

fn main() {
    let mut client = ClientServices::new(String::from("localhost"), 8787);
    let request = messages::fragment_request::FragmentRequest::new(String::from("worker"), 10);
    let task = client.request_task(request);
    println!("{}", task.serialize());
    //TODO: calculer la fractale
}
