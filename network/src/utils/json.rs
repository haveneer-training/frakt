use crate::models::commmunication::NetworkProtocoles;

pub fn to_string(message: &NetworkProtocoles) -> Result<String, serde_json::Error> {
    serde_json::to_string(message)
}

pub fn to_fragment(message: &str)-> Result<NetworkProtocoles, serde_json::Error>  {
    serde_json::from_str(message)
}
