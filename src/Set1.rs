extern crate serialize;

use serialize::base64::{mod, ToBase64};
use serialize::hex::FromHex;

fn main() {
    let challenge_string = String::from_str("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    println!("{}", ch1(challenge_string));
}

// Challenge1
fn ch1(input_hex: String) -> String {
    let output_base64 = input_hex.from_hex().unwrap().as_slice().to_base64(base64::STANDARD);
    output_base64
}

