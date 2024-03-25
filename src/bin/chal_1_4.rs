use itertools::Itertools;
use rustopals::EverythingRemainsRaw;
use ureq;

fn main() {
    println!("https://cryptopals.com/sets/1/challenges/4 - Detect single-character XOR");

    // Get the ciphertext strings directly from the cryptopals site. If you are reading this in 50 years and the URL is long dead; you have my condolences, cryptopals was really cool back in the day
    // One line to rule them all
    let ice = ureq::get("https://cryptopals.com/static/challenge-data/4.txt")
        .call().unwrap()
        .into_string().unwrap().split('\n')
        .map(|line| Vec::from_hex(line))
        .map(|ct| rustopals::SBX::from_ciphertext(&ct))
        .sorted_by_key(|trial| trial.score)
        .rev().next().unwrap();

    //Print the results
    println!("Key: dec:{} hex:{:x} ascii:{}", ice.key, ice.key, ice.key as char);
    println!("{}", ice.to_string());
}

#[test]
fn chal_1_4() {
    let ice = ureq::get("https://cryptopals.com/static/challenge-data/4.txt")
        .call().unwrap()
        .into_string().unwrap().split('\n')
        .map(|line| Vec::from_hex(line))
        .map(|ct| rustopals::SBX::from_ciphertext(&ct))
        .sorted_by_key(|trial| trial.score)
        .rev().next().unwrap();
    assert_eq!(ice.key, !202); // Obfuscated ;)
}
