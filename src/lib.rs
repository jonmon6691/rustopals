use itertools::Itertools;

pub fn hex_decode(data: &str) -> Result<Vec<u8>, hex::FromHexError>
{
    let mut out: Vec<u8> = Vec::with_capacity(data.len()/2);
    for (hi, low) in data.chars().tuples() {
        let num = hi.to_digit(16).unwrap() * 16 + low.to_digit(16).unwrap();
        out.push(num as u8);
    }
    Ok(out)
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
pub fn score_text(text: &Vec<u8>) -> usize {
    return text.iter()
        .map(|x| {SCORES_EN_US[*x as usize]})
        .sum::<usize>() / text.len();
}

/// Encrypts/decrypts by xor'ing each u8 of `text` with `key`
pub fn single_byte_xor(text: &Vec<u8>, key: u8) -> Vec<u8>
{
    text.iter().map(|b| b ^ key).collect()
}

/// Uses an english language huristic language model to find the key over which ct was encrypted with.
/// 
/// Returns a tuple `(key: u8, score: usize)` which has the highest `score` over all `key`'s
pub fn crack_single_byte_xor(ct: &Vec<u8>) -> (u8, usize) {
    return (0..=255) // Try every possible key
        .map(|key|
            (key, score_text(&single_byte_xor(ct, key))) // Get the score
        )
        .sorted_by_key(|(_key, score)| *score) // Sort by score
        .rev().next().unwrap(); // Return the tuple with the highest score
}
