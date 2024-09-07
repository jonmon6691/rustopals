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
    let mut ret: Vec<Vec<u8>> = data
        .chunks(k_len)
        .into_iter()
        .map(|chunk| pkcs7_padder(chunk, k_len))
        .collect();
    if data.len() % k_len == 0 {
        ret.push(Vec::from_iter(itertools::repeat_n(k_len as u8, k_len)));
    }
    ret
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

/// Passes longer and longer strings to the black box encrypter until its size jumps. The size of the jump is assumed to be the block length
pub fn detect_ecb_blocksize(
    blackbox: impl Fn(&[u8]) -> Vec<u8>,
    max_bytes: usize,
) -> Option<usize> {
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
