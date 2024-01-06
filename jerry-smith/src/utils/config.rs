use std::{fs, io};
use log::{debug, warn, trace};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct Config {
    pub server_address: String,
    pub port: String
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigToml {
    server: Option<ConfigServer>,
}
#[derive(Debug, Serialize, Deserialize)]
struct ConfigServer {
    server_address: Option<String>,
    port: Option<String>
}

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
                    server: None
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


        Config{
            server_address,
            port,
        }
    }
}
