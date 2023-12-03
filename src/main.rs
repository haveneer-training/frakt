mod messages;
mod client;

use crate::client::client_services::ClientServices;

fn main() {
    let mut client = ClientServices::new(String::from("localhost"), 8787);
    let request = messages::fragmentrequest::FragmentRequest::new(String::from("w1"), 10);
    client.request_task(request);
}
