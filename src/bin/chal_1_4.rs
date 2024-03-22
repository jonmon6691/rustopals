use ureq;

fn main() {
    // Get the ciphertext strings directly from the cryptopals site. If you are reading this in 50 years and the URL is long dead; you have my condolences, cryptopals was really cool back in the day
    let ct_files= ureq::get("https://cryptopals.com/static/challenge-data/4.txt")
        .call().unwrap()
        .into_string().unwrap();
    
    let ct_lines: Vec<&str> = ct_files.split('\n').collect();

    for line in ct_lines {
        println!("{}", line);
    }
}
