fn main() {
    let answer =  rustopals::SBX::from_ciphertext(&rustopals::hex_decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"));
    println!("Key: dec:{} hex:{:x} ascii:{}\n{}", answer.key, answer.key, answer.key as char, answer.to_string());
}
