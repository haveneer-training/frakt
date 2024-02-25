use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;

use image::EncodableLayout;
use shared_lib::complementary_types::pixelintensity::PixelIntensity;
use shared_lib::fractal_implementation::fractal::FractalDescriptor;
use shared_lib::messages::fragment_request::FragmentRequest;
use shared_lib::messages::fragment_result::FragmentResult;
use shared_lib::messages::fragment_task::FragmentTask;

pub struct ClientServices {
    stream: TcpStream,
}

impl ClientServices {
    pub fn connect_to(host: &str, port: &u16) -> TcpStream {
        let server_addr: String = format!("{}:{}", host, port);
        TcpStream::connect(server_addr).expect("Could not connect to server")
    }

    pub fn new(host: &String, port: &u16) -> ClientServices {
        let stream = ClientServices::connect_to(&host, &port);
        ClientServices { stream }
    }

    //TODO: virer expect et mut
    pub fn request_task(&mut self, request: FragmentRequest) -> (FragmentTask, Vec<u8>) {
        let serialized = request.serialize();
        let json_bytes = serialized.as_bytes();

        let msg_len: u32 = json_bytes.len() as u32;
        let a = msg_len.to_be_bytes();
        self.stream.write(&a).expect("Could not write to stream");
        self.stream.write(&a).expect("Could not write to stream");
        self.stream
            .write(json_bytes)
            .expect("Could not write to stream");

        return self.read_task_response();
    }

    pub fn read_task_response(&mut self) -> (FragmentTask, Vec<u8>) {
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
        (task, data_buffer)
    }

    pub fn send_result(&mut self, result: FragmentResult, datas: Vec<PixelIntensity>, id: Vec<u8>) {
        let serialized = result.serialize();
        let json_bytes = serialized.as_bytes();
        let msg_len: u32 = json_bytes.len() as u32;

        //Total message size = message size + count (Id size in bytes) + number of pixels * ( 4 bytes for zn (u32) + 4 bytes for count (u32)).
        let total_msg_len: u32 = msg_len + (result.pixels.offset + result.pixels.count * (4 + 4));
        println!(
            "{:?} {:?}",
            &datas[0].zn.to_be_bytes(),
            &datas[0].count.to_be_bytes()
        );
        //send Total message size
        let a = total_msg_len.to_be_bytes();
        self.stream.write(&a).expect("Could not write to stream");

        //send Json message size
        let b = msg_len.to_be_bytes();
        self.stream.write(&b).expect("Could not write to stream");

        //send Json message (FragmentResult)
        self.stream
            .write(json_bytes)
            .expect("Could not write to stream");

        //send Id

        self.stream
            .write(&id.as_bytes())
            .expect("Could not write to stream");

        //send zn and count for each pixels
        for pixel in datas {
            let zn = pixel.zn;
            let count = pixel.count;
            self.stream
                .write(&zn.to_be_bytes())
                .expect("Could not write to stream");
            self.stream
                .write(&count.to_be_bytes())
                .expect("Could not write to stream");
        }

        println!("Done successfully");
    }
}


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
    let request = FragmentRequest::new(String::from("worker"), 10);
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

