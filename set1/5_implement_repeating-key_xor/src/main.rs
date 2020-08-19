use std::str;
use std::fs::File;
use std::io::prelude::*;

fn encrypt_repeating(buf: &[u8], keys: &[u8]) -> Vec<u8> {
    let mut out = vec![0; buf.len()];
    let keys_len = keys.len();
    for x in 0..buf.len() {
        let keys_pos = x % keys_len;
        let key = keys[keys_pos];
        out[x] = buf[x] ^ key;
    }
    return out;
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("src/secret.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let buf = contents.into_bytes();
    let keys = "ICE".as_bytes();
    let cipher = encrypt_repeating(&buf, &keys);
    println!("{}", hex::encode(cipher));
    Ok(())
}
