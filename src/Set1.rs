extern crate serialize;

use serialize::base64::{mod, ToBase64};
use serialize::hex::FromHex;
use serialize::hex::ToHex;

fn main() {
    // Challenge 1
    let challenge_string = String::from_str("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let mut expected = String::from_str("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    let mut result = ch1(challenge_string);
    assert!(result == expected);
    println!("Challenge1 result = {}", result);

    // Challenge 2
    expected = String::from_str("746865206b696420646f6e277420706c6179");
    let challenge_string1 = String::from_str("1c0111001f010100061a024b53535009181c");
    let challenge_string2 = String::from_str("686974207468652062756c6c277320657965");
    result = ch2(challenge_string1, challenge_string2);
    assert!(result == expected);
    println!("Challenge2 result = {}", result);

    // Challenge 3
}

// Challenge1 : Convert hex to base64
fn ch1(input_hex: String) -> String {
    let output_base64 = input_hex.from_hex().unwrap().as_slice().to_base64(base64::STANDARD);
    output_base64
}

// Challenge2 : Fixed XOR
fn ch2(string_1: String, string_2: String) -> String {
    assert!(string_1.len() == string_2.len());
    let string_1 = string_1.from_hex().unwrap();
    let string_2 = string_2.from_hex().unwrap();
    let output_base64 = string_1.iter().zip(string_2.iter()).map(|(a,b)| *a ^ *b).collect::<Vec<u8>>();
    let result = output_base64.to_hex();
    result
}

// Challenge3 : Single-byte XOR cipher

