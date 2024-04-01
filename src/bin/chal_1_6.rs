use std::fs;
use itertools::Itertools;
use rustopals::{raw::EverythingRemainsRaw, RBX};

const MAX_KEY_LEN: usize = 40;

fn do_chal() -> RBX {
    // Load the file, remove newlines to make one long line of b64 and decode
    let raw_input = Vec::from_base64(&fs::read_to_string("test_data/6.txt")
        .expect("Error reading file")
        .split('\n')
        .join(""));

    // Avoid divide by zero by forcing input data to be at least as big as 2 key-lengths
    assert!(raw_input.len() > MAX_KEY_LEN * 2);
    RBX::from_ciphertext(&raw_input, MAX_KEY_LEN)
}

fn main() {
    println!("Lets do this");
    let a = do_chal();

    println!("Key length (bytes): {}", a.k_len);

    let k_str = String::from_utf8(a.key.unwrap().clone())
        .unwrap_or("[Not valid UTF-8]".to_owned());
    println!("Key: {}", &k_str);

    // Decrypt the ciphertext
    let plaintext = a.plaintext.unwrap_or("[Not valid UTF-8]".to_owned());
    println!("\nPlaintext:\n{}", plaintext);
}

#[test]
fn chal_1_6() {
    // Obfuscated ;)
    let expected_key = Vec::from_base64("VGVybWluYXRvciBYOiBCcmluZyB0aGUgbm9pc2U=");
    assert_eq!(do_chal().key.unwrap(), expected_key);
}
