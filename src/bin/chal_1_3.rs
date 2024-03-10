use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    println!("sets/1/challenges/3 - Single-byte XOR cipher");

    let ct_bytes = rustopals::hex_decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();
    
    // Holds the results of the cracking algo, key->(score, plaintext)
    let mut scores: HashMap<u8, (usize, Result<String,_>)> = HashMap::new();

    // Try every possible key
    for key_byte in 0..=255 {
        let mut pt_bytes: Vec<u8> = Vec::with_capacity(ct_bytes.len());
        
        // Decrypt
        for a in ct_bytes.iter() {
            pt_bytes.push(a ^ key_byte);
        }

        // Calculate score and store in the hashmap
        let score = score_utf8(&pt_bytes);
        scores.insert(key_byte, (score, String::from_utf8(pt_bytes)));
    }

    // Get the highest score
    let i =  scores.iter().sorted_by_key(|x| x.1.0).rev().next().unwrap();

    //Print the results
    println!("Key: dec:{} hex:{:x} ascii:{}", i.0, i.0, *i.0 as char);
    println!("Plaintext: {}", match &i.1.1 {Ok(s) => s, Err(_) => "error"});
}

// Mapping from utf8 codepoint to character frequency score.
// generated in /tools/generate_char_freq.py
static SCORES_EN_US: [usize; 256] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29070, 0, 0, 10000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 277618, 4417, 3303, 3777, 59, 77, 210, 10975, 1224, 1146, 3944, 269, 12436, 4318, 23840, 655, 1201, 2795, 2204, 463, 341, 437, 284, 202, 295, 417, 14400, 314, 108, 101, 315, 4743, 61, 5597, 3547, 4570, 3489, 3478, 3140, 5467, 3455, 11644, 1370, 916, 3654, 3785, 3650, 3237, 2913, 419, 4004, 5650, 6461, 1999, 858, 4721, 333, 3129, 173, 1305, 30, 1300, 3, 152, 12, 93396, 21681, 32506, 45379, 138374, 21620, 30871, 54799, 84862, 1719, 19336, 57855, 29504, 87914, 107507, 23718, 929, 72815, 74159, 103030, 41444, 10078, 26158, 4687, 29411, 1789, 8, 59, 4, 13, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 4, 5, 5, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 5, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0];

// Returns the average score per character of a raw string
fn score_utf8(cps: &Vec<u8>) -> usize {
    let mut score = 0;
    for a in cps  {
        score += SCORES_EN_US[*a as usize];
    }
    score / cps.len()
}
