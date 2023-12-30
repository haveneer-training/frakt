use std::{net::{TcpListener, TcpStream}, io};

use crate::{fractal::fractal_types::{FreactalDescriptor, JuliaDescriptor}, utils::json};

use super::{network::Network, communication_types::{FragmentRequest, NetworkProtocoles, FragmentTask, U8Data, Complex, Resolution, Range, Point, FragmentResult}};


#[derive(Debug)]
pub struct Server {
    network: Network
}

impl Server {

    pub fn new(server_address: String, port: String) -> Server{
        Server {
            network: Network::new(server_address, port)
        }
    }

    pub fn start_server(&self) -> Result<TcpListener, io::Error> {
        TcpListener::bind(self.network.get_fulladdress())
    }

    
    pub fn get_work_request(stream: &mut TcpStream) -> Result<(FragmentRequest, Vec<u8>), io::Error> {

        match Network::read_message(stream) {
            Ok((NetworkProtocoles::FragmentRequest(fragment), data)) => Ok((fragment, data)),
            Ok((NetworkProtocoles::FragmentTask(_), _ )) => Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Did not send a job request",
            )),
            Ok((NetworkProtocoles::FragmentResult(_), _ )) => Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Did not send a job request",
            )),
            Err(err) => Err(err)
        }
    }

    pub fn send_work(stream: &mut TcpStream) -> Result<(), io::Error> {

        let test_fragment_task = FragmentTask{
            id: U8Data {
                offset: 13,
                count: 13
            },
            fractal: {
                FreactalDescriptor::Julia( JuliaDescriptor {
                    c: Complex {
                        re: 13.2,
                        im: 13.2
                    },
                    divergence_threshold_square: 13.4
                })
            },
            max_iteration: 0,
            resolution: Resolution {
                nx: 100,
                ny: 100
            },
            range: Range {
                min: Point {
                    x: 13.4,
                    y: 18.4,
                },
                max: Point {
                    x: 13.4,
                    y: 18.4,
                }
            }
        };

        let network_test_fragment = NetworkProtocoles::FragmentTask(test_fragment_task);
        let network_test_fragment_serialized = json::to_string(&network_test_fragment)?;

        let data_tmp: Vec<u8> = Vec::new();
        Network::send_message(stream, network_test_fragment_serialized, data_tmp)?;

        Ok(())
    }

    pub fn get_work_done(stream: &mut TcpStream) -> Result<(FragmentResult, Vec<u8>), io::Error>{
        match Network::read_message(stream) {
            Ok((NetworkProtocoles::FragmentResult(fragment), data)) => Ok((fragment, data)),
            Ok((NetworkProtocoles::FragmentTask(_), _ )) => Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Did not send a job done",
            )),
            Ok((NetworkProtocoles::FragmentRequest(_), _ )) => Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Did not send a job done",
            )),
            Err(err) => Err(err)
        }

    }
}
