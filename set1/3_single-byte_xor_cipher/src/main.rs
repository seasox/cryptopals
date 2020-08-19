use hex::FromHex;
use std::str;

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

fn main() {
    let cipher_str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let cipher: Vec<u8> = Vec::from_hex(cipher_str).expect("invalid hex string");
    let mut max_score = 0;
    let mut result_str = String::new();
    for candidate in 0..255 {
        let key: Vec<u8> = vec![candidate; cipher.len()];
        let decrypt = xor(&cipher, &key);
        let secret = str::from_utf8(&decrypt);
        if secret.is_ok() {
            let score = score(secret.unwrap());
            if score > max_score {
                max_score = score;
                result_str = secret.unwrap().to_string();
            }
        }
    }
    println!("secret: {}; score: {}", result_str, max_score);
}
