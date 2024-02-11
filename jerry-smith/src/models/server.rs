use std::{net::{TcpListener, TcpStream}, io};

use blue_box::{models::network::Network, types::{protocols::{FragmentRequest, Fragment, FragmentTask, FragmentResult}, desc::{U8Data, Resolution, Range, Point}, fractal_type::{FractalDescriptor, JuliaDescriptor}}};
use cmplx_nbr::Complex;
use log::trace;

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

    pub fn read_messge_from_client(stream: &mut TcpStream) -> Result<(Fragment, Vec<u8>), io::Error>{
        return Network::read_message(stream);
    }

    pub fn send_work(
        stream: &mut TcpStream,
        fragment_request: &FragmentRequest
    ) -> Result<(), io::Error> {
        let test_fragment_task = FragmentTask {
            id: U8Data {
                offset: 0,
                count: 3,
            },
            fractal: {
                FractalDescriptor::Julia(JuliaDescriptor {
                    c: Complex{ re: 0.285, im: 0.013 },
                    divergence_threshold_square: 4.0,
                })
            },
            max_iteration: 64,
            resolution: Resolution { nx: 1, ny: 1},
            range: Range {
                min: Point { x: 13.4, y: 18.4 },
                max: Point { x: 13.4, y: 18.4 },
            },
        };

        let network_test_fragment = Fragment::FragmentTask(test_fragment_task);

        let mut data_tmp: Vec<u8> = vec![13, 12, 14];
        Network::send_message(stream, network_test_fragment, &mut data_tmp)?;

        Ok(())
    }

    pub fn get_work_done(
        stream: &mut TcpStream,
        fragment_result: &FragmentResult,
        data: Vec<u8>
    ) -> Result<(), io::Error> {
        trace!("Data recived : {data:?}");
        Ok(())
    }

}
