use rand::{thread_rng, Rng};
use rustopals::{
    blocky::{crack_randoprefix_ecb_suffix, make_randoprefix_ecb_channel, print_blocks},
    raw::EverythingRemainsRaw,
};

fn main() {
    let mut rng = thread_rng();
    // Random key, generated here so it can be persistent throughout the attack
    let key: [u8; 16] = rng.gen();
    let suffix: Vec<u8> = Vec::from_base64("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK");
    let mut ch = make_randoprefix_ecb_channel(rng, 64, &suffix, key);

    // Lil check to see that the length is changing and that repeated blocks can still be seen
    let ct = ch(&[0; 64]);
    print_blocks(&ct, 16);
    let ct = ch(&[0; 64]);
    print_blocks(&ct, 16);

    let pt_suffix = crack_randoprefix_ecb_suffix(ch)
        .expect("Couldn't crack cipher.")
        .to_string();

    println!("Suffix:\n{pt_suffix}");
}

#[test]
fn chal_2_14() {
    let mut rng = thread_rng();
    // Random key, generated here so it can be persistent throughout the attack
    let key: [u8; 16] = rng.gen();
    let suffix: Vec<u8> = Vec::from_base64("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK");
    let ch = make_randoprefix_ecb_channel(rng, 10, &suffix, key);

    let pt_suffix = crack_randoprefix_ecb_suffix(ch).expect("Couldn't crack cipher.");

    assert_eq!(suffix, pt_suffix);
}
