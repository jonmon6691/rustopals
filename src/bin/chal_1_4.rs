use itertools::Itertools;
use ureq;

fn main() {
    println!("https://cryptopals.com/sets/1/challenges/4 - Detect single-character XOR");

    // Get the ciphertext strings directly from the cryptopals site. If you are reading this in 50 years and the URL is long dead; you have my condolences, cryptopals was really cool back in the day
    let ct_file= ureq::get("https://cryptopals.com/static/challenge-data/4.txt")
        .call().unwrap()
        .into_string().unwrap();

    // Split the long body string into lines
    let ct_lines: Vec<&str> = ct_file.split('\n').collect();

    // One line to rule them all
    let (_score, key, plaintext) = ct_lines.iter()
        .map(|x| rustopals::hex_decode(x).unwrap())
        .map(|x| rustopals::crack_single_byte_xor(&x))
        .sorted_by_key(|(score, _, _)| *score)
        .rev().next().unwrap();

    //Print the results
    println!("Key: dec:{} hex:{:x} ascii:{}", key, key, key as char);
    println!("Plaintext: {}", plaintext);
}
