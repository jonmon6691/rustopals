use std::iter::zip;

use itertools::Itertools;

/// Base64 character lookup table
static B64_ENC: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
    'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a',
    'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
    'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6',
    '7', '8', '9', '+', '/'];

static B64_DEC: [Option<u8>; 255] = [None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    Some(62), None, None, None, Some(63), Some(52), Some(53), Some(54),
    Some(55), Some(56), Some(57), Some(58), Some(59), Some(60), Some(61), None,
    None, None, None, None, None, None, Some(0), Some(1), Some(2), Some(3),
    Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11),
    Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18),
    Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), None,
    None, None, None, None, None, Some(26), Some(27), Some(28), Some(29),
    Some(30), Some(31), Some(32), Some(33), Some(34), Some(35), Some(36),
    Some(37), Some(38), Some(39), Some(40), Some(41), Some(42), Some(43),
    Some(44), Some(45), Some(46), Some(47), Some(48), Some(49), Some(50),
    Some(51), None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None];

pub trait EverythingRemainsRaw {
    fn from_hex(data: &str) -> Vec<u8>;
    fn into_hex(self) -> String;
    fn into_base64(self) -> String;
    fn from_base64(data: &str) -> Vec<u8>;
}

impl EverythingRemainsRaw for Vec<u8> {
    /// Loops through `data` and returns a `u8` for each consecutive pair of `char`'s
    /// interpreted as a hexadecimal byte. This is the inverse of `hex_encode`
    /// 
    /// If the input length is odd, then the trailing nibble is ignored and no `u8`
    /// is emitted for it.
    /// 
    /// # Panics
    /// Panics if a character in `data` is not valid hex
    fn from_hex(data: &str) -> Vec<u8> {
        data.chars().tuples()
            .map(|(hi, low)|
                (hi.to_digit(16).unwrap() * 16 +
                low.to_digit(16).unwrap()) as u8
            ).collect()
    }

    /// Converts the given `data` into a String hexadecimal representation. This is
    /// the inverse function of `hex_decode`.
    fn into_hex(self) -> String {
        self.iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }

    /// Converts `data` into a `String` containing the base64 representation of the
    /// raw data.
    fn into_base64(self) -> String {
        self.into_iter().triples().map(|(a, b, c)| {
            vec![
                B64_ENC[(a >> 2) as usize],
                B64_ENC[((a & 0x3) << 4 | b.unwrap_or(0) >> 4) as usize],
                b.map_or('=', |x| B64_ENC[((x & 0xf) << 2 | c.unwrap_or(0) >> 6) as usize]),
                c.map_or('=', |x| B64_ENC[(x & 0x3f) as usize])]
        }).flatten().collect()
    }

    fn from_base64(data: &str) -> Vec<u8> {
        data.as_bytes().iter()
            .map(|c: &u8| B64_DEC[*c as usize])
            .tuples().map(|chunk| {
                match chunk {
                    (None, _, _, _) => panic!("Invalid base64 length"),
                    (Some(_), None, _, _) => panic!("Invalid base64 length"),
                    (Some(a), Some(b), None, _) => {
                        assert_eq!(b & 0b00001111,  0, "Invalid base64 char at end");
                        vec![a << 2 | b >> 4]
                    },
                    (Some(a), Some(b), Some(c), None) => {
                        assert_eq!(c & 0b00000011, 0, "Invalid base64 char at end");
                        vec![
                            a << 2 | b >> 4,
                            b << 4 | c >> 2
                        ]
                    }
                    (Some(a), Some(b), Some(c), Some(d)) => {
                        vec![
                            a << 2 | b >> 4,
                            b << 4 | c >> 2,
                            c << 6 | d
                        ]
                    }
                }
            }).flatten().collect()
    }
}

