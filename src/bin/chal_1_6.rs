use itertools::Itertools;
use rustopals::{raw::EverythingRemainsRaw, RBX};
use std::{fs, env};

fn do_chal(file: &str, max_k_len: usize) -> RBX {
    // Load the file, remove newlines to make one long line of b64 and decode
    let raw_input = Vec::from_base64(
        &fs::read_to_string(file)
            .expect("Error reading file")
            .split_ascii_whitespace()
            .join(""),
    );

    // Avoid divide by zero by forcing input data to be at least as big as 2 key-lengths
    assert!(raw_input.len() > max_k_len * 2, "Error: Data length ({} bytes) must be at least twice the max key length ({} bytes)", raw_input.len(), max_k_len);
    RBX::from_ciphertext(&raw_input, max_k_len)
}

// Provides a commandline decrypter
// `$ cargo run --bin chal_1_6 <input file of b64> <max key length to search, default 40>`
fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args
        .get(1)
        .expect("Argument missing: provide a path to the file to read");

    let max_k_len: usize = args
        .get(2)
        .unwrap_or(&"40".to_owned())
        .parse()
        .expect("Argument error: 2nd argument for max key length should be an int");

    assert!(max_k_len >= 1, "Argument error: 2nd argument for max key length should be greater than or equal to 1");

    let a = do_chal(path, max_k_len);

    // Print stats to stderr so that the plaintext can be piped into a file cleanly
    eprintln!("Key length (bytes): {}", a.chunk_info.k_len);
    eprintln!("Key: {}", a.key.to_string());
    eprintln!("\nPlaintext:\n");
    print!("{}", a.plaintext.to_string());
}

#[test]
fn chal_1_6() {
    // Obfuscated ;)
    let expected_key = Vec::from_base64("VGVybWluYXRvciBYOiBCcmluZyB0aGUgbm9pc2U=");
    assert_eq!(do_chal("test_data/6.txt", 40).key, expected_key);
}
