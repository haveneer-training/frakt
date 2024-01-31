use std::net::SocketAddr;

pub fn validate_worker_argument(arguments: Vec<String>) -> Result<SocketAddr, Box<dyn std::error::Error>> {
    match arguments.len() {
        0 | 1 => panic!("Needs one argument: IP address of the server"),
        2 => (),
        _ => panic!("Too many arguments, only one is needed: IP address of the server")
    }
    let Ok(server_address) = arguments[1].parse::<SocketAddr>()
        else {
            panic!("Invalid IP address: {}\nWrite it using the following format 127.0.0.1:8000", arguments[1]);
        };
    Ok(server_address)
}