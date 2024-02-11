use std::{fs, io, u16};
use log::{debug, warn, trace};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct Config {
    pub server_address: String,
    pub port: String,
    pub width: u16,
    pub height: u16
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigToml {
    server: Option<ConfigServer>,
    display: Option<ConfigDisplay>
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigServer {
    server_address: Option<String>,
    port: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigDisplay {
    width: Option<u16>,
    height: Option<u16> 
}

// FIX: Possible to make this file cleaner

impl Config {
    pub fn read() -> Self {

        let config_filepath: [&str; 2] = [
            "./jerry-smith/config.toml",
            "./config.toml"
        ];

        let mut content: String = "".to_owned();

        for filepath in config_filepath{
            let result: Result<String, io::Error> = fs::read_to_string(filepath);

            match result {
                Ok(e) => {
                    content = e;
                    break;
                },
                Err(_) => {},
            };
        }

        trace!("Value of file {content}");

        let config_toml: ConfigToml= match toml::from_str(&content){
            Ok(r) => r,
            Err(_) => {
                ConfigToml{
                    server: None,
                    display: None
                }
            }
        };

        debug!("{:?}", config_toml);

        let (server_address, port): (String, String) = match config_toml.server {
            Some(server) => {
                let serv_address: String = server.server_address.unwrap_or_else(|| {
                    warn!("Missing field server address in table server.");
                    "localhost".to_owned()
                });

                let server_port: String = server.port.unwrap_or_else(|| {
                    warn!("Missing field port in table server.");
                    "8787".to_owned()
                });

                (serv_address, server_port)
            },
            None => {
                warn!("Missing table server, default value will be use.");
                ("localhost".to_owned(), "8787".to_owned())
            }
        };

        let (width, height): (u16, u16) = match config_toml.display {
            Some(display) => {
                let wi: u16 = display.width.unwrap_or_else(|| {
                    warn!("Missing field width in table display");
                    400
                });

                let he: u16 = display.height.unwrap_or_else(|| {
                    warn!("Missing field height in table display");
                    400
                });

                (wi, he)
            },
            None => {
                warn!("Missing table display, default value will be use?");
                (400, 400)
            }
        };


        Config{
            server_address,
            port,
            width,
            height
        }
    }
}
