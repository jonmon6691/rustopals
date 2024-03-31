use std::iter::zip;
use rustopals::raw::EverythingRemainsRaw;

fn main() {
    println!("https://cryptopals.com/sets/1/challenges/2 - Fixed XOR");

    let input_bytes = Vec::from_hex("1c0111001f010100061a024b53535009181c");
    let key_bytes = Vec::from_hex("686974207468652062756c6c277320657965");
    let pt = String::from_utf8(zip(input_bytes.iter(), key_bytes.iter()).map(|(a, b)| a ^ b).collect::<Vec<u8>>()).unwrap();

    println!("Key:       {}", String::from_utf8(key_bytes.clone()).unwrap());
    println!("Plaintext: {}", pt);
}

#[test]
fn chal_1_2() {
    let input_bytes = Vec::from_hex("1c0111001f010100061a024b53535009181c");
    let key_bytes = Vec::from_hex("686974207468652062756c6c277320657965");
    let expected = Vec::from_hex("746865206b696420646f6e277420706c6179");

    let pt: Vec<u8> = zip(input_bytes.iter(), key_bytes.iter()).map(|(a, b)| a ^ b).collect();
    
    assert_eq!(pt, expected);
}
