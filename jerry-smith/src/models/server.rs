use std::{
    net::{
        TcpListener,
        TcpStream
    },
    io, sync::mpsc::Sender,
};
use blue_box::{
    models::network::Network,
    types::{
        protocols::{
            FragmentRequest,
            Fragment,
            FragmentTask,
            FragmentResult
        }, 
        desc::{
            U8Data,
            Resolution,
            Range,
            Point, PixelIntensity
        },
        fractal_type::{
            FractalDescriptor, 
            JuliaDescriptor
        }
    }
};
use cmplx_nbr::Complex;
use log::{trace, info, debug, warn};
use rand::Rng;

#[derive(Debug)]
pub struct Server {
    network: Network,
    width: u16,
    height: u16
}

impl Server {

    pub fn new(
        server_address: String,
        port: String,
        width: u16,
        height: u16
    ) -> Server{
        Server {
            network: Network::new(server_address, port),
            width,
            height
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
                count: 16,
            },
            fractal: {
                FractalDescriptor::Julia(JuliaDescriptor {
                    c: Complex{ re: 0.285, im: 0.013 },
                    divergence_threshold_square: 4.0,
                })
            },
            max_iteration: 64,
            resolution: Resolution { nx: 400, ny: 400},
            range: Range {
                min: Point { x: -1.2, y:-1.},
                max: Point { x: 1.2, y: 1.2 },
            },
        };

        let network_test_fragment = Fragment::FragmentTask(test_fragment_task);

        let mut data_tmp: Vec<u8> = Server::get_task_id();
        info!("New Date : {data_tmp:?}");
        Network::send_message(stream, network_test_fragment, &mut data_tmp)?;

        Ok(())
    }

    pub fn get_work_done(
        fragment_result: &FragmentResult,
        data: Vec<u8>,
        tx: &Sender<Vec<PixelIntensity>>
    ) -> Result<(), io::Error> {
        warn!("fragment : {fragment_result:?}");
        trace!("Data recived : {data:?}");

        tx.send(Server::from_data_to_pixel_intensity(&data[16..]));

        Ok(())
    }

    pub fn handle_client(
        stream: &mut TcpStream,
        tx: &Sender<Vec<PixelIntensity>>
    ) -> Result<(), io::Error> {
        info!("Incoming connection {stream:?}");

        match Server::read_messge_from_client(stream){
            Ok((Fragment::FragmentRequest(fragment), _)) => {
                debug!("Work request received");
                Server::send_work(stream, &fragment);
            },
            Ok((Fragment::FragmentResult(fragment), data)) => {
                debug!("Work done");
                Server::get_work_done(&fragment, data, tx);
            },
            Ok((Fragment::FragmentTask(_), _)) => {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "The worker send a task",
                ));
            },
            Err(err) => {
                return Err(err);
            }
        };

        Ok(())
    }

    fn get_task_id() -> Vec<u8> {
        let mut rng = rand::thread_rng();
        (0..16).map(|_| rng.gen()).collect()
    }

    fn from_data_to_pixel_intensity(data: &[u8]) -> Vec<PixelIntensity> {
        //FIX: ChatGPT make this, can be shit (at least more than the rest of the code)
        let mut rep: Vec<PixelIntensity> = vec![];
        for chunk in data.chunks(8) {
            if chunk.len() == 8 {
                let (zn_slice, count_slice) = chunk.split_at(4);
                let zn: [u8; 4] = zn_slice.try_into().expect("slice with incorrect length");
                let count: [u8; 4] = count_slice.try_into().expect("slice with incorrect length");
                let pi = PixelIntensity {
                    zn: f32::from_be_bytes(zn),
                    count: f32::from_be_bytes(count),
                };
                rep.push(pi);
            }
        }

        rep
    }
}
