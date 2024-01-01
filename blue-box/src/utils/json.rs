use crate::types::protocols::Fragment;

pub fn to_string(message: &Fragment) -> Result<String, serde_json::Error> {
    serde_json::to_string(message)
}

pub fn to_fragment(message: &str)-> Result<Fragment, serde_json::Error>  {
    serde_json::from_str(message)
}
