use hex::FromHex;
use std::str;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;

fn xor(x1: &[u8], x2: &[u8]) -> Vec<u8> {
    assert_eq!(x1.len(), x2.len());
    let mut out = vec![0; x1.len()];
    for x in 0..x1.len() {
        out[x] = x1[x] ^ x2[x];
    }
    return out;
}

fn score(string: &str) -> i32 {
    let rated = "ETAOINSHRDLCUMWFGYPBVKJXQZ";
    let mut score: i32 = 0;
    for (i, c) in rated.split("").enumerate() {
        let rate = (26 - i as i32) * string.to_uppercase().matches(c).count() as i32;
        score += rate;
    }
    return score;
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("src/data.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut results: BTreeMap<i32, Vec<String>> = BTreeMap::new();
    for cipher_str in contents.split("\n") {
        let cipher: Vec<u8> = Vec::from_hex(cipher_str).expect("invalid hex string");
        for candidate in 0..255 {
            let key: Vec<u8> = vec![candidate; cipher.len()];
            let decrypt = xor(&cipher, &key);
            let secret = str::from_utf8(&decrypt);
            if secret.is_ok() {
                let score = score(secret.unwrap());
                results
                    .entry(score)
                    .or_insert_with(Vec::new)
                    .push(secret.unwrap().to_string());
            }
        }
    }
    for (k, v) in results {
        println!("score {}:", k);
        for s in v {
            println!("\t{}", s);
        }
    }
    Ok(())
}
