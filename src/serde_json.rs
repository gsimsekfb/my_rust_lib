// skip

// Slightly modified version from book:
// Rust in Action 1st Edition by Tim McNamara - 2021
// Ch 7, pg 215

// Also see:
// https://github.com/serde-rs/json

use bincode::serialize as to_bincode;
use serde_cbor::to_vec as to_cbor;
use serde_json::to_string as to_json;
use serde_derive::{Serialize};

// Instructs the serde_derive crate to write the necessary code to carry out
// the conversion from an in-memory City obj. to on-disk City obj.
#[derive(Serialize)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64,
}

// 1. Obj to json
#[test] fn ex1_to_json() {
    let calabar = City {
        name: String::from("Calabar"),
        population: 470_000,
        latitude: 4.95,
        longitude: 8.33,
    };

    // CBOR and bincode are more compact than JSON but at the
    // expense of being machine-readable only
    let as_json    = to_json(&calabar).unwrap();
    let as_cbor    = to_cbor(&calabar).unwrap();
    let as_bincode = to_bincode(&calabar).unwrap();

    println!("json:\n{}\n", &as_json);
    println!("cbor:\n{:?}\n", &as_cbor);
    println!("bincode:\n{:?}\n", &as_bincode);
    println!("json (as UTF-8):\n{}\n", String::from_utf8_lossy(as_json.as_bytes()));
    println!("cbor (as UTF-8):\n{:?}\n", String::from_utf8_lossy(&as_cbor));
    println!("bincode (as UTF-8):\n{:?}\n", String::from_utf8_lossy(&as_bincode));

    let expected_str = 
        r#"{"name":"Calabar","population":470000,"latitude":4.95,"longitude":8.33}"#;
    assert_eq!(as_json, expected_str);
}
// Out: 
/*
json:
{"name":"Calabar","population":470000,"latitude":4.95,"longitude":8.33}

cbor:
[164, 100, 110, 97, 109, 101, 103, 67, 97, 108, 97, 98, 97, 114, 106, 112, 111, 112, 117, 108, 97, 116, 105, 111, 110, 26, 0, 7, 43, 240, 104, 108, 97, 116, 105, 116, 117, 100, 101, 251, 64, 19, 204, 204, 204, 204, 204, 205, 105, 108, 111, 110, 103, 105, 116, 117, 100, 101, 251, 64, 32, 168, 245, 194, 143, 92, 41]

bincode:
[7, 0, 0, 0, 0, 0, 0, 0, 67, 97, 108, 97, 98, 97, 114, 240, 43, 7, 0, 0, 0, 0, 0, 205, 204, 204, 204, 204, 204, 19, 64, 41, 92, 143, 194, 245, 168, 32, 64]

json (as UTF-8):
{"name":"Calabar","population":470000,"latitude":4.95,"longitude":8.33}

cbor (as UTF-8):
"�dnamegCalabarjpopulation\u{1a}\0\u{7}+�hlatitude�@\u{13}������ilongitude�@ ��\u{8f}\\)"

bincode (as UTF-8):
"\u{7}\0\0\0\0\0\0\0Calabar�+\u{7}\0\0\0\0\0������\u{13}@)\\���� @
*/


// 2. json to obj

// Also see:
// https://github.com/serde-rs/json

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    address: Address,
    phone_numbers: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Address {
    street: String,
    city: String,
    country: String,
}

#[test] fn ex2_from_json() {
    let json = r#"{
        "FirstName": "John",
        "LastName": "Doe",
        "Age": 43,
        "Address": {
            "Street": "Downing Street 10",
            "City": "London",
            "Country": "Great Britain"
        },
        "PhoneNumbers": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

    let person: Person = serde_json::from_str(json).expect("Invalid JSON");
    println!("{:#?}", person);
        /* 
        Person {
            first_name: "John",
            last_name: "Doe",
            age: 43,
            address: Address {
                street: "Downing Street 10",
                city: "London",
                country: "Great Britain",
            },
            phone_numbers: [
                "+44 1234567",
                "+44 2345678",
            ],
        }
        */    

    assert_eq!(person.first_name, "John");
    assert_eq!(person.age, 43);
}
