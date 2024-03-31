use std::{fs::File, io::Read};
use itertools::Itertools;
use rustopals::{raw::EverythingRemainsRaw, SBX};

const MAX_KEY_LEN: usize = 40;

fn do_chal() -> String {
    // Load the file, remove newlines to make one long line of b64
    let a = File::open("test_data/6.txt")
        .unwrap()
        .bytes()
        .map(|c| c.unwrap() as char)
        .collect::<String>()
        .split('\n').join("");

    // Decode b64
    let raw_input = Vec::from_base64(&a);

    // Avoid divide by zero by forcing input data to be at least as big as 2 key-lengths
    assert!(raw_input.len() > MAX_KEY_LEN * 2);

    // Find key length with functional programming ~magic~
    let (k_len, _score) = (1..MAX_KEY_LEN)
        .map(|ks| -> (usize, usize) {
            (ks,
            (0..raw_input.len()/ks - 1).map(|j| -> usize {
                rustopals::hamming(
                    &raw_input[j*ks..(j+1)*ks],
                    &raw_input[(j+1)*ks..(j+2)*ks]
                ) / ks
            }).sum::<usize>() * 100 / (raw_input.len() / ks - 1))
        }).sorted_by_key(|(_k_len, score)| *score)
        .next()
        .unwrap();

    println!("Key length (bytes): {}", k_len);

    // Find key buy performing parallel one-byte-xor's
    let k: Vec<u8> = (0..k_len)
        .map(|i| -> u8 {
            SBX::from_ciphertext(&raw_input
                .clone()
                .into_iter()
                .skip(i)
                .step_by(k_len)
                .collect::<Vec<u8>>()
            ).key
        }).collect();

    let k_str = String::from_utf8(k.clone()).unwrap();
    println!("Key: {}", &k_str);

    // Decrypt the ciphertext
    let plaintext = String::from_utf8(raw_input.iter().zip(k.iter().cycle()).map(|(a, b)| a ^ b).collect()).unwrap();
    println!("\nPlaintext:\n{}", plaintext);

    // Return the key
    k_str
}

fn main() {
    println!("Lets do this");
    do_chal();
}

#[test]
fn chal_1_6() {
    // Obfuscated ;)
    let expected_key = String::from_utf8(Vec::from_base64("VGVybWluYXRvciBYOiBCcmluZyB0aGUgbm9pc2U=")).unwrap();
    assert_eq!(do_chal(), expected_key);
}
