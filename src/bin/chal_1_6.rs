use std::{collections::HashMap, fs::File, io::Read};
use itertools::Itertools;
use rustopals::{raw::EverythingRemainsRaw, SBX};

const MAX_KEY_LEN: usize = 40;

fn do_chal() -> String {
    // Load the file, remove newlines to make one long line of b64
    let a = File::open("test_data/6.txt")
        .unwrap().bytes()
        .map(|c| c.unwrap() as char)
        .collect::<String>()
        .split('\n').join("");

    // Decode b64
    let raw_input = Vec::from_base64(&a);
    
    // Group the data into chunks of len()==keysize, compute the avg hamming distance between every consecutive group
    let mut avg_hamms: HashMap<usize, (f32, Vec<Vec<u8>>)> = HashMap::new();
    for key_size in 1..MAX_KEY_LEN {
        let mut ri_iter = raw_input.iter();
        let mut chunks: Vec<Vec<u8>> = Vec::new();
        
        // Make groups
        loop {
            let mut chunk: Vec<u8> = Vec::new();
            for _ in 0..key_size {
                let n = match ri_iter.next() {
                    Some(n) => n.clone(),
                    None => break 
                };
                chunk.push(n);
            }

            if chunk.len() == key_size {
                chunks.push(chunk)
            } else {
                break; /* loop */ 
            }
        }

        // Compute average hamming distance
        let mut n_groups = 0;
        let mut d_total = 0;
        for (a, b) in chunks.iter().tuple_windows() {
            n_groups += 1;
            d_total += rustopals::hamming(a, b) / key_size;
        }
        if n_groups != 0 {
            avg_hamms.insert(key_size, (d_total as f32 / n_groups as f32, chunks));
        }
    }

    // Get key-length with lowest Hamming Distance
    let (klen, (_score, chunks)) = avg_hamms.iter().sorted_by(|(_ks1, hd1), (_ks2, hd2)| hd1.0.partial_cmp(&hd2.0).expect("Can't sort a NaN")).next().unwrap().clone();

    println!("Most probable key length (bytes): {}", klen);

    // Transpose the data into single-byte xor problems
    let mut transpose: Vec<Vec<u8>> = Vec::from((0..*klen).map(|_| Vec::new()).collect::<Vec<Vec<u8>>>());
    for chunk in chunks {
        for i in 0..*klen {
            transpose.get_mut(i).unwrap().push(match chunk.get(i) { Some(i) => *i, None => {break;}});
        }
    }
    
    // Solve each SBX
    let key: Vec<u8> = transpose.iter().map(|x| SBX::from_ciphertext(x).key).collect();
    println!("Key: \"{}\"", String::from_utf8(key.clone()).unwrap());

    // Decrypt the ciphertext
    let plaintext = String::from_utf8(raw_input.iter().zip(key.iter().cycle()).map(|(a, b)| a ^ b).collect()).unwrap();
    println!("\nPlaintext:\n{}", plaintext);

    // Return the key
    String::from_utf8(key.clone()).unwrap()
}

fn main() {
    println!("Lets do this");
    do_chal();
}

#[test]
fn chal_1_6() {
    // Obfuscated ;)
    let expected_key = String::from_utf8(Vec::from_base64("VGVybWluYXRvciBYOiBCcmluZyB0aGUgbm9pc2U=")).unwrap();
    assert_eq!(do_chal(), expected_key);
}
