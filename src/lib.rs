use itertools::Itertools;
use std::iter::zip;

pub mod raw;

/// Calculates the Hamming Distance between two raw strings
pub fn hamming(a: &[u8], b: &[u8]) -> usize {
    zip(a, b)
        .map(|(aa, bb)| aa ^ bb)
        .map(|x| {
            (0..8)
                .map(|i| if x & (1 << i) > 0 { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

/// Goes great with split pea soup, mmmmm
pub fn ham_chunks(data: &Vec<u8>, k_len: usize, chunk_i: usize) -> usize {
    hamming(
        &data[chunk_i * k_len..(chunk_i + 1) * k_len],
        &data[(chunk_i + 1) * k_len..(chunk_i + 2) * k_len],
    ) * 100
        / k_len
}

#[test]
fn unmistakable_hamming() {
    let a = Vec::from("this is a test".as_bytes());
    let b = Vec::from("wokka wokka!!!".as_bytes());
    let hd = hamming(&a, &b);
    assert_eq!(hd, 37);
}

// Mapping from utf8 codepoint to character frequency score.
// generated in /tools/generate_char_freq.py
static SCORES_EN_US: [usize; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29070, 0, 0, 10000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 277618, 4417, 3303, 3777, 59, 77, 210, 10975, 1224, 1146, 3944, 269, 12436, 4318,
    23840, 655, 1201, 2795, 2204, 463, 341, 437, 284, 202, 295, 417, 14400, 314, 108, 101, 315,
    4743, 61, 5597, 3547, 4570, 3489, 3478, 3140, 5467, 3455, 11644, 1370, 916, 3654, 3785, 3650,
    3237, 2913, 419, 4004, 5650, 6461, 1999, 858, 4721, 333, 3129, 173, 1305, 30, 1300, 3, 152, 12,
    93396, 21681, 32506, 45379, 138374, 21620, 30871, 54799, 84862, 1719, 19336, 57855, 29504,
    87914, 107507, 23718, 929, 72815, 74159, 103030, 41444, 10078, 26158, 4687, 29411, 1789, 8, 59,
    4, 13, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 4, 5, 5, 0, 0, 1, 0, 1, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 1, 0, 0, 5, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
    2, 0, 0, 0,
];

/// Returns the average score per character of a raw string
fn score_text(text: &Vec<u8>) -> usize {
    text.iter()
        .map(|x| SCORES_EN_US[*x as usize])
        .sum::<usize>()
        / text.len()
}

/// SBX - Single Byte XOR
/// A message decrypted using the held `key` byte and with a `score` relating to
/// its likelihood of being valid English text.
pub struct SBX {
    pub key: u8,
    pub plaintext: Vec<u8>,
    pub score: usize,
}

impl SBX {
    /// Use a given `key` to decrypt a given `ciphertext`. The `plaintext` and
    /// `score` are calculated during instantiation.
    pub fn new(key: u8, ciphertext: &Vec<u8>) -> SBX {
        let plaintext = ciphertext.iter().map(|b| b ^ key).collect();
        SBX {
            key,
            score: score_text(&plaintext),
            plaintext,
        }
    }

    /// Returns a new SBX which has the highest `score` out of all possible `key`'s
    pub fn from_ciphertext(ciphertext: &Vec<u8>) -> SBX {
        (0..=255)
            .map(|key| SBX::new(key, ciphertext))
            .sorted_by_key(|trial| trial.score) // Sort by score
            .rev()
            .next()
            .unwrap() // Return the tuple with the highest score
    }
}

/// RBX - Repeating Byte XOR
pub struct RBX {
    pub chunk_info: ChunkCoherence,
    pub key: Vec<u8>,
    pub plaintext: Vec<u8>,
}

impl RBX {
    pub fn from_ciphertext(ciphertext: &Vec<u8>, max_k_len: usize) -> RBX {
        // Find the key length with the lowest hamming score between consecutive key length chunks
        let chunk_info = (1..max_k_len)
            .map(|ks| ChunkCoherence::new(ks, &ciphertext))
            .sorted_by_key(|trail| trail.ham_score)
            .next()
            .unwrap();

        // Find the key by performing SingleByteXor on each subset of the ciphertext that corresponds to the same key byte
        let key: Vec<u8> = (0..chunk_info.k_len)
            .map(|i| {
                SBX::from_ciphertext(
                    &ciphertext
                        .iter()
                        .skip(i)
                        .step_by(chunk_info.k_len)
                        .copied()
                        .collect::<Vec<u8>>(),
                )
                .key
            })
            .collect();

        // Convert the ciphertext by xor'ing the key
        let plaintext = ciphertext
                .iter()
                .zip(key.iter().cycle())
                .map(|(a, b)| a ^ b)
                .collect();

        RBX {
            chunk_info,
            key,
            plaintext,
        }
    }
}

pub struct ChunkCoherence {
    pub k_len: usize,
    pub ham_score: usize,
}

impl ChunkCoherence {
    pub fn new(k_len: usize, ciphertext: &Vec<u8>) -> ChunkCoherence {
        let n_chunks = ciphertext.len() / k_len - 1;
        ChunkCoherence {
            k_len,
            ham_score: (0..n_chunks)
                .map(|i| ham_chunks(&ciphertext, k_len, i))
                .sum::<usize>()
                * 100
                / n_chunks,
        }
    }
}
