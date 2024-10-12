/// Block based stuff
use super::hamming;
use crate::raw::EverythingRemainsRaw;
use aes::{
    cipher::{
        block_padding::Pkcs7, generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit,
    },
    Aes128,
};
use itertools::{repeat_n, Itertools};
use rand::{rngs::ThreadRng, Rng};
use std::iter;

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

pub fn print_blocks(data: &[u8], block_len: usize) {
    for c in data.chunks(block_len) {
        let mut token = Vec::from(c).into_base64();
        token.truncate(3);
        print!("{} ", token);
    }
    print!("\n");
}

// Returns a function that emulates an unknown ECB channel
pub fn make_ecb_channel<'a>(
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

// Returns a function that emulates an unknown ECB channel with a random length of random bytes as a prefix
pub fn make_randoprefix_ecb_channel<'a>(
    mut prefix_rng: ThreadRng,
    max_prefix: usize,
    suffix: &'a [u8],
    key: [u8; 128 / 8],
) -> impl FnMut(&[u8]) -> Vec<u8> + 'a {
    move |input: &[u8]| -> Vec<u8> {
        // Make a range of random length, then map it to random bytes.
        let prefix =
            (0..prefix_rng.gen_range::<usize, _>(0..max_prefix)).map(|_| prefix_rng.gen::<u8>());
        aes_128_ecb_encrypt_vec(
            Vec::from_iter(
                prefix
                    .chain(input.iter().cloned())
                    .chain(suffix.iter().cloned()),
            ),
            key,
        )
    }
}

