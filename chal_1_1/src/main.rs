use base64::Engine;
use itertools::Itertools;

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
    let str_iter = data.chars();
    let mut out: Vec<u8> = vec![];
    for (hi, low) in str_iter.tuples() {
        let num = hi.to_digit(16).unwrap() * 16 + low.to_digit(16).unwrap();
        out.push(num as u8);
    }
    Ok(out)
}

fn base64_encode(data: Vec<u8>) -> String
{
    String::from_utf8(data).unwrap()
}
