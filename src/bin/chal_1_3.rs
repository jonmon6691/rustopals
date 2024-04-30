/// Breaking single byte XOR

use rustopals::raw::EverythingRemainsRaw;

fn main() {
    println!("https://cryptopals.com/sets/1/challenges/3 - Single-byte XOR cipher");

    let ct_bytes =
        Vec::from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    // Get the highest score
    let answer = rustopals::SBX::from_ciphertext(&ct_bytes);

    //Print the results
    println!(
        "Key: dec:{} hex:{:x} ascii:{}",
        answer.key, answer.key, answer.key as char
    );
    println!("Plaintext: {}", answer.plaintext.to_string());
}

#[test]
fn chal_1_3() {
    let ct_bytes =
        Vec::from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let answer = rustopals::SBX::from_ciphertext(&ct_bytes);
    assert_eq!(answer.key, 111 ^ 55); // Obfuscated ;)
}
