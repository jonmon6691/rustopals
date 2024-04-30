/// Block based stuff

use super::hamming;

/// Takes a Vec<u8> of data of any length, returns a vector of chunks of size k_len, with the last chunk padded using PKCS7
/// 
/// # Panics
/// Panics if k_len is greater than 255
pub fn pkcs7_chunker(data: Vec<u8>, k_len:usize) -> Vec<Vec<u8>> {
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
            chunk.iter()
                .chain(itertools::repeat_n(&(padding as u8), padding))
                .copied()
                .collect()
        }, // Do padding
        std::cmp::Ordering::Equal => Vec::from(chunk),
        std::cmp::Ordering::Greater => panic!("Data to pad is larger than desired block length!"),
    }
}

/// Returns true if there are any two k_len chunks in data that are repeated
/// Only searches chunks alinged by k_len
pub fn detect_ecb(data: &Vec<u8>, k_len: usize) -> bool {
    let n_chunks = data.len() / k_len - 1;

    (0..n_chunks).map(|left| {
        (left+1..n_chunks).map(move |right: usize| {
            hamming(
                &data[left  * k_len .. (left  + 1) * k_len],
                &data[right * k_len .. (right + 1) * k_len])
        })
    })
    .flatten()
    .filter(|m| *m == 0)
    .count() > 0
}
