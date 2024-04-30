/// Trying to crack the key to the detected ECB mode ciphertext from challenge 8
/// So far I've tried
/// * Every 16 character word (5k of them) key from https://github.com/dwyl/english-words/blob/master/words_alpha.txt
/// * every 2-word key ala "YELLOW SUBMARINE", both upper and lower case. (many many millions of them)
/// * that's it so far

use std::{collections::HashMap, fs, io};
use aes::{cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit}, Aes128};
use cipher::block_padding::Pkcs7;
use rustopals::raw::EverythingRemainsRaw;
use rayon::prelude::*;

fn main() {
    let ct = Vec::from_base64("2IBhl0CooZt4QKijHIEKPQhkmvcNwG9P1dLWnHRM0oPi3QUva2Qdv50RsDSFQrtXCGSa9w3Ab0/V0tacdEzSg5R1yd/bwdRll5SdnH6Cv1oIZJr3DcBvT9XS1px0TNKDl6k+q41q7NVmSJFUeJprAwhkmvcNwG9P1dLWnHRM0oPUAxgMmMj22x8qP5xAQN6wq1GymTPywSPFg4awb7oYag==");
    //let ct = Vec::from_base64(&fs::read_to_string("test_data/7.txt").expect("Error reading file")); // For sanity checking, helps to make a trivial dictionary file as well
    
    let mut words_by_len: HashMap<usize, Vec<String>> = HashMap::new();
    
    // Read the dictionary and load it into words_by_len... by length
    io::read_to_string(fs::File::open("test_data/words_alpha.txt")
        .expect("Error opening dictionary, get it from https://github.com/dwyl/english-words/blob/master/words_alpha.txt"))
        .expect("Error reading dictionary")
        .split_ascii_whitespace()
        .for_each(|word| words_by_len
            .entry(word.len())
            .or_insert(vec![])
            .push(word.to_uppercase()));
        
    // Try every 16 character word
    let panagrams = words_by_len.get(&16).unwrap();
    for word in panagrams{
        let mut pt = ct.clone();
        let pt = Aes128::new(&GenericArray::from_slice(word.as_bytes())).decrypt_padded::<Pkcs7>(&mut pt);
        if let Ok(raw_pt) = pt {
            if let Ok(pt_str) = String::from_utf8(raw_pt.to_vec()) {
                println!("{} - {}", word, pt_str.split("\n").next().unwrap_or(""));
            }
        }
    }
    println!("Finished {}:{} = {}", 16, 0, panagrams.len());

    // Try every space-separated-two-word key
    // Parallelized somewhat lazily since the word lengths have a huge difference
    // in run-time. 1+15 takes a few seconds, while 7+8 takes many dozens of minutes
    (1..15).into_par_iter().for_each(|i| {
        match (words_by_len.get(&i), words_by_len.get(&(16 - i - 1))) {
            (Some(lefts), Some(rights)) => {
                for left in lefts {
                    for right in rights {
                        let key = format!("{} {}", left, right);
                        let mut pt = ct.clone();
                        let pt = Aes128::new(&GenericArray::from_slice(key.as_bytes())).decrypt_padded::<Pkcs7>(&mut pt);
                        if let Ok(raw_pt) = pt {
                            if let Ok(pt_str) = String::from_utf8(raw_pt.to_vec()) {
                                println!("{} {} - {}", left, right, pt_str.split("\n").next().unwrap_or(""));
                            }
                        }
                    }
                }
                println!("Finished {}:{} = {}", i, 16-i-1, lefts.len()*rights.len());
            },
            (_, _) => () // The case where for some reason there isn't a list of words of length `i` or `16 - i - 1`
        }
    });
}
