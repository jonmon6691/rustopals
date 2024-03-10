fn main() {
    let input_bytes = rustopals::hex_decode("1c0111001f010100061a024b53535009181c").unwrap();
    let key_bytes = rustopals::hex_decode("686974207468652062756c6c277320657965").unwrap();
    let key_str = String::from_utf8(key_bytes.clone()).unwrap();
    println!("Key: {key_str}");

    let mut pt_bytes: Vec<u8> = Vec::new();
    for (i, a) in input_bytes.iter().enumerate() {
        pt_bytes.push(*a ^ key_bytes.get(i % key_bytes.len()).unwrap());
    }

    let pt_str = String::from_utf8(pt_bytes).unwrap();
    println!("Plaintext: {pt_str}");
}
