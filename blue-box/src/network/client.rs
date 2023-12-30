use std::{net::TcpStream, io};

use crate::utils::json;

use super::{network::Network, communication_types::{FragmentTask, FragmentRequest, NetworkProtocoles, FragmentResult}};


#[derive(Debug)]
pub struct Client {
    network: Network
}

impl Client {

    pub fn new(server_address: String, port: String) -> Client {
        Client {
            network: Network::new(server_address, port)
        }
    }

    pub fn connect_to_server(&self) -> Result<TcpStream, io::Error> {
        TcpStream::connect(self.network.get_fulladdress())
    }

    pub fn ask_for_work(
        stream: &mut TcpStream,
        worker_name: String,
        maximal_work_load: u32,
    ) -> Result<(FragmentTask, Vec<u8>), io::Error> {
        let work_request = FragmentRequest {
            worker_name,
            maximal_work_load,
        };

        let enum_network = NetworkProtocoles::FragmentRequest(work_request);
        let work_serialized = json::to_string(&enum_network)?;

        let data_tmp: Vec<u8> = Vec::new();
        Network::send_message(stream, work_serialized, data_tmp)?;

        match Network::read_message(stream) {
            Ok((NetworkProtocoles::FragmentTask(fragment_task), data)) => Ok((fragment_task, data)),
            Ok((NetworkProtocoles::FragmentRequest(_), _ )) => Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Not the right response type returned FragmentRequest",
            )),
            Ok((NetworkProtocoles::FragmentResult(_), _ )) => Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Not the right response type returned FragmentResult",
            )),
            Err(err) => Err(err),
        }
    }
    
    pub fn send_work_done(stream: &mut TcpStream, fragment_result: FragmentResult, data: Vec<u8>) -> Result<(), io::Error> {
        let enum_network = NetworkProtocoles::FragmentResult(fragment_result);
        let work_serialized = json::to_string(&enum_network)?;

        Network::send_message(stream, work_serialized, data)?;

        Ok(())
    }

}
