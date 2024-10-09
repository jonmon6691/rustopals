use rand::{thread_rng, Rng};
use rustopals::{
    blocky::{aes_128_ecb_decrypt_vec, aes_128_ecb_encrypt_vec},
    raw::EverythingRemainsRaw,
};
use serde::{Deserialize, Serialize};
use serde_urlencoded;

#[derive(Debug, Deserialize, Serialize)]
struct User {
    email: String,
    uid: usize,
    role: String,
}

fn main() {
    let mut rng = thread_rng();
    let key = rng.gen();

    // Attempt 1: doesn't work because serde_urlencoded doesn't allow duplicate keys, and uid=10 
    // shows up in both snippets needed to put this token together.
    // Result: "email=myemail123%2Blolz%40gmail.com&uid=10&role=admin&uid=10&rol=user"
    let target = profile_for("myemail123+lolz@gmail.com", key); // gives role=$ on a boundary
    let admin_prefix = profile_for("lololololoadmin", key); // gives ^admin& on a boundary
    let eq_user_prefix = profile_for("lololololololo", key); // gives ^=user on a boundary

    let mut mal = Vec::new();
    mal.extend_from_slice(&target[0..48]);
    mal.extend_from_slice(&admin_prefix[16..32]);
    mal.extend_from_slice(&eq_user_prefix[32..48]);
    let login = load_profile(mal, key);
    dbg!(login);

    // Attempt 2: Lets assume that the role parser allows for trailing whitespace
    // Result: "email=myemail123%40gmail.com+++++++&uid=10&role=admin+++++++++++"
    // login = Some(
    //    User {
    //        email: "myemail123@gmail.com       ",
    //        uid: 10,
    //        role: "admin           ",
    //    },
    //)

    let target = profile_for("myemail123@gmail.com       ", key); // gives role=$ on a boundary
    let guts = profile_for("lololololoadmin                          ", key); // gives ^admin on one boundary and an empty padding block at the end
    let mut mal = Vec::new();
    mal.extend_from_slice(&target[0..48]);
    mal.extend_from_slice(&guts[16..32]);
    mal.extend_from_slice(&guts[64..80]);
    let login = load_profile(mal, key);
    dbg!(login);
}

fn profile_for(email: &str, key: [u8; 128 / 8]) -> Vec<u8> {
    let u = User {
        email: email.to_owned(),
        uid: 10,
        role: "user".to_owned(),
    };
    let token = serde_urlencoded::to_string(&u).expect("URL should have been encoded");
    print_block(&token);
    aes_128_ecb_encrypt_vec(token.into_bytes(), key)
}

fn print_block(input: &str) {
    for (i, c) in (0..).zip(input.chars()) {
        print!("{c}");
        if i % 16 == 15 {
            print!(" ");
        }
    }
    println!("");
    for i in 0..input.len() {
        print!("{}", (i%16) % 10);
        if i % 16 == 15 {
            print!(" ");
        }
    }
    println!("");
}

fn load_profile(token: Vec<u8>, key: [u8; 128 / 8]) -> Option<User> {
    let binding = &aes_128_ecb_decrypt_vec(token, key).to_string();
    dbg!(binding);
    match serde_urlencoded::from_str(binding) {
        Ok(v) => Some(v),
        Err(e) => {
            println!("Error deserializing token: {}", e.to_string());
            None
        }
    }
}
