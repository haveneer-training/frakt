use std::io::{Read, Write};
use std::net::TcpStream;

use crate::messages::complementary_types::pixelintensity::PixelIntensity;
use crate::messages::fragment_request::FragmentRequest;
use crate::messages::fragment_result::FragmentResult;
use crate::messages::fragment_task::FragmentTask;

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

    //TODO: virer expect et mut
    pub fn request_task(&mut self, request: FragmentRequest) -> FragmentTask {
        let serialized = request.serialize();
        let json_bytes = serialized.as_bytes();

        let msg_len: u32 = json_bytes.len() as u32;
        let a = msg_len.to_be_bytes();
        self.stream.write(&a).expect("Could not write to stream");
        self.stream.write(&a).expect("Could not write to stream");
        self.stream
            .write(json_bytes)
            .expect("Could not write to stream");

        let mut buffer = [0; 4];
        self.stream
            .read_exact(&mut buffer)
            .expect("could not read from stream");
        let total_message_size: usize = u32::from_be_bytes(buffer).try_into().expect("aezd");

        let mut buffer = [0; 4];
        self.stream
            .read_exact(&mut buffer)
            .expect("could not read from stream");
        let json_message_size: usize = u32::from_be_bytes(buffer).try_into().expect("aeaze");

        let mut json_buffer = vec![0; json_message_size];
        self.stream
            .read_exact(&mut json_buffer)
            .expect("could not read from stream");
        let json_message = String::from_utf8(json_buffer).expect("azeaze");

        let mut data_buffer = vec![0; total_message_size - json_message_size];
        self.stream
            .read_exact(&mut data_buffer)
            .expect("could not read from stream");

        let task = FragmentTask::deserialize(&json_message);
        task
    }

    pub fn send_result(&mut self, result: FragmentResult, pixels_intensity: Vec<PixelIntensity>) {
        let serialized = result.serialize();
        let json_bytes = serialized.as_bytes();
        let msg_len: u32 = json_bytes.len() as u32;

        //Total message size = message size + number of PixelIntensity entites * octet size for zn and count which are 2 f32 numbers.
        let total_msg_len: u32 = msg_len + (16 + pixels_intensity.len() * 4 * 2) as u32;
        println!(
            "{:?} {:?}",
            &pixels_intensity[0].zn.to_be_bytes(),
            &pixels_intensity[0].count.to_be_bytes()
        );
        let a = total_msg_len.to_be_bytes();
        self.stream.write(&a).expect("Could not write to stream");
        let b = msg_len.to_be_bytes();
        self.stream.write(&b).expect("Could not write to stream");
        self.stream
            .write(json_bytes)
            .expect("Could not write to stream");
        let machin = result.id.offset.to_be_bytes();
        let truc = result.id.count.to_be_bytes();

        self.stream
            .write(&machin)
            .expect("Could not write to stream");
        self.stream.write(&truc).expect("Could not write to stream");
        for pixel in pixels_intensity {
            self.stream
                .write(&pixel.zn.to_be_bytes())
                .expect("Could not write to stream");
            self.stream
                .write(&pixel.count.to_be_bytes())
                .expect("Could not write to stream");
        }

        println!("Done successfully");
    }
}
