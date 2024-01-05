use std::{io, fs};

#[derive(Debug)]
pub struct Config {
    pub server_address: String,
    pub port: String,
    pub worker_name: String,
    pub max_work: u8
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

        println!("configfile -> {}", content);

        Config {
            server_address: "".to_string(),
            port: "".to_string(),
            worker_name: "".to_string(),
            max_work: 100
        }
    }
}
