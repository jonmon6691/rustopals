use itertools::Itertools;
use ureq;

fn main() {
    let ice = ureq::get("https://cryptopals.com/static/challenge-data/4.txt").call().unwrap().into_string().unwrap().split('\n').map(|line| rustopals::hex_decode(line)).map(|ct| rustopals::SBX::from_ciphertext(&ct)).sorted_by_key(|trial| trial.score).rev().next().unwrap();
    println!("Key: dec:{} hex:{:x} ascii:{}\n{}", ice.key, ice.key, ice.key as char, ice.to_string());
}
