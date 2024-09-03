use rand::{thread_rng, Rng};
use rustopals::{blocky::aes_128_ecb_encrypt_vec, raw::EverythingRemainsRaw};

fn main() {
    let mut rng = thread_rng();
    // Random key, generated here so it can be persistent throughout the attack
    let key: [u8; 16] = rng.gen();
    ecb_phonebook(|input| channel(input, key));
}

fn ecb_phonebook(blackbox: impl Fn(&[u8]) -> Vec<u8>) -> Vec<u8> {
    let block_size = detect_ecb_blocksize(&blackbox, 512 / 8);
    println!("Detected block size (bytes): {:?}", block_size);
    vec![]
}

/// Passes longer and longer strings to the black box encrypter until its size jumps. The size of the jump is assumed to be the block length
fn detect_ecb_blocksize(blackbox: impl Fn(&[u8]) -> Vec<u8>, max_bytes: usize) -> Option<usize> {
    let mut probe: Vec<u8> = Vec::with_capacity(max_bytes);
    let min_size = blackbox(&probe).len();
    for _ in 1..=max_bytes {
        probe.push(0);
        let delta = blackbox(&probe).len() - min_size;
        if delta != 0 {
            return Some(delta);
        }
    }
    None
}

fn channel(input: &[u8], key: [u8; 128 / 8]) -> Vec<u8> {
    // Secret suffix, we will decrypt it by attacking ECB's inherent weakness
    let suffix: Vec<u8> = Vec::from_base64("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK");
    let data: Vec<u8> = Vec::from_iter(input.iter().chain(suffix.iter()).copied());
    aes_128_ecb_encrypt_vec(data, key)
}

#[test]
fn chal_2_12() {
    assert_eq!(1, 1);
}
