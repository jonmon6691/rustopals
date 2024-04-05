use rustopals::raw::EverythingRemainsRaw;

fn main() {
    println!("https://cryptopals.com/sets/1/challenges/1 - Convert hex to base64");

    let input_bytes = Vec::from_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let output = input_bytes.clone().into_base64();

    println!("Input    : {}", input_bytes.to_string());
    println!("Expected : {}", expected);
    println!("Got      : {}", output);
}

#[test]
fn chal_1_1() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let input_bytes = Vec::from_hex(input);
    let output = input_bytes.into_base64();
    assert_eq!(
        output,
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string()
    );
}
