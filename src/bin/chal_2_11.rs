use rand::{thread_rng, Rng};
use rustopals::blocky::{aes_128_cbc_encrypt_vec, aes_128_ecb_encrypt_vec, detect_ecb};
use std::io::Read;

fn main() {
    let mut rng = thread_rng();
    let n = 100;
    let mut n_ecb = 0;
    let mut n_cbc = 0;

    for _ in 0..n {
        let use_ecb = rng.gen_bool(0.5);
        match aes_mode_detector(|input| channel(input, use_ecb)) {
            AESMode::AESECB => n_ecb += 1,
            AESMode::AESCBC => n_cbc += 1,
        }
    }
    println!("ECB: {n_ecb}/{n}");
    println!("CBC: {n_cbc}/{n}");
}

#[derive(Debug, PartialEq)]
enum AESMode {
    AESECB,
    AESCBC,
}

fn aes_mode_detector(blackbox: impl Fn(&[u8]) -> Vec<u8>) -> AESMode {
    // Use 3 blocks worth of constant data, this forces at least one
    // consecutive pair of blocks to be identical in ECB mode
    let input = [0u8; 128 / 8 * 3];
    let ct = blackbox(&input);

    if detect_ecb(&ct, 128 / 8) {
        AESMode::AESECB
    } else {
        AESMode::AESCBC
    }
}

fn channel(input: &[u8], use_ecb: bool) -> Vec<u8> {
    let mut rng = thread_rng();
    // Random key
    let key: [u8; 16] = rng.gen();

    // Random length prefix of random bytes
    let mut prefix: Vec<u8> = Vec::new();
    for _ in 0..rng.gen_range(5..=10) {
        prefix.push(rng.gen());
    }

    // Random length suffix of random bytes
    let mut suffix: Vec<u8> = Vec::new();
    for _ in 0..rng.gen_range(5..=10) {
        suffix.push(rng.gen());
    }

    // TODO: Surely this is the worst possible way to do this
    let data: Vec<u8> = prefix
        .chain(input)
        .chain(suffix.as_slice())
        .bytes()
        .map(|x| x.unwrap())
        .collect();

    if use_ecb {
        // Heads, do EBC
        aes_128_ecb_encrypt_vec(data, key)
    } else {
        // Tails, do CBC, with a random iv
        let iv: [u8; 16] = rng.gen();
        aes_128_cbc_encrypt_vec(data, key, iv)
    }
}

#[test]
fn chal_2_11() {
    assert_eq!(AESMode::AESECB, aes_mode_detector(|i| channel(i, true)));
    assert_eq!(AESMode::AESCBC, aes_mode_detector(|i| channel(i, false)));
}
