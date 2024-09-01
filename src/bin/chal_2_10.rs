use aes::Aes128;
use cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use rustopals::raw::EverythingRemainsRaw;
/// Encrypts and decrypts in AES-128-CBC mode
use std::fs;

fn main() {
    let ct = Vec::from_base64(
        &fs::read_to_string("test_data/10.txt").expect("Error reading input file"),
    );
    let iv = Vec::from([0u8; 16]);
    let key = "YELLOW SUBMARINE".to_owned();
    let mut pt: Vec<u8> = Vec::new();

    // Decrypt
    ct.chunks_exact(16).fold(iv.clone(), |iv, ct_chunk| {
        pt.extend(aes_128_cbc_decrypt_block(ct_chunk, key.as_bytes(), iv).iter());
        Vec::from(ct_chunk)
    });

    println!("{}", pt.clone().to_string());

    // Re-encrypt
    let mut ct_check: Vec<u8> = Vec::new();
    pt.chunks_exact(16).fold(iv.clone(), |iv, pt_chunk| {
        let ct = aes_128_cbc_encrypt_block(pt_chunk, key.as_bytes(), iv);
        ct_check.extend(ct.iter());
        ct
    });

    // Check round trip
    assert_eq!(ct, ct_check);
}

fn aes_128_cbc_encrypt_block(block: &[u8], key: &[u8], iv: Vec<u8>) -> Vec<u8> {
    let mut block = GenericArray::from_iter(block.iter().zip(iv.iter()).map(|(a, b)| a ^ b));
    Aes128::new(GenericArray::from_slice(key)).encrypt_block(&mut block);
    block.into_iter().collect()
}

fn aes_128_cbc_decrypt_block(block: &[u8], key: &[u8], iv: Vec<u8>) -> Vec<u8> {
    let mut block = GenericArray::from_iter(block.iter().copied());
    Aes128::new(GenericArray::from_slice(key)).decrypt_block(&mut block);
    block.iter().zip(iv.iter()).map(|(a, b)| *a ^ *b).collect()
}

#[test]
fn chal_2_10() {
    let ct = Vec::from_base64(
        &fs::read_to_string("test_data/10.txt").expect("Error reading input file"),
    );
    let iv = Vec::from([0u8; 16]);
    let key = "YELLOW SUBMARINE".to_owned();
    let mut pt: Vec<u8> = Vec::new();

    // Decrypt
    ct.chunks_exact(16).fold(iv.clone(), |iv, ct_chunk| {
        pt.extend(aes_128_cbc_decrypt_block(ct_chunk, key.as_bytes(), iv).iter());
        Vec::from(ct_chunk)
    });

    // Check plaintext sum
    let sum = pt.iter().fold(0 as usize, |a, &b| a + b as usize);
    assert_eq!(sum, 247170);

    // Re-encrypt
    let mut ct_check: Vec<u8> = Vec::new();
    pt.chunks_exact(16).fold(iv.clone(), |iv, pt_chunk| {
        let ct = aes_128_cbc_encrypt_block(pt_chunk, key.as_bytes(), iv);
        ct_check.extend(ct.iter());
        ct
    });

    // Check round trip
    assert_eq!(ct, ct_check);
}
