use rustopals::blocky::pkcs7_chunker;

fn main() {
    println!("Fillerup");
    let data = Vec::from("YELLOW SUBMARINE");
    let padded: Vec<Vec<u8>> = pkcs7_chunker(data, 20);
    println!("{:?}", padded);
}

#[test]
fn chal_2_9() {
    let padded: Vec<Vec<u8>> = pkcs7_chunker(Vec::from("YELLOW SUBMARINE"), 20);
    assert_eq!(padded[0].len(), 20);
    assert_eq!(padded[0][16..], [4, 4, 4, 4])
}
