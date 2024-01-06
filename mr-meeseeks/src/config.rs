use std::{io, fs};
use log::{warn, debug};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Config {
    pub server_address: String,
    pub port: String,
    pub worker_name: String,
    pub max_work: u32
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigToml {
    server: Option<ConfigTomlServer>,
    worker: Option<ConfigTomlWorker>
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigTomlServer {
    server_address: Option<String>,
    port: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigTomlWorker {
    worker_name: Option<String>,
    max_work: Option<u32>
}

impl Config {
    pub fn read() -> Self{
        let config_filepaths: [&str; 2] = [
            "./mr-meeseeks/config.toml",
            "./config.rs"
        ];

        let mut content: String = "".to_owned();

        for filepath in config_filepaths{
            let result: Result<String, io::Error> = fs::read_to_string(filepath);

            match result {
                Ok(e) => {
                    content = e;
                    break;
                },
                Err(_) => {},
            }
        }

        let config_toml_result = toml::from_str(&content);
        let config_toml = match config_toml_result {
            Ok(r) => r,
            Err(_) => {
                ConfigToml {
                    server: None,
                    worker: None
                }
            }
        };

        debug!("{:?}", config_toml);

        let (server_address, port): (String, String) = match config_toml.server {
            Some(server) => {
                let server_serv_address: String = server.server_address.unwrap_or_else(|| {
                    warn!("Missing field server address in table server.");
                    "localhost".to_owned()
                });

                let server_port: String = server.port.unwrap_or_else(|| {
                    warn!("Missing field port in table server.");
                    "8787".to_owned()
                });

                (server_serv_address, server_port)
            },
            None => {
                warn!("Missing table server, default value will be use.");
                ("localhost".to_owned(), "8787".to_string())
            }
        };

        let (worker_name, max_work): (String, u32) = match config_toml.worker {
            Some(worker) => {
                let work: String = worker.worker_name.unwrap_or_else(|| {
                    warn!("Missing field worker name in table worker.");
                    "Group 7".to_owned()
                });

                let max: u32 = worker.max_work.unwrap_or_else(|| {
                    warn!("Missing field max work in table worker.");
                    100
                });

                (work, max)

            },
            None => {
                warn!("Missing table Worker, default value will be use.");
                ("Group 7".to_owned(), 100)
            }
        };

        Config {
            server_address,
            port,
            worker_name,
            max_work
        }
    }
}
