use std::{fs::File, io::Read};
use itertools::Itertools;
use rustopals::EverythingRemainsRaw;

fn do_chal() {
    let a = File::open("test_data/6.txt")
        .unwrap().bytes()
        .map(|c| c.unwrap() as char)
        .collect::<String>()
        .split('\n').join("");

    let raw_input = Vec::from_base64(&a);
    let mut ri_iter = raw_input.into_iter();

    for key_size in 0..40 {
        loop {
            let mut chunk: Vec<Option<u8>> = Vec::new();
            for _ in 0..key_size {
                chunk.push(ri_iter.next());
            }
            if chunk.last() == None { break; /* loop */ }
        }
    }
        
    println!("{}", a);
}

fn main() {
    println!("Lets do this");
    do_chal();
}

#[test]
fn chal_1_6() {
    assert_eq!(2+2, 5);
}
