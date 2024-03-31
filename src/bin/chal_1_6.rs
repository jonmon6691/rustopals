use std::{fs::File, io::Read};
use itertools::Itertools;
use rustopals::raw::EverythingRemainsRaw;

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
    let mut avg_hamms: [Option<f32>; MAX_KEY_LEN] = [None; MAX_KEY_LEN];
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
        avg_hamms[key_size] = match n_groups {
            0 => None,
            _ => Some(d_total as f32 / n_groups as f32),
        };
        
        println!("{} {:?}", key_size, avg_hamms[key_size]);
    }
    "Who knows!".to_string()
}

fn main() {
    println!("Lets do this");
    do_chal();
}

#[test]
fn chal_1_6() {
    assert_eq!(do_chal(), "I sure don't!");
}
