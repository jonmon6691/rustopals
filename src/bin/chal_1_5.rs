use rustopals::raw::EverythingRemainsRaw;

fn main() {
    println!("https://cryptopals.com/sets/1/challenges/5 - Implement repeating-key XOR");

    let plaintext = String::from("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal").into_bytes();
    let key = String::from("ICE").into_bytes();

    // Function chaining feels so right
    let ct: Vec<u8> = plaintext.iter()
        .zip(key.iter().cycle())
        .map(|(a, b)| a ^ b)
        .collect();

    let ct: String = ct.into_hex();
    
    println!("Got: {}", ct)
}

#[test]
fn chal_1_5() {
    let plaintext = String::from("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal").into_bytes();
    let key = String::from("ICE").into_bytes();

    // Function chaining feels so right
    let ct: Vec<u8> = plaintext.iter()
        .zip(key.iter().cycle())
        .map(|(a, b)| a ^ b)
        .collect();

    let ct: String = ct.into_hex();
    let ct_expected = String::from("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    
    assert_eq!(ct, ct_expected);
}
