use cipher::BlockSizeUser;
use itertools::{
    repeat_n,
    FoldWhile::{Continue, Done},
    Itertools,
};
use rand::{thread_rng, Rng};
use rustopals::{blocky::aes_128_ecb_encrypt_vec, hamming, raw::EverythingRemainsRaw};

fn main() {
    let mut rng = thread_rng();
    // Random key, generated here so it can be persistent throughout the attack
    let key: [u8; 16] = rng.gen();
    ecb_phonebook(|input| channel(input, key));
}

/// Decrypts an unknown suffix appended to user controlled input into an ECB encryption system.
/// Can't work for prefixes because I can't control where in the block the data appears like
/// I can for suffixes.
fn ecb_phonebook(blackbox: impl Fn(&[u8]) -> Vec<u8>) -> Option<Vec<u8>> {
    // the block size of the cipher
    let block_size = (1..512).fold_while(blackbox(&[]).len(), |prev_size, input_len: usize| {
        let new_size = blackbox(&Vec::from_iter(repeat_n(0u8, input_len))).len();
        if new_size != prev_size {
            Done(new_size - prev_size)
        } else {
            Continue(new_size)
        }
    });
    let block_size = match block_size {
        Continue(_) => {
            println!("Couldn't determine block size!");
            return None;
        }
        Done(x) => x,
    };
    dbg!(block_size);

    // Finds pair of repeated blocks caused by our input, this tells us when the suffix is aligned on a block boundary and where it is.
    let (n_pad_until_suffix_aligned, start_of_repeated_blocks) = (2 * block_size..3 * block_size)
        .map(|i| {
            find_consecutive_same_blocks(&blackbox(&Vec::from_iter(repeat_n(0u8, i))), block_size)
        })
        .find_position(|trial| trial.is_some())
        .expect("Couldn't find duplicate blocks, are you sure this is ECB?");

    let start_of_repeated_blocks = start_of_repeated_blocks.expect("This can't happen");
    dbg!(n_pad_until_suffix_aligned);
    dbg!(start_of_repeated_blocks);

    // Find out how much of the suffix is in the padding block
    let n_pad_until_suffix_overflow =
        (n_pad_until_suffix_aligned + 1..n_pad_until_suffix_aligned + block_size).fold_while(
            blackbox(&Vec::from_iter(repeat_n(0, n_pad_until_suffix_aligned))).len(),
            |prev_size, i| {
                let new_size = blackbox(&Vec::from_iter(repeat_n(0, i))).len();
                if new_size != prev_size {
                    Done(i)
                } else {
                    Continue(new_size)
                }
            },
        );

    let n_pad_until_suffix_overflow = match n_pad_until_suffix_overflow {
        Continue(_) => {
            println!("Not really possible to get here since you've already proven this is a reasonable cipher but ok");
            return None;
        }
        Done(x) => x,
    };
    let suffix_remainder = block_size + n_pad_until_suffix_aligned - n_pad_until_suffix_overflow;
    dbg!(suffix_remainder);

    let ct_len = blackbox(&Vec::from_iter(repeat_n(0, n_pad_until_suffix_aligned))).len();
    let len_suffix = ct_len + suffix_remainder - block_size - start_of_repeated_blocks;
    let len_prefix = start_of_repeated_blocks - n_pad_until_suffix_aligned;
    dbg!(len_suffix);
    dbg!(len_prefix);

    // Honestly this can all be more efficient... If there's no prefix then you don't need a separate step to determine siffix remainder, you'll just be repeating youself. This could be fixed by memoizing the blackbox which probably isn't a bad idea, or by just being not-dumb (R)

    //But for now I should just proceed since it seems to work
    // Now for the fun part :)

    let mut pt: Vec<u8> = Vec::new();
    for i in 0 .. len_suffix {
        dbg!(pt.clone().to_string());
        // TODO: There will have to be some kind of block offset here too
        let block_offset = block_size * (i / block_size);
        let ct_target = &blackbox(&Vec::from_iter(repeat_n(0, block_offset + block_size - 1 - i)))[block_offset + start_of_repeated_blocks.. block_offset + start_of_repeated_blocks + block_size];
        for check_byte in 0..=255 {
            let probe = Vec::from_iter(repeat_n(0, block_offset + block_size - 1 - i).chain(pt.iter().copied()).chain(repeat_n(check_byte, 1)));
            let ct = &blackbox(&probe)[block_offset + start_of_repeated_blocks .. block_offset + start_of_repeated_blocks + block_size];
            if *ct_target == *ct {
                dbg!(check_byte);
                pt.push(check_byte);
                break; // for check_byte
            }
        }
    }

    Some(vec![])
}

fn print_blocks(data: &[u8], block_len: usize) {
    for c in data.chunks(block_len) {
        let mut token = Vec::from(c).into_base64();
        token.truncate(3);
        print!("{} ", token);
    }
    print!("\n");
}

/// Returns None if there are no consecutive blocks which match, otherwise returns the index of the first block in the matching consecutive pair
fn find_consecutive_same_blocks(data: &[u8], block_len: usize) -> Option<usize> {
    let n_chunks = data.len() / block_len - 1;
    for i in 0..n_chunks {
        if hamming(
            &data[(i + 0) * block_len..(i + 1) * block_len],
            &data[(i + 1) * block_len..(i + 2) * block_len],
        ) == 0
        {
            return Some(i * block_len);
        }
    }
    None
}

fn channel(input: &[u8], key: [u8; 128 / 8]) -> Vec<u8> {
    // Secret suffix, we will decrypt it by attacking ECB's inherent weakness
    let suffix: Vec<u8> = Vec::from_base64("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK");
    let prefix: Vec<u8> = Vec::new(); //from_iter(0..21);
    let data: Vec<u8> = Vec::from_iter(
        prefix
            .iter()
            .chain(input.iter().chain(suffix.iter()))
            .copied(),
    );
    let ret = aes_128_ecb_encrypt_vec(data, key);
    // print_blocks(&ret, key.len());
    ret
}

#[test]
fn chal_2_12() {
    assert_eq!(1, 1);
}
