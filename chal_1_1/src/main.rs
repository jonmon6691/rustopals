use base64::Engine;
use itertools::Itertools;

const STRBASE64:&'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

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
    let mut out: Vec<u8> = Vec::with_capacity(data.len()/2);
    for (hi, low) in data.chars().tuples() {
        let num = hi.to_digit(16).unwrap() * 16 + low.to_digit(16).unwrap();
        out.push(num as u8);
    }
    Ok(out)
}

fn base64_encode(data: Vec<u8>) -> String
{
    let mut out: Vec<char> = Vec::with_capacity(data.len());
    for (one, two, three) in data.iter().tuples() {
        let cp_one = one >> 2;
        let cp_two = ((one & 0x3) << 4) | (two >> 4);
        let cp_three = ((two & 0xf) << 2) | (three >> 6);
        let cp_four = three & 0x3f;
        out.push(STRBASE64.chars().nth(cp_one as usize).unwrap());
        out.push(STRBASE64.chars().nth(cp_two as usize).unwrap());
        out.push(STRBASE64.chars().nth(cp_three as usize).unwrap());
        out.push(STRBASE64.chars().nth(cp_four as usize).unwrap());
    }
    out.iter().collect()
}
