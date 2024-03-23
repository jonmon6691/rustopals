fn main() {
    println!("https://cryptopals.com/sets/1/challenges/3 - Single-byte XOR cipher");

    let ct_bytes = rustopals::hex_decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();
    
    // Get the highest score
    let (key, _score) =  rustopals::crack_single_byte_xor(&ct_bytes);

    //Print the results
    println!("Key: dec:{} hex:{:x} ascii:{}", key, key, key as char);
    println!("Plaintext: {}", 
        match String::from_utf8(rustopals::single_byte_xor(&ct_bytes, key)) {
            Ok(s) => s,
            Err(_) => String::from("[utf-8 decoding error]")
        });
}
