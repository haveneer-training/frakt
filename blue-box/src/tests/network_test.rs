#![cfg(test)]

use crate::models::network::Network;

#[test]
fn test_network_new_and_get_fulladdress() {
    let server_address = "127.0.0.1".to_string();
    let port = "8080".to_string();
    let network = Network::new(server_address.clone(), port.clone());

    assert_eq!(network.get_fulladdress(), format!("{}:{}", server_address, port));
}
