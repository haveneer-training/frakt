use log::{debug, trace};

use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

use crate::utils::json;

use super::communication_types::NetworkProtocoles;

#[derive(Debug)]
pub struct Network {
    server_address: String,
    port: String,
}

impl Network {

    pub fn new(server_address: String, port: String) -> Network {
        Network {
            server_address,
            port,
        }
    }

    pub fn get_fulladdress(&self) -> String {
        format!("{}:{}", self.server_address, self.port)
    }

    pub fn send_message(
        stream: &mut TcpStream,
        json_message: String,
        data: Vec<u8>,
    ) -> Result<(), io::Error> {
        let json_message_size = json_message.len() as u32;
        let data_message_size = data.len() as u32;
        let total_message_size: u32 = json_message_size + data_message_size;

        stream.write_all(&total_message_size.to_be_bytes())?;
        stream.write_all(&json_message_size.to_be_bytes())?;
        stream.write_all(&json_message.as_bytes())?;
        stream.write_all(&data)?;

        debug!("Sent message : {json_message}", );

        Ok(())
    }

    pub fn read_message(stream: &mut TcpStream) -> Result<(NetworkProtocoles, Vec<u8>), io::Error> {
        let mut total_len_buf = [0; 4];
        stream.read_exact(&mut total_len_buf)?;
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

        debug!("Data size: {:?}", data.len());
        // let data = String::from_utf8_lossy(&data_len_buf);

        Ok((fragment, data))
    }
}
