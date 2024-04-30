/// Detecting single key XOR

use itertools::Itertools;
use rustopals::{raw::EverythingRemainsRaw, SBX};
use std::fs;

fn do_chal() -> SBX {
    // One line to rule them all
    fs::read_to_string("test_data/4.txt")
        .expect("Error reading input file")
        .split_whitespace()
        .map(|line| Vec::from_hex(line))
        .map(|ct| rustopals::SBX::from_ciphertext(&ct))
        .sorted_by_key(|trial| trial.score)
        .rev()
        .next()
        .expect("No data found in input file")
}

fn main() {
    println!("https://cryptopals.com/sets/1/challenges/4 - Detect single-character XOR");

    let ice = do_chal();

    //Print the results
    println!(
        "Key: dec:{} hex:{:x} ascii:{}",
        ice.key, ice.key, ice.key as char
    );
    println!("Plaintext: {}", ice.plaintext.to_string());
}

#[test]
fn chal_1_4() {
    let ice = do_chal();
    assert_eq!(ice.key, !202); // Obfuscated ;)
}

/* For posterity, here's the version that took down the website (maybe)
use ureq;

fn chal_1_4() {
    // Get the ciphertext strings directly from the cryptopals site. If you are reading this in 50 years and the URL is long dead; you have my condolences, cryptopals was really cool back in the day
    let ice = ureq::get("https://cryptopals.com/static/challenge-data/4.txt")
        .call().unwrap()
        .into_string().unwrap().split('\n')
        .map(|line| Vec::from_hex(line))
        .map(|ct| rustopals::SBX::from_ciphertext(&ct))
        .sorted_by_key(|trial| trial.score)
        .rev().next().unwrap();
    assert_eq!(ice.key, !202); // Obfuscated ;)
}
*/
