use std::fs;
use aes::{cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit}, Aes128};
use cipher::block_padding::Pkcs7;
use itertools::Itertools;
use rustopals::raw::EverythingRemainsRaw;

fn do_chal() -> Vec<u8> {
    // Load the file, remove newlines to make one long line of b64 and decode
    let mut raw_input = Vec::from_base64(
        &fs::read_to_string("test_data/7.txt")
            .expect("Error reading file")
            .split_ascii_whitespace()
            .join("")
    );
    let key = "YELLOW SUBMARINE".as_bytes();

    Aes128::new(&GenericArray::from_slice(&key))
        .decrypt_padded::<Pkcs7>(&mut raw_input)
        .expect("Padding error in decrypted message!")
        .to_owned()
}

fn main() {
    println!("Chal 7!");
    
    println!("{}", do_chal().to_string());
}

#[test]
fn chal_1_7() {
    use itertools::fold;

    let pt = do_chal();

    // Check sum
    let sum = fold(pt.iter(), 0 as usize, |a, &b| a + b as usize);
    assert_eq!(sum, 247154);
}
