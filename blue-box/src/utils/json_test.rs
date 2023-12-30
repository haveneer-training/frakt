#![cfg(test)]

use std::process;

use crate::{network::communication_types::{NetworkProtocoles, FragmentRequest}, utils::json};


#[test]
fn test_convert_fragment_into_string() {
    let fragment = NetworkProtocoles::FragmentRequest(FragmentRequest {
        worker_name: String::from("test_worker"),
        maximal_work_load: 100,
    });
    let tested_response_request = json::to_string(&fragment);
    let tested_response = match tested_response_request {
        Ok(rep) => rep,
        Err(_) => process::exit(1)
    };
    let response = "{\"FragmentRequest\":{\"worker_name\":\"test_worker\",\"maximal_work_load\":100}}";

    assert_eq!(tested_response, response);
}

#[test]
fn test_convert_string_into_fragment() {
    let input_string = "{\"FragmentRequest\":{\"worker_name\":\"test_worker\",\"maximal_work_load\":100}}";

    let fragment = NetworkProtocoles::FragmentRequest(FragmentRequest {
        worker_name: String::from("test_worker"),
        maximal_work_load: 100,
    });

    let tested_response_request = json::to_fragment(&input_string);
    let tested_response = match tested_response_request {
        Ok(rep) => rep,
        Err(_) => process::exit(1)
    };

    assert_eq!(tested_response, fragment)
}

#[test]
fn test_deserialization_object_incorect() {
    let input_string = "{\"worker_name\":\"test_worker\",\"maximal_work_load\":100}";

    let tested_response_request = json::to_fragment(&input_string);
    assert!(tested_response_request.is_err());

}
