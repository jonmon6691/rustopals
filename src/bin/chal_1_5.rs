fn main() {
    let ct: String = rustopals::hex_encode(String::from("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal").into_bytes().iter().zip(String::from("ICE").into_bytes().iter().cycle()).map(|(a, b)| a ^ b).collect());
    let ct_expected = String::from("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    println!("{}", if ct == ct_expected {"Challenge 5 passed"} else {"Try again!"});
}
