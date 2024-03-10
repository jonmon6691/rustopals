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
static SCORES_EN_US: [usize; 256] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 13581, 89329, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105892, 1597, 0, 0, 23258, 147, 166, 46578, 4890, 4952, 7157, 2804, 117318, 61236, 116832, 1161428, 4458, 5182, 2621, 1732, 1452, 2144, 1451, 1065, 1265, 2125, 3705, 5566, 0, 0, 0, 4694, 0, 11385, 6527, 7776, 4080, 3166, 4263, 3444, 8015, 12543, 3008, 1494, 3252, 7455, 3798, 3267, 5162, 241, 3663, 10322, 15568, 1640, 1055, 6003, 56, 1610, 122, 2, 0, 2, 0, 0, 35348, 484308, 267985, 253541, 285539, 631440, 107635, 108321, 269240, 459060, 149004, 29685, 241284, 127694, 996881, 389663, 285497, 14052, 345806, 439101, 589193, 127798, 174571, 99300, 12493, 80164, 36105, 16, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

// Returns the average score per character of a raw string
fn score_utf8(cps: &Vec<u8>) -> usize {
    let mut score = 0;
    for a in cps  {
        score += SCORES_EN_US[*a as usize];
    }
    score / cps.len()
}
