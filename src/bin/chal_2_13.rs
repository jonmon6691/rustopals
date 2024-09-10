use rand::{thread_rng, Rng};
use rustopals::{
    blocky::{aes_128_ecb_decrypt_vec, aes_128_ecb_encrypt_vec, crack_ecb_suffix},
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
    let channel = |input: &[u8]| {
        profile_for(
            &String::from_utf8(input.to_vec())
                .unwrap_or("Couldn't convert name into UTF-8".to_owned()),
            key,
        )
    };
    // TODO: the crack function needs to work if the channel has strict requirements on the probe
    //          characters.
    // let suf = crack_ecb_suffix(&channel);
    // dbg!(suf);
    let my_prof = profile_for("jonmon6691@gmail.com", key);
    let login = load_profile(my_prof, key);
    dbg!(login);
}

fn profile_for(email: &str, key: [u8; 128 / 8]) -> Vec<u8> {
    let u = User {
        email: email.to_owned(),
        uid: 10,
        role: "user".to_owned(),
    };
    let token = serde_urlencoded::to_string(&u).expect("URL should have been encoded");
    aes_128_ecb_encrypt_vec(token.into_bytes(), key)
}

fn load_profile(token: Vec<u8>, key: [u8; 128 / 8]) -> Option<User> {
    serde_urlencoded::from_str(&aes_128_ecb_decrypt_vec(token, key).to_string()).ok()
}
