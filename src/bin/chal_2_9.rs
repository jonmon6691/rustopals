fn main() {
    println!("Fillerup");
    let data = Vec::from("YELLOW SUBMARINE");
    let padded: Vec<Vec<u8>> = pkcs7_chunker(data, 20);
    println!("{:?}", padded);
}

/// Takes a Vec<u8> of data of any length, returns a vector of chunks of size k_len, with the last chunk padded using PKCS7
/// 
/// # Panics
/// Panics if k_len is greater than 255
fn pkcs7_chunker(data: Vec<u8>, k_len:usize) -> Vec<Vec<u8>> {
    assert!(k_len <= 255);
    data.chunks(k_len)
        .into_iter()
        .map(|chunk| pkcs7_padder(chunk, k_len))
        .collect()
}

fn pkcs7_padder(chunk: &[u8], k_len: usize) -> Vec<u8> {
    match chunk.len().cmp(&k_len) {
        std::cmp::Ordering::Less => {
            let padding = k_len - chunk.len();
            chunk.iter()
                .chain(itertools::repeat_n(&(padding as u8), padding))
                .copied()
                .collect()
        }, // Do padding
        std::cmp::Ordering::Equal => Vec::from(chunk),
        std::cmp::Ordering::Greater => panic!("Data to pad is larger than desired block length!"),
    }
}

#[test]
fn chal_2_9() {
    let padded: Vec<Vec<u8>> = pkcs7_chunker(Vec::from("YELLOW SUBMARINE"), 20);
    assert_eq!(padded[0].len(), 20);
    assert_eq!(padded[0][16..], [4, 4, 4, 4])
}
