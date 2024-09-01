/// Block based stuff
use super::hamming;
use aes::Aes128;
use cipher::generic_array::GenericArray;
use cipher::{BlockEncrypt, KeyInit};

/// Takes a Vec<u8> of data of any length, returns a vector of chunks of size k_len, with the last chunk padded using PKCS7
///
/// # Panics
/// if k_len is greater than 255
pub fn pkcs7_chunker(data: Vec<u8>, k_len: usize) -> Vec<Vec<u8>> {
    assert!(k_len <= 255);
    data.chunks(k_len)
        .into_iter()
        .map(|chunk| pkcs7_padder(chunk, k_len))
        .collect()
}

pub fn pkcs7_padder(chunk: &[u8], k_len: usize) -> Vec<u8> {
    match chunk.len().cmp(&k_len) {
        std::cmp::Ordering::Less => {
            let padding = k_len - chunk.len();
            chunk
                .iter()
                .chain(itertools::repeat_n(&(padding as u8), padding))
                .copied()
                .collect()
        } // Do padding
        std::cmp::Ordering::Equal => Vec::from(chunk),
        std::cmp::Ordering::Greater => panic!("Data to pad is larger than desired block length!"),
    }
}

/// Returns true if there are any two k_len chunks in data that are repeated
/// Only searches chunks alinged by k_len
pub fn detect_ecb(data: &[u8], k_len: usize) -> bool {
    let n_chunks = data.len() / k_len - 1;

    (0..n_chunks)
        .map(|left| {
            (left + 1..n_chunks).map(move |right: usize| {
                hamming(
                    &data[left * k_len..(left + 1) * k_len],
                    &data[right * k_len..(right + 1) * k_len],
                )
            })
        })
        .flatten()
        .filter(|m| *m == 0)
        .count()
        > 0
}

pub fn aes_128_cbc_encrypt_vec(pt: Vec<u8>, key: [u8; 16], iv: [u8; 16]) -> Vec<u8> {
    let mut ct = Vec::new();
    pkcs7_chunker(pt, 128 / 8).iter().fold(iv, |iv, pt_chunk| {
        let block = aes_128_cbc_encrypt_block(pt_chunk, &key, &iv);
        ct.extend(block.iter());
        block
    });
    ct
}

pub fn aes_128_ecb_encrypt_vec(pt: Vec<u8>, key: [u8; 16]) -> Vec<u8> {
    let mut ct = Vec::new();
    for block in pkcs7_chunker(pt, 128 / 8) {
        let mut block = GenericArray::from_slice(&block).to_owned();
        Aes128::new(GenericArray::from_slice(&key)).encrypt_block(&mut block);
        ct.extend(block.iter());
    }
    ct
}

fn aes_128_cbc_encrypt_block(block: &[u8], key: &[u8], iv: &[u8]) -> [u8; 16] {
    let mut block = GenericArray::from_iter(block.iter().zip(iv.iter()).map(|(a, b)| a ^ b));
    Aes128::new(GenericArray::from_slice(key)).encrypt_block(&mut block);
    block.into()
}
