/// Converts data between raw, hex, and base64
/// via command line utility, use --help for docs

use std::{fs, io::{self, stdin, stdout, Read, Write}};
use itertools::Itertools;
use rustopals::raw::EverythingRemainsRaw;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
struct Args {
    /// Input file (use '-' for STDIN)
    #[arg(default_value_t = {"-".to_string()})]
    input_file: String,
    
    /// Output file (use '-' for STDOUT)
    #[arg(default_value_t = {"-".to_string()})]
    output_file: String,
    
    /// Specifies the format of the input data
    #[arg(short='i', value_enum, default_value_t = DataFormat::Hex)]
    input_format: DataFormat,

    /// Specifies the format of the output data
    #[arg(short='o', value_enum, default_value_t = DataFormat::Base64)]
    output_format: DataFormat,
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

    // Setup the input decoder
    let data = match args.input_format {
        DataFormat::Base64 => Vec::from_base64(&data.split_ascii_whitespace().join("")),
        DataFormat::Hex => Vec::from_hex(&data.split_ascii_whitespace().join("")),
        DataFormat::Raw => Vec::from(data.as_bytes()),
    };

    // Setup the output encoder
    let encoder = match args.output_format {
        DataFormat::Base64 => data.into_base64().into_bytes(),
        DataFormat::Hex => data.into_hex().into_bytes(),
        DataFormat::Raw => data,
    };

    dest.write(&encoder)
        .expect(&format!("Error writing into file {}", args.output_file));
}

#[test]
fn chal_1_1() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let input_bytes = Vec::from_hex(input);
    let output = input_bytes.into_base64();
    assert_eq!(
        output,
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string()
    );
}