/// Decrypts an unknown suffix appended to user controlled input into an ECB
/// encryption system. Can't work for prefixes because I can't control where
/// in the block the data appears like I can for suffixes.
pub fn crack_randoprefix_ecb_suffix(blackbox: impl FnMut(&[u8]) -> Vec<u8>) -> Option<Vec<u8>> {
    // Determine the block size of the cipher
    // let block_size =
        // detect_ecb_blocksize(&blackbox, 512).expect("Couldn't determine cipher block size");

    // Finds pair of repeated blocks caused by our input, this tells us when the suffix is aligned on a block boundary and where it is.
    // TODO: This can probably be optimized by noticing when a changing block freezes, instead of waiting for 2 identical blocks to appear.
    // let (n_pad_until_suffix_aligned, start_of_repeated_blocks) = (2 * block_size..3 * block_size)
        // .map(|i| find_repeat_blocks(&blackbox(&Vec::from_iter(repeat_n(0x61, i))), block_size))
        // .find_position(|trial| trial.is_some())
        // .expect("Couldn't find duplicate blocks, are you sure this is ECB?");
    // let start_of_repeated_blocks = start_of_repeated_blocks
        // .expect("find_position only returns if is_some() is true, therefore this check is theoretically infallible. If you're reading this, good luck.");

    // // Find out how much of the suffix is in the padding block
    // let n_pad_until_suffix_overflow = (n_pad_until_suffix_aligned
        // ..=n_pad_until_suffix_aligned + block_size)
        // .map(|probe_length| blackbox(&Vec::from_iter(repeat_n(0x61, probe_length))).len())
        // .tuple_windows()
        // .enumerate()
        // .filter_map(|(i, (prev, next))| if next > prev { Some(i) } else { None })
        // .next()
        // .expect("Cipher should have grown by one block length");

    // let _len_prefix = start_of_repeated_blocks - n_pad_until_suffix_aligned;
    // let ct_len = blackbox(&Vec::from_iter(repeat_n(0x61, n_pad_until_suffix_aligned))).len();
    // let len_suffix = ct_len - start_of_repeated_blocks - n_pad_until_suffix_overflow - 1;

    // // Break AES in ECB mode
    // let mut pt: Vec<u8> = Vec::with_capacity(len_suffix);
    // for i in 0..len_suffix {
        // let block_offset = block_size * (i / block_size);
        // // Shift in the portion of the siffix we've already cracked plus one
        // // more byte. Then encrypt it. Then pull out the block which is all
        // // known except for that last byte
        // let ct_target = &blackbox(&Vec::from_iter(repeat_n(
            // 0x61,
            // block_offset + n_pad_until_suffix_aligned + block_size - 1 - i,
        // )))[block_offset + start_of_repeated_blocks..][..block_size];
        // // Loop through every possible value of the last byte and compare to
        // // the ct_target block
        // pt.push(
            // (0..=255)
                // .filter_map(|check_byte| {
                    // let probe = Vec::from_iter(
                        // repeat_n(
                            // 0x61,
                            // block_offset + n_pad_until_suffix_aligned + block_size - 1 - i,
                        // )
                        // .chain(pt.iter().copied())
                        // .chain(iter::once(check_byte)),
                    // );
                    // if blackbox(&probe)[block_offset + start_of_repeated_blocks..][..block_size]
                        // == *ct_target
                    // {
                        // Some(check_byte)
                    // } else {
                        // None
                    // }
                // })
                // .next()
                // .expect("There should be some byte that results in a matching block"),
        // );
    // }
    // Some(pt)
    Some(vec![])
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

/// Decrypts an unknown suffix appended to user controlled input into an ECB
/// encryption system. Can't work for prefixes because I can't control where
/// in the block the data appears like I can for suffixes.
pub fn crack_ecb_suffix(blackbox: impl Fn(&[u8]) -> Vec<u8>) -> Option<Vec<u8>> {
    // Determine the block size of the cipher
    let block_size =
        detect_ecb_blocksize(&blackbox, 512).expect("Couldn't determine cipher block size");

    // Finds pair of repeated blocks caused by our input, this tells us when the suffix is aligned on a block boundary and where it is.
    // TODO: This can probably be optimized by noticing when a changing block freezes, instead of waiting for 2 identical blocks to appear.
    let (n_pad_until_suffix_aligned, start_of_repeated_blocks) = (2 * block_size..3 * block_size)
        .map(|i| find_repeat_blocks(&blackbox(&Vec::from_iter(repeat_n(0x61, i))), block_size))
        .find_position(|trial| trial.is_some())
        .expect("Couldn't find duplicate blocks, are you sure this is ECB?");
    let start_of_repeated_blocks = start_of_repeated_blocks
        .expect("find_position only returns if is_some() is true, therefore this check is theoretically infallible. If you're reading this, good luck.");

    // Find out how much of the suffix is in the padding block
    let n_pad_until_suffix_overflow = (n_pad_until_suffix_aligned
        ..=n_pad_until_suffix_aligned + block_size)
        .map(|probe_length| blackbox(&Vec::from_iter(repeat_n(0x61, probe_length))).len())
        .tuple_windows()
        .enumerate()
        .filter_map(|(i, (prev, next))| if next > prev { Some(i) } else { None })
        .next()
        .expect("Cipher should have grown by one block length");

    let _len_prefix = start_of_repeated_blocks - n_pad_until_suffix_aligned;
    let ct_len = blackbox(&Vec::from_iter(repeat_n(0x61, n_pad_until_suffix_aligned))).len();
    let len_suffix = ct_len - start_of_repeated_blocks - n_pad_until_suffix_overflow - 1;

    // Break AES in ECB mode
    let mut pt: Vec<u8> = Vec::with_capacity(len_suffix);
    for i in 0..len_suffix {
        let block_offset = block_size * (i / block_size);
        // Shift in the portion of the siffix we've already cracked plus one
        // more byte. Then encrypt it. Then pull out the block which is all
        // known except for that last byte
        let ct_target = &blackbox(&Vec::from_iter(repeat_n(
            0x61,
            block_offset + n_pad_until_suffix_aligned + block_size - 1 - i,
        )))[block_offset + start_of_repeated_blocks..][..block_size];
        // Loop through every possible value of the last byte and compare to
        // the ct_target block
        pt.push(
            (0..=255)
                .filter_map(|check_byte| {
                    let probe = Vec::from_iter(
                        repeat_n(
                            0x61,
                            block_offset + n_pad_until_suffix_aligned + block_size - 1 - i,
                        )
                        .chain(pt.iter().copied())
                        .chain(iter::once(check_byte)),
                    );
                    if blackbox(&probe)[block_offset + start_of_repeated_blocks..][..block_size]
                        == *ct_target
                    {
                        Some(check_byte)
                    } else {
                        None
                    }
                })
                .next()
                .expect("There should be some byte that results in a matching block"),
        );
    }
    Some(pt)
}

/// Passes longer and longer strings to the black box encrypter until its size jumps. The size of the jump is assumed to be the block length
pub fn detect_ecb_blocksize(
    blackbox: impl Fn(&[u8]) -> Vec<u8>,
    max_bytes: usize,
) -> Option<usize> {
    (0..max_bytes)
        .map(|probe_length| blackbox(&Vec::from_iter(repeat_n(0x61, probe_length))).len())
        .tuple_windows()
        .filter_map(|(prev, next)| if next > prev { Some(next - prev) } else { None })
        .next()
}

/// Returns None if there are no consecutive blocks which match, otherwise
/// returns the index of the first block in the matching consecutive pair
pub fn find_repeat_blocks(data: &[u8], block_len: usize) -> Option<usize> {
    let n_chunks = data.len() / block_len - 1;
    for i in 0..n_chunks {
        if hamming(
            &data[(i + 0) * block_len..(i + 1) * block_len],
            &data[(i + 1) * block_len..(i + 2) * block_len],
        ) == 0
        {
            return Some(i * block_len);
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

pub fn aes_128_ecb_decrypt_vec(ct: Vec<u8>, key: [u8; 16]) -> Vec<u8> {
    let mut raw_input = Vec::from(ct);
    Aes128::new(&GenericArray::from_slice(&key))
        .decrypt_padded::<Pkcs7>(&mut raw_input)
        .expect("Padding error in decrypted message!")
        .to_owned()
}

fn aes_128_cbc_encrypt_block(block: &[u8], key: &[u8], iv: &[u8]) -> [u8; 16] {
    let mut block = GenericArray::from_iter(block.iter().zip(iv.iter()).map(|(a, b)| a ^ b));
    Aes128::new(GenericArray::from_slice(key)).encrypt_block(&mut block);
    block.into()
}
