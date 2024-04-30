/// # Repeating key XOR aka Vigenere Cipher
/// 
/// Provides a simple commandline "encryption" tool
/// `$ cat plaintextfile.txt | cargo run --bin chal_1_5 <key> > ciphertext_file.b64`

use rustopals::raw::EverythingRemainsRaw;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();

    let key = args
        .get(1)
        .expect("Argument error: provide the key as an argument")
        .clone()
        .into_bytes();

    let plaintext = io::read_to_string(io::stdin())
        .expect("Error reading file from STDIN");

    // Function chaining feels so right
    let ct: Vec<u8> = plaintext
        .as_bytes()
        .iter()
        .zip(key.iter().cycle())
        .map(|(a, b)| a ^ b)
        .collect();

    print!("{}", ct.into_base64());
}

#[test]
fn chal_1_5() {
    let plaintext =
        String::from("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal")
            .into_bytes();
    let key = String::from("ICE").into_bytes();

    // Function chaining feels so right
    let ct: Vec<u8> = plaintext
        .iter()
        .zip(key.iter().cycle())
        .map(|(a, b)| a ^ b)
        .collect();

    let ct: String = ct.into_hex();
    let ct_expected = String::from("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");

    assert_eq!(ct, ct_expected);
}
