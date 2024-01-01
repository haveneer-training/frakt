use std::{net::{TcpListener, TcpStream}, io};

use blue_box::{models::network::Network, types::{protocols::{FragmentRequest, Fragment, FragmentTask, FragmentResult}, desc::{U8Data, Resolution, Range, Point}, fractal_type::{FractalDescriptor, JuliaDescriptor}}, utils::json};
use cmplx_nbr::Complex;

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

    pub fn get_work_request(
        stream: &mut TcpStream,
    ) -> Result<(FragmentRequest, Vec<u8>), io::Error> {
        match Network::read_message(stream) {
            Ok((Fragment::FragmentRequest(fragment), data)) => Ok((fragment, data)),
            Ok((Fragment::FragmentTask(_), _)) => Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Did not send a job request",
            )),
            Ok((Fragment::FragmentResult(_), _)) => Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Did not send a job request",
            )),
            Err(err) => Err(err),
        }
    }

    pub fn send_work(stream: &mut TcpStream) -> Result<(), io::Error> {
        let test_fragment_task = FragmentTask {
            id: U8Data {
                offset: 13,
                count: 13,
            },
            fractal: {
                FractalDescriptor::Julia(JuliaDescriptor {
                    c: Complex{ re: 13.2, im: 13.2 },
                    divergence_threshold_square: 13.4,
                })
            },
            max_iteration: 0,
            resolution: Resolution { nx: 100, ny: 100 },
            range: Range {
                min: Point { x: 13.4, y: 18.4 },
                max: Point { x: 13.4, y: 18.4 },
            },
        };

        let network_test_fragment = Fragment::FragmentTask(test_fragment_task);

        let mut data_tmp: Vec<u8> = Vec::new();
        Network::send_message(stream, network_test_fragment, &mut data_tmp)?;

        Ok(())
    }

    pub fn get_work_done(stream: &mut TcpStream) -> Result<(FragmentResult, Vec<u8>), io::Error> {
        match Network::read_message(stream) {
            Ok((Fragment::FragmentResult(fragment), data)) => Ok((fragment, data)),
            Ok((Fragment::FragmentTask(_), _)) => Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Did not send a job done",
            )),
            Ok((Fragment::FragmentRequest(_), _)) => Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Did not send a job done",
            )),
            Err(err) => Err(err),
        }
    }

}
