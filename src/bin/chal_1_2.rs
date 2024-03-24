use std::iter::zip;

fn main() {
    println!("https://cryptopals.com/sets/1/challenges/2 - Fixed XOR");

    let input_bytes = rustopals::hex_decode("1c0111001f010100061a024b53535009181c");
    let key_bytes = rustopals::hex_decode("686974207468652062756c6c277320657965");
    let pt = String::from_utf8(zip(input_bytes.iter(), key_bytes.iter()).map(|(a, b)| a ^ b).collect::<Vec<u8>>()).unwrap();

    println!("Key:       {}", String::from_utf8(key_bytes.clone()).unwrap());
    println!("Plaintext: {}", pt);
}

#[test]
fn chal_1_2() {
    let input_bytes = rustopals::hex_decode("1c0111001f010100061a024b53535009181c");
    let key_bytes = rustopals::hex_decode("686974207468652062756c6c277320657965");
    let expexted = rustopals::hex_decode("746865206b696420646f6e277420706c6179");

    let pt: Vec<u8> = zip(input_bytes.iter(), key_bytes.iter()).map(|(a, b)| a ^ b).collect();
    
    assert_eq!(pt, expexted);
}
