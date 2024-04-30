/// Detects ciphertext in ECB mode
/// 
/// Provides a commandline utility, use --help for more info

use std::{fs, io::{self, stdin, Read}};
use rustopals::{blocky::detect_ecb, raw::EverythingRemainsRaw};
use clap::{command, Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about = "Filters lines of ciphertexts on STDIN for those that contain repeats.")]
struct Args {
    /// Input file (use '-' for STDIN)
    #[arg(default_value_t = {"-".to_string()})]
    file: String,

    /// Specifies the format of the input data
    #[arg(short='f', value_enum, default_value_t = DataFormat::Hex)]
    data_format: DataFormat,

    /// Length of blocks to look for repeats of
    #[arg(short='l', default_value_t=16)]
    block_len: usize,

    /// Single line mode, newlines are removed from input and are treated as one monolithic ciphertext
    #[arg(short, default_value_t = false)]
    single: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum DataFormat {
    /// Standard Base64
    Base64,
    /// Hexidecimal bytes with no separators i.e. `CAFED00D0102`
    Hex,
    /// Raw bytes (not reccomended)
    Raw,
}

fn main() {
    let args = Args::parse();

    // Setup the input source
    let source: Box<dyn Read> = if args.file == "-".to_owned() {
        Box::new(stdin())
    } else {
        Box::new(fs::File::open(&args.file)
            .expect(&format!("Couldn't open input file {}", args.file)))
    };

    // Setup the input decoder
    let decoder: fn(&str) -> Vec<u8> = match args.data_format {
        DataFormat::Base64 => |x| Vec::from_base64(x),
        DataFormat::Hex => |x| Vec::from_hex(x),
        DataFormat::Raw => |x| Vec::from(x.as_bytes()),
    };

    // Read input
    let lines:Vec<String> = io::read_to_string(source)
        .expect("Error reading input")
        .split_ascii_whitespace()
        .map(|line| line.to_string())
        .collect();
    
    // Concat input if the single flag is provided
    let lines = match args.single {
        true => vec![lines.join("")],
        false => lines,
    };

    // Filter for ecb mode
    lines.iter()
        .map(|ct| decoder(ct))
        .filter(|ct| detect_ecb(ct, args.block_len))
        .for_each(|ct| println!("{}", ct.into_hex()));
}

#[test]
fn chal_1_8() {
    let cts: Vec<Vec<u8>> = fs::read_to_string("test_data/8.txt")
        .expect("Error reading file")
        .split_ascii_whitespace()
        .map(|ct| Vec::from_hex(ct))
        .filter(|ct| detect_ecb(ct, 16))
        .collect();

    assert!(cts.len() == 1);
    let result_b64 = cts[0].clone().into_base64();

    // Obfuscated ;-)
    assert_eq!(result_b64[0..16], "2IBhl0CooZt4QKij".to_owned());
}
