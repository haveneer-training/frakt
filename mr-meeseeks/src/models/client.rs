//! This Rust module defines a `Client` structure for handling network interactions in a distributed computing context, particularly for handling fragment tasks. 
//! The client is responsible for communicating with a server, requesting work, and sending completed tasks back. 
//! It uses the `Network` module for establishing TCP connections and the `protocols` module for the message format.
//!
//! The `Client` struct includes a `Network` instance to manage the connection details. It provides two main functionalities:
//! 
//! The client utilizes internal utility function `connect_to_server` to establish a TCP connection with the server,
//! and it ensures the connection is closed after each operation.
//! This module is crucial for maintaining efficient and reliable communication between the distributed computing client and server,
//! handling both the initiation of tasks and the return of completed work.
//!
//! ## Example: 
//! To create a new customer you can use the following code:
//! ```
//! let client = Client::new("184.194.9.1", "8787")
//! ```

use std::{net::TcpStream, io};

use blue_box::{models::network::Network, types::protocols::{FragmentTask, FragmentRequest, Fragment, FragmentResult}};
use log::debug;

/// The client structure contains server connection information
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

    /// This method requests a new `FragmentTask` from the server. 
    /// It sends a `FragmentRequest` to the server, including the worker's name and their maximal work capacity. 
    /// The server responds with a task and data to process. In case of an incorrect response type or network errors, appropriate errors are returned.
    pub fn ask_for_work(
        &self,
        worker_name: &str,
        maximal_work_load: u32,
    ) -> Result<(FragmentTask, Vec<u8>), io::Error> {
        let mut stream = self.connect_to_server()?;

        let work_request = FragmentRequest {
            worker_name: worker_name.to_string(),
            maximal_work_load,
        };

        let enum_network = Fragment::FragmentRequest(work_request);

        let mut data_tmp: Vec<u8> = Vec::new();
        Network::send_message(&mut stream, enum_network, &mut data_tmp)?;

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

    /// After processing a task, the client uses this method to send the `FragmentResult` back to the server.
    /// If the server accepts the result, it returns a new `FragmentTask`
    pub fn send_work_done(&self, fragment_result: FragmentResult, data: &mut Vec<u8>) -> Result<FragmentTask, io::Error> {
        let mut stream = self.connect_to_server()?;

        let enum_network = Fragment::FragmentResult(fragment_result);

        let sent_message = Network::send_message(&mut stream, enum_network, data)?;
        debug!("Sent message : {sent_message}");

        let fragment = match Network::read_message(&mut stream) {
            Ok((Fragment::FragmentTask(fragment_task), new_data)) => {
                *data = new_data;
                fragment_task
            },
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

        Ok(fragment)
    }

    fn connect_to_server(&self) -> Result<TcpStream, io::Error> {
        TcpStream::connect(self.network.get_fulladdress())
    }
}
