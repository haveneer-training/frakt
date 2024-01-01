use std::{net::{TcpStream, Shutdown}, io::{self, Write, Read}};

use log::{debug, warn};

use crate::{types::protocols::Fragment, utils::json};

#[derive(Debug)]
pub struct Network {
    server_address: String,
    port: String
}

impl Network  {
    pub fn new(server_address: String, port: String) -> Network{
        Network {server_address, port}
    }

    pub fn get_fulladdress(&self) -> String {
        format!("{}:{}", self.server_address, self.port)
    }

    pub fn send_message(
        stream: &mut TcpStream,
        fragment: Fragment,
        data: &Vec<u8>
    ) -> Result<String , io::Error> {

        let json_message = json::to_string(&fragment)?;

        let json_message_size = json_message.len() as u32;
        let data_message_size = data.len() as u32;
        let total_message_size: u32 = json_message_size + data_message_size;

        stream.write_all(&total_message_size.to_be_bytes())?;
        stream.write_all(&json_message_size.to_be_bytes())?;
        stream.write_all(&json_message.as_bytes())?;
        stream.write_all(data)?;

        Ok(json_message)
    }

    pub fn read_message(stream: &mut TcpStream) -> Result<(Fragment , Vec<u8>), io::Error> {
        let mut total_len_buf = [0; 4];
        match stream.read_exact(&mut total_len_buf){
            Ok(_) => debug!("Start getting something"),
            Err(err) => {
                warn!("Something's wrong, I can't receive the total message size");
                return Err(err);
            } 
            
        };
        let total_message_size = u32::from_be_bytes(total_len_buf);

        let mut json_len_buf = [0; 4];
        stream.read_exact(&mut json_len_buf)?;
        let json_message_size = u32::from_be_bytes(json_len_buf);

        if total_message_size < json_message_size {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Json message size if bigger than total message size",
            ));
        }

        let data_message_size = total_message_size - json_message_size;

        let mut sbuf = vec![0_u8; json_message_size as usize];
        stream.read(&mut sbuf)?;
        let s = String::from_utf8_lossy(&sbuf);

        debug!("Message received : {s}");

        let fragment_request = json::to_fragment(&s.to_string());
        let fragment = match fragment_request {
            Ok(r) => r,
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Message received by server cannot be deserialized",
                ));
            }
        };

        let mut data = vec![0_u8; data_message_size as usize];
        stream.read(&mut data)?;

        // debug!("Data size: {:?}", data.len());
        // let data = String::from_utf8_lossy(&data_len_buf);

        Ok((fragment, data))
    }

    pub fn close_connection(stream: &mut TcpStream) {
        stream.shutdown(Shutdown::Both);
    }
}
