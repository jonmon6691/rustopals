use base64::Engine;

fn main() {
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
    let input_bytes = hex_decode(input).unwrap();
    let output = base64_encode(input_bytes);
    println!("{output}");
}

fn hex_decode(data: &str) -> Result<Vec<u8>, hex::FromHexError>
{
    Ok(vec![1])
}

fn base64_encode(data: Vec<u8>) -> String
{
    "Hard mode isn't done yet".to_string()
}
