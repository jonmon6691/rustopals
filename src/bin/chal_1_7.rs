/// AES-128-ECB Decryption
///
/// Provides a commandline utility, use --help for more info

use std::{fs, io::{self, stdin, stdout, Read, Write}};
use aes::{cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit}, Aes128};
use cipher::block_padding::Pkcs7;
use rustopals::raw::EverythingRemainsRaw;
use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Decrypts input using provided key. Writes result to output")]
struct Args {
    /// Input file (use '-' for STDIN)
    #[arg(default_value_t = {"-".to_string()})]
    input_file: String,
    
    /// Output file (use '-' for STDOUT)
    #[arg(default_value_t = {"-".to_string()})]
    output_file: String,
    
    /// AES Key (128-bit only)
    #[arg(short='k', long="key")]
    key: String,
}

fn main() {
    let args = Args::parse();
    
    // Setup the input source
    let source: Box<dyn Read> = if args.input_file == "-".to_owned() {
        Box::new(stdin())
    } else {
        Box::new(fs::File::open(&args.input_file)
            .expect(&format!("Couldn't open input file {}", args.input_file)))
    };

    // Setup the output source
    let mut dest: Box<dyn Write> = if args.output_file == "-".to_owned() {
        Box::new(stdout())
    } else {
        Box::new(fs::File::create(&args.output_file)
            .expect(&format!("Couldn't open output file {}", args.output_file)))
    };

    let data = io::read_to_string(source)
        .expect(&format!("Error reading from {}", args.input_file));

    let mut ct = Vec::from_base64(&data);

    let key = args.key.as_bytes();
    assert_eq!(key.len(), 16, "Key must be 16 bytes!");

    let pt = Aes128::new(&GenericArray::from_slice(&key))
        .decrypt_padded::<Pkcs7>(&mut ct)
        .expect("Padding error in decrypted message!");

    dest.write(pt).expect("Couldn't write to output file!");
}

#[test]
fn chal_1_7() {
    use itertools::fold;

    let mut raw_input = Vec::from_base64(&fs::read_to_string("test_data/7.txt")
        .expect("Error reading file"));

    let key = "YELLOW SUBMARINE".as_bytes();

    let pt = Aes128::new(&GenericArray::from_slice(&key))
        .decrypt_padded::<Pkcs7>(&mut raw_input)
        .expect("Padding error in decrypted message!")
        .to_owned();

    // Check sum
    let sum = fold(pt.iter(), 0 as usize, |a, &b| a + b as usize);
    assert_eq!(sum, 247154);
}
