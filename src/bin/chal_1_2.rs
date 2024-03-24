use std::iter::zip;

fn main() {
    println!("https://cryptopals.com/sets/1/challenges/2 - Fixed XOR");

    let input_bytes = rustopals::hex_decode("1c0111001f010100061a024b53535009181c");
    let key_bytes = rustopals::hex_decode("686974207468652062756c6c277320657965");
    println!("Key: '{}'", String::from_utf8(key_bytes.clone()).unwrap());

    let pt = String::from_utf8(
        zip(input_bytes.iter(), key_bytes.iter())
        .map(|(a, b)| a ^ b)
        .collect()).unwrap();

    println!("Plaintext: '{pt}'");
}
