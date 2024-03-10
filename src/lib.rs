use itertools::Itertools;

pub fn hex_decode(data: &str) -> Result<Vec<u8>, hex::FromHexError>
{
    let mut out: Vec<u8> = Vec::with_capacity(data.len()/2);
    for (hi, low) in data.chars().tuples() {
        let num = hi.to_digit(16).unwrap() * 16 + low.to_digit(16).unwrap();
        out.push(num as u8);
    }
    Ok(out)
}

const STRBASE64:&'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn base64_encode(data: Vec<u8>) -> String
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
