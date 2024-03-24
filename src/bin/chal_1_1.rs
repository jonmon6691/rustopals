use base64::Engine;

fn main() {
    println!("https://cryptopals.com/sets/1/challenges/1 - Convert hex to base64");
    
    easy_mode();
    hard_mode();
}

fn easy_mode() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let input_bytes = hex::decode(input).unwrap();
    let output = base64::engine::general_purpose::STANDARD.encode(input_bytes);
    println!("{output}");
}

fn hard_mode() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let input_bytes = rustopals::hex_decode(input);
    let output = rustopals::base64_encode(input_bytes);
    println!("{output}");
}

