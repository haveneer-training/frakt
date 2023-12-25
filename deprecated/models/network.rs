use std::{net::TcpStream, io::{self, Write, Read}};
use super::commmunication::{FragmentRequest, NetworkProtocoles, FragmentTask};
use super::super::utils::json;

#[derive(Debug)]
pub struct Network {
    server_address : String,
    port : String
}

impl Network {
    pub fn new(server_address: String, port: String) -> Network {
        Network {server_address, port}
    }

    pub fn get_server_connection(&self) -> Result<TcpStream, io::Error>{
        return TcpStream::connect(self._get_fulladdress());
    }

    pub fn ask_for_work(&self, stream: &mut TcpStream, worker_name: String, maximal_work_load: u32) -> Result<FragmentTask, io::Error>{
        let work_request = FragmentRequest { worker_name, maximal_work_load};

        let enum_network = NetworkProtocoles::FragmentRequest(work_request);
        let work_serialized = json::to_string(&enum_network)?;

        self._send_message(stream, work_serialized, "".to_string())?;

        match self._read_message(stream) {
            Ok(NetworkProtocoles::FragmentTask(fragment_task)) => Ok(fragment_task),
            Ok(NetworkProtocoles::FragmentRequest(_)) => Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not the right response type")),
            Err(err) => Err(err)
        }        
        
    }

    fn _get_fulladdress(&self) -> String {
        format!("{}:{}", self.server_address, self.port)
    }

    fn _send_message(&self, stream: &mut TcpStream, json_message: String, data: String) -> Result<(), io::Error> {
        let json_message_size = json_message.len() as u32;
        let data_message_size = data.len() as u32;
        let total_message_size: u32 = json_message_size + data_message_size;

        stream.write(&total_message_size.to_be_bytes())?;
        stream.write(&json_message_size.to_be_bytes())?;
        stream.write(&json_message.as_bytes())?;
        stream.write(&data.as_bytes())?;

        Ok(())
    }

    fn _read_message(&self, stream: &mut TcpStream) -> Result<NetworkProtocoles, io::Error> {
        let mut total_len_buf = [0; 4];
        stream.read_exact(&mut total_len_buf)?;
        let total_message_size = u32::from_be_bytes(total_len_buf);

        let mut json_len_buf = [0; 4];
        stream.read_exact(&mut json_len_buf)?;
        let json_message_size = u32::from_be_bytes(json_len_buf);

        if total_message_size < json_message_size {
            return Err(io::Error::new(io::ErrorKind::Other, "Json message size if bigger than total message size"));
        }

        let data_message_size = total_message_size - json_message_size;

        let mut sbuf = vec![0_u8; json_message_size as usize];
        stream.read(&mut sbuf)?;
        let s = String::from_utf8_lossy(&sbuf);
        let fragment_request = json::to_fragment(&s.to_string());
        let fragment = match fragment_request {
            Ok(r) => r,
            Err(_) => {
                return Err(io::Error::new(io::ErrorKind::Other, "Message received by server cannot be deserialized"));
            }
        };

        let mut data_len_buf = vec![0_u8; data_message_size as usize];
        stream.read(&mut data_len_buf)?;
        // let data = String::from_utf8_lossy(&data_len_buf);

        Ok(fragment)
    }
}
