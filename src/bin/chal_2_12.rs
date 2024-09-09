use rand::{thread_rng, Rng};
use rustopals::{
    blocky::{aes_128_ecb_encrypt_vec, crack_ecb_suffix},
    raw::EverythingRemainsRaw,
};

fn main() {
    let mut rng = thread_rng();
    // Random key, generated here so it can be persistent throughout the attack
    let key: [u8; 16] = rng.gen();
    let suffix: Vec<u8> = Vec::from_base64("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK");
    let ch = make_channel(&[], &suffix, key);

    let pt_suffix = crack_ecb_suffix(ch)
        .expect("Couldn't crack cipher.")
        .to_string();

    println!("Suffix:\n{pt_suffix}");
}

// Returns a function that emulates an unknown ECB channel
fn make_channel<'a>(
    prefix: &'a [u8],
    suffix: &'a [u8],
    key: [u8; 128 / 8],
) -> impl Fn(&[u8]) -> Vec<u8> + 'a {
    move |input: &[u8]| -> Vec<u8> {
        aes_128_ecb_encrypt_vec(
            Vec::from_iter(
                prefix
                    .iter()
                    .chain(input.iter())
                    .chain(suffix.iter())
                    .copied(),
            ),
            key,
        )
    }
}

#[test]
fn chal_2_12() {
    let mut rng = thread_rng();
    // Random key, generated here so it can be persistent throughout the attack
    let key: [u8; 16] = rng.gen();
    let suffix: Vec<u8> = Vec::from_base64("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK");
    let ch = make_channel(&[], &suffix, key);

    let pt_suffix = crack_ecb_suffix(ch).expect("Couldn't crack cipher.");

    assert_eq!(suffix, pt_suffix);
}
