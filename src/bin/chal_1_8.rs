use std::fs;
use rustopals::{detect_ecb, raw::EverythingRemainsRaw};

fn main() {
    println!("Hola todos");
    let cts: Vec<Vec<u8>> = fs::read_to_string("test_data/8.txt")
        .expect("Error reading file")
        .split_ascii_whitespace()
        .map(|ct| Vec::from_hex(ct))
        .filter(|ct| detect_ecb(ct, 16))
        .collect();

    cts.iter().for_each(|ecb| println!("{}", ecb.clone().into_hex()));
}

#[test]
fn chal_1_8() {
    let cts: Vec<Vec<u8>> = fs::read_to_string("test_data/8.txt")
        .expect("Error reading file")
        .split_ascii_whitespace()
        .map(|ct| Vec::from_hex(ct))
        .filter(|ct| detect_ecb(ct, 16))
        .collect();

    assert!(cts.len() == 1);
    let result_b64 = cts.get(0).unwrap().clone().into_base64();
    let result_b64 = result_b64.get(0..16).unwrap();

    // Obfuscated ;-)
    assert_eq!(result_b64, "2IBhl0CooZt4QKij".to_owned());
}
