fn main() {
    println!("https://cryptopals.com/sets/1/challenges/3 - Single-byte XOR cipher");

    let ct_bytes = rustopals::hex_decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();
    
    // Get the highest score
    let answer =  rustopals::SBX::from_ciphertext(&ct_bytes);

    //Print the results
    println!("Key: dec:{} hex:{:x} ascii:{}", answer.key, answer.key, answer.key as char);
    println!("{}", answer.to_string());
}
