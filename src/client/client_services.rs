use std::io::{Write, Read};
use std::net::TcpStream;

use crate::messages::fragmentrequest::FragmentRequest;
use crate::messages::fragmenttask::FragmentTask;

pub struct ClientServices {
    stream: TcpStream,
}

impl ClientServices {
    pub fn connect_to(host: &str, port: u16) -> TcpStream {
        let server_addr: String = format!("{}:{}", host, port);
        TcpStream::connect(server_addr).expect("Could not connect to server")
    }

    pub fn new(host: String, port: u16) -> ClientServices {
        let stream = ClientServices::connect_to(&host, port);
        ClientServices { stream }
    }

    pub fn request_task(&mut self, request: FragmentRequest) {
        let serialized = serde_json::to_string(&request).expect("Could not serialize request");
        println!("{:?}", serialized);
        let json_bytes = serialized.as_bytes();
        // le serveur dit que le message est trop long, mais ras dans wireshark?
        self.stream.write(json_bytes).expect("Could not write to stream");
        println!("{:?}", json_bytes);
        self.stream.flush().expect("Could not flush stream");
        // let buf = &mut [0; 1024];
        // self.stream.read(buf).expect("Could not read from stream");
        // println!("{:?}", buf);
    }
}