#[test]
fn b64_round_trip_test() {
    let v_list = vec![
        vec![1],
        vec![1,2],
        vec![1,2,3],
        vec![1,2,3,4],
        vec![1,2,3,4,5],
        vec![1,2,3,4,5,6]
    ];
    for v in v_list { 
        let b64 = v.clone().into_base64();
        println!("{}", &b64);
        assert!(b64.len() > 0);
        let vp = Vec::from_base64(&b64);
        assert_eq!(v, vp);
    }
}

/// Define all iterators as having an implementation of TriplesIterator
/// This is known as an "iterator adapter" and is what lets allows triples()
/// to be called as a method of all existing iterators
impl <I: Iterator> TriplesIterator for I {}
trait TriplesIterator: Iterator {
    fn triples(self) -> Triples<Self> where Self: Sized { Triples { iter: self } }
}

/// Triples iterator returns three `Option<u8>`'s of the iterator at a time.
/// Including a final pass with any remaining elements passed as normal, and
/// None values for the remaining portion.
/// 
/// # Example
/// ```ignore
/// let a: Vec<u8> = vec![1, 2, 3, 4, 5].into_iter().triples();
/// 
/// assert_eq!(Some((1, Some(2), Some(3))), a.next());
/// assert_eq!(Some((4, Some(5), None)), a.next());
/// assert_eq!(None, a.next());
/// ```
struct Triples<I> { iter: I }
impl <I: Iterator<Item = u8>> Iterator for Triples<I> {
    type Item = (u8, Option<u8>, Option<u8>);
    fn next(&mut self) -> Option<Self::Item> {
        match (self.iter.next(), self.iter.next(), self.iter.next()) {
            (None, _, _) => None,
            (Some(first), second, third) => Some((first, second, third))
        }
    }
}

/// Calculates the Hamming Distance between two raw strings
pub fn hamming(a: &Vec<u8>, b: &Vec<u8>) -> usize {
    zip(a, b)
        .map(|(aa, bb)| aa ^ bb)
        .map(|x| 
            (0..8).map(|i| if x & (1 << i) > 0 {1} else {0}).sum::<usize>())
        .sum()
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
static SCORES_EN_US: [usize; 256] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29070, 0, 0,
    10000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 277618, 4417,
    3303, 3777, 59, 77, 210, 10975, 1224, 1146, 3944, 269, 12436, 4318, 23840,
    655, 1201, 2795, 2204, 463, 341, 437, 284, 202, 295, 417, 14400, 314, 108,
    101, 315, 4743, 61, 5597, 3547, 4570, 3489, 3478, 3140, 5467, 3455, 11644,
    1370, 916, 3654, 3785, 3650, 3237, 2913, 419, 4004, 5650, 6461, 1999, 858,
    4721, 333, 3129, 173, 1305, 30, 1300, 3, 152, 12, 93396, 21681, 32506, 45379,
    138374, 21620, 30871, 54799, 84862, 1719, 19336, 57855, 29504, 87914, 107507,
    23718, 929, 72815, 74159, 103030, 41444, 10078, 26158, 4687, 29411, 1789, 8,
    59, 4, 13, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 4, 5, 5,
    0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 5,
    0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 2,
    0, 0, 0];

/// Returns the average score per character of a raw string
fn score_text(text: &Vec<u8>) -> usize {
    text.iter()
        .map(|x| {SCORES_EN_US[*x as usize]})
        .sum::<usize>() / text.len()
}

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
        (0..=255).map(|key| SBX::new(key, ciphertext))
            .sorted_by_key(|trial| trial.score) // Sort by score
            .rev().next().unwrap() // Return the tuple with the highest score
    }

    /// Attempt to convert the decrypted message to a `String` otherwise return a `String`
    /// containing an error message.
    /// 
    /// Note: the decrypted string will always be returned with
    /// the prefix `"Plaintext:" `to avoid confusing a plaintext which itself looks like the
    /// error message string.
    pub fn to_string(self) -> String {
        format!("Plaintext: {}", String::from_utf8(self.plaintext.clone())
            .unwrap_or("Failure: [Error Decoding UTF-8]".to_string()))
    }
}
