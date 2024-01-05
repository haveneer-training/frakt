use std::{io, fs};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Config {
    pub server_address: String,
    pub port: String,
    pub worker_name: String,
    pub max_work: u16
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
    max_work: Option<u16>
}

impl Config {
    pub fn new() -> Self{
        let config_filepaths: [&str; 2] = [
            "./mr-meeseeks/config.toml",
            "./Config.toml"
        ];

        let mut content: String = "".to_owned();

        for filepath in config_filepaths{
            let result: Result<String, io::Error> = fs::read_to_string(filepath);

            if result.is_ok(){
                content = result.unwrap();
                break;
            }
        }
        println!("{}", content);

        let config_toml: ConfigToml = toml::from_str(&content).unwrap_or_else(|_| {
            println!("Failed to create ConfigToml object out of config file");
            ConfigToml{
                server: None,
                worker: None
            }
        });

        let (server_address, port): (String, String) = match config_toml.server {
            Some(server) => {
                let server_serv_address: String = server.server_address.unwrap_or_else(|| {
                    println!("Missing field server address in table server.");
                    "localhost".to_string()
                });

                let server_port: String = server.port.unwrap_or_else(|| {
                    println!("Missing field password in table server.");
                    "8787".to_string()
                });

                (server_serv_address, server_port)
            },
            None => {
                println!("Missing table server");
                ("unknown".to_string(), "unknown".to_string())
            }
        };

        let (worker_name, max_work): (String, u16) = match config_toml.worker {
            Some(worker) => {
                let work: String = worker.worker_name.unwrap_or_else(|| {
                    println!("Missing field worker name in table worker.");
                    "Group 7".to_string()
                });

                let max: u16 = worker.max_work.unwrap_or_else(|| {
                    println!("Missing field max work in table worker.");
                    100
                });

                (work, max)

            },
            None => {
                println!("Missing table Worker");
                ("unknown".to_string(), 0)
            }
            
        };

        println!("{} {}", server_address, port);

        Config {
            server_address,
            port,
            worker_name,
            max_work
        }
    }
}
