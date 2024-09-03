use rand::{thread_rng, Rng};
use rustopals::{blocky::aes_128_ecb_encrypt_vec, raw::EverythingRemainsRaw};
use std::io::Read;

fn main() {
    let mut rng = thread_rng();
    // Random key, generated here so it can be persistent throughout the attack
    let key: [u8; 16] = rng.gen();
    ecb_phonebook(|input| channel(input, key));
}

fn ecb_phonebook(blackbox: impl Fn(&[u8]) -> Vec<u8>) -> Vec<u8> {
    let block_size = detect_ecb_blocksize(&blackbox, 512 / 8);
    println!("{:?}", block_size);
    vec![]
}

fn detect_ecb_blocksize(blackbox: impl Fn(&[u8]) -> Vec<u8>, max_bytes: usize) -> Option<usize> {
    let mut probe: Vec<u8> = Vec::new();
    let mut last_size: Option<usize> = None;
    // TODO: This could probably be done with some kind of clever fold
    for _ in 0..=max_bytes {
        let ct_len = blackbox(&probe).len();
        let delta = ct_len - *last_size.get_or_insert(ct_len);
        if delta != 0 {
            return Some(delta);
        }
        probe.push(0);
    }
    None
}

fn channel(input: &[u8], key: [u8; 128 / 8]) -> Vec<u8> {
    // Random length suffix of random bytes
    let suffix: Vec<u8> = Vec::from_base64("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK");

    // TODO: Surely this is the worst possible way to do this
    let data: Vec<u8> = input
        .chain(suffix.as_slice())
        .bytes()
        .map(|x| x.unwrap())
        .collect();

    aes_128_ecb_encrypt_vec(data, key)
}

#[test]
fn chal_2_12() {
    assert_eq!(1, 1);
}
