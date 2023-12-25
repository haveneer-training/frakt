#![cfg(test)]

use crate::Network;

#[test]
fn test_start_server() {
    let server = Network::new("127.0.0.1".to_string(),"8080".to_string());

    let stream = server.start_server();
    assert!(stream.is_ok());
}

#[test]
fn test_start_server_with_bad_address() {
    let server = Network::new("1274".to_string(),"8080".to_string());

    let stream = server.start_server();
    assert!(stream.is_err());
}
