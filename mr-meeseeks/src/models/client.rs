use std::{net::TcpStream, io};

use blue_box::{models::network::Network, types::protocols::{FragmentTask, FragmentRequest, Fragment, FragmentResult}, utils::json};
use log::{debug, warn};
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

    pub fn ask_for_work(
        &self,
        worker_name: String,
        maximal_work_load: u32,
    ) -> Result<(FragmentTask, Vec<u8>), io::Error> {
        let mut stream = self.connect_to_server()?;

        let work_request = FragmentRequest {
            worker_name,
            maximal_work_load,
        };

        let enum_network = Fragment::FragmentRequest(work_request);

        let data_tmp: Vec<u8> = Vec::new();
        Network::send_message(&mut stream, enum_network, data_tmp)?;

        let (fragment, data) = match Network::read_message(&mut stream) {
            Ok((Fragment::FragmentTask(fragment_task), data)) => (fragment_task, data),
            Ok((Fragment::FragmentRequest(_), _ )) => return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Not the right response type returned FragmentRequest",
            )),
            Ok((Fragment::FragmentResult(_), _ )) => return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Not the right response type returned FragmentResult",
            )),
            Err(err) => return Err(err),
        };

        Network::close_connection(&mut stream);

        Ok((fragment, data))
    }

    pub fn send_work_done(&self, fragment_result: FragmentResult, data: Vec<u8>) -> Result<(FragmentTask, Vec<u8>), io::Error> {
        let mut stream = self.connect_to_server()?;

        let enum_network = Fragment::FragmentResult(fragment_result);

        Network::send_message(&mut stream, enum_network, data)?;
        let (fragment, data) = match Network::read_message(&mut stream) {
            Ok((Fragment::FragmentTask(fragment_task), data)) => (fragment_task, data),
            Ok((Fragment::FragmentRequest(_), _ )) => return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Not the right response type returned FragmentRequest",
            )),
            Ok((Fragment::FragmentResult(_), _ )) => return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Not the right response type returned FragmentResult",
            )),
            Err(err) => return Err(err),
        };

        Network::close_connection(&mut stream);

        Ok((fragment, data))
    }

    fn connect_to_server(&self) -> Result<TcpStream, io::Error> {
        TcpStream::connect(self.network.get_fulladdress())
    }
}
