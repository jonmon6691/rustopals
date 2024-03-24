use itertools::Itertools;

/// Loops through `data` and returns a `u8` for each consecutive pair of `char`'s
/// interpreted as a hexidecimal byte. This is the inverse of `hex_encode`
/// 
/// If the input length is odd, then the trailing nibble is ignored and no `u8`
/// is emitted for it.
/// 
/// # Panics
/// Panics if a character in `data` is not valid hex
pub fn hex_decode(data: &str) -> Vec<u8> {
    data.chars().tuples()
        .map(|(hi, low)|
            (hi.to_digit(16).unwrap() * 16 +
            low.to_digit(16).unwrap()) as u8
        ).collect()
}

pub fn hex_encode(data: Vec<u8>) -> String
{
    return data.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
}

const STRBASE64:&'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn base64_encode(data: Vec<u8>) -> String
{
    let mut out: Vec<char> = Vec::with_capacity(data.len());
    for (one, two, three) in data.iter().tuples() {
        let cp_one = one >> 2;
        let cp_two = ((one & 0x3) << 4) | (two >> 4);
        let cp_three = ((two & 0xf) << 2) | (three >> 6);
        let cp_four = three & 0x3f;
        out.push(STRBASE64.chars().nth(cp_one as usize).unwrap());
        out.push(STRBASE64.chars().nth(cp_two as usize).unwrap());
        out.push(STRBASE64.chars().nth(cp_three as usize).unwrap());
        out.push(STRBASE64.chars().nth(cp_four as usize).unwrap());
    }
    out.iter().collect()
}

// Mapping from utf8 codepoint to character frequency score.
// generated in /tools/generate_char_freq.py
static SCORES_EN_US: [usize; 256] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29070, 0, 0, 10000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 277618, 4417, 3303, 3777, 59, 77, 210, 10975, 1224, 1146, 3944, 269, 12436, 4318, 23840, 655, 1201, 2795, 2204, 463, 341, 437, 284, 202, 295, 417, 14400, 314, 108, 101, 315, 4743, 61, 5597, 3547, 4570, 3489, 3478, 3140, 5467, 3455, 11644, 1370, 916, 3654, 3785, 3650, 3237, 2913, 419, 4004, 5650, 6461, 1999, 858, 4721, 333, 3129, 173, 1305, 30, 1300, 3, 152, 12, 93396, 21681, 32506, 45379, 138374, 21620, 30871, 54799, 84862, 1719, 19336, 57855, 29504, 87914, 107507, 23718, 929, 72815, 74159, 103030, 41444, 10078, 26158, 4687, 29411, 1789, 8, 59, 4, 13, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 4, 5, 5, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 5, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0];

/// Returns the average score per character of a raw string
fn score_text(text: &Vec<u8>) -> usize {
    return text.iter()
        .map(|x| {SCORES_EN_US[*x as usize]})
        .sum::<usize>() / text.len();
}

pub struct SBX {
    pub key: u8,
    pub score: usize,
    pub plaintext: Vec<u8>,
}

impl SBX {
    pub fn new(key: u8, ciphertext: &Vec<u8>) -> SBX {
        let plaintext = ciphertext.iter().map(|b| b ^ key).collect();
        SBX {
            key,
            score: score_text(&plaintext),
            plaintext,
        }
    }

    pub fn from_ciphertext(ciphertext: &Vec<u8>) -> SBX {
        return (0..=255).map(|key| SBX::new(key, ciphertext))
            .sorted_by_key(|trial| trial.score) // Sort by score
            .rev().next().unwrap(); // Return the tuple with the highest score
    }

    pub fn to_string(self) -> String {
        format!("Plaintext: {}", String::from_utf8(self.plaintext.clone())
            .unwrap_or("Failure: [Error Decoding UTF-8]".to_string()))
    }
}
