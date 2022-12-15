use hex::FromHex;
use std::iter::repeat;
use std::str;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;

fn bit(b: u8, f: u8) -> u8 {
    return b >> f & 0b1;
}

fn hamming(b1: &[u8], b2: &[u8]) -> u32 {
    assert_eq!(b1.len(), b2.len());

    let mut ham = 0;
    for i in 0..b1.len() {
        for j in 0..8 {
            if bit(b1[i], j) != bit(b2[i], j) {
                ham += 1;
            }
        }
    }
    return ham;
}

fn read_file(fname: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(fname)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.into_bytes())
}

fn xor(x1: &[u8], x2: &[u8]) -> Vec<u8> {
    assert!(x1.len() <= x2.len());
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
    // 2: unit test for hamming distance
    assert_eq!(hamming("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()), 37);

    let cipher = read_file("src/data.txt")?;
    // 1: iterate from 2 to 40 for keysize
    let mut keysizes = BTreeMap::new();
    for keysize in 2..41 {
        // 3+4: for each KEYSIZE, take the first and KEYSIZE of bytes. The KEYSIZE with the lowest
        // distance probably is correct. This implementation is digusting tho :D 
        let mut buf1 = vec![0; keysize];
        let mut buf2 = vec![0; keysize];
        let mut buf3 = vec![0; keysize];
        let mut buf4 = vec![0; keysize];
        for x in 0..keysize {
            buf1[x] = cipher[x];
        }
        for x in keysize..2*keysize {
            buf2[x-keysize] = cipher[x];
        }
        for x in 2*keysize..3*keysize {
            buf3[x-2*keysize] = cipher[x];
        }
        for x in 3*keysize..4*keysize {
            buf4[x-3*keysize] = cipher[x];
        }
        let dist1 = hamming(&buf1, &buf2) as f64 / keysize as f64;
        let dist2 = hamming(&buf3, &buf4) as f64 / keysize as f64;
        let avg_dist = ((dist1 + dist2)/2.0 * 1000000.0).floor() as u32; // floats are not ordered in rust b/c NaN != NaN. Multiply to keep decimal digits
        //println!("keysize {}: {:.02}", keysize, avg_dist);
        keysizes.entry(avg_dist).or_insert(Vec::new()).push(keysize); // no rating collisions in dataset
    }
    println!("keysizes: {:?}", keysizes);

    // 4: use 3 smallest keysize values
    let num_candidates = 3;
    let mut candidates = vec![0; num_candidates];
    let mut iter = keysizes.iter();
    for i in 0..num_candidates {
        let vector = iter.next().expect("empty").1;
        assert!(vector.len() == 1); // we do not support uckets with > 1 entry for our picks
        candidates[i] = vector[0];
    }
    println!("picked {} candidates: {:?}", num_candidates, candidates);

    for keysize in candidates {
        println!("trying candidate {}", keysize);
        // 5: break cipher in blocks of KEYSIZE length
        let num_blocks = (cipher.len() as f64/keysize as f64).ceil() as usize;
        let mut blocks = vec![vec![0; keysize]; num_blocks];
        for i in 0..cipher.len() {
            let j = i % keysize;
            let blockno = i / keysize;
            blocks[blockno][j] = cipher[i];
        }
        println!("{} blocks of len {}", blocks.len(), keysize);
        //println!("blocks: {:?}", blocks);

        // 6: transpose blocks
        let mut transposed = vec![vec![0; num_blocks]; keysize];
        for i in 0..cipher.len() {
            let blockno = i / keysize;
            let j = i % keysize;
            transposed[j][blockno] = blocks[blockno][j];
        }
        //println!("transposed: {:?}", transposed);
        // 7: solve each transposed block as if it was single-character XOR
        let mut possible_key = vec![0; keysize];
        for (i, block) in transposed.iter().enumerate() {
            let mut max_score = 0;
            let mut result_str = String::new();
            let mut best_candidate = 0;
            for candidate in 0..255 {
                let key: Vec<u8> = vec![candidate; block.len()];
                let decrypt = xor(&block, &key);
                let secret = str::from_utf8(&decrypt);
                if secret.is_ok() {
                    let score = score(secret.unwrap());
                    //println!("score for key {}: {}", candidate, score);
                    if score > max_score {
                        max_score = score;
                        result_str = secret.unwrap().to_string();
                        best_candidate = candidate;
                    }
                } else {
                    //println!("nok: {}", candidate);
                }
            }
            //println!("{}: {}", result_str, max_score);
            //println!("best candidate: {}", best_candidate);
            possible_key[i] = best_candidate;
        }
        println!("possible key: {:?}", possible_key);
        let key = repeat(possible_key).take(num_blocks).flatten().collect::<Vec<u8>>();
        let secret = xor(&cipher, &key);
        let secret_str = str::from_utf8(&secret);
        if secret_str.is_ok() {
            println!("possible result: {}", secret_str.unwrap().to_string());
        } else {
            println!("nok");
        }
    }

    Ok(())
}

pub fn split<T>(slice: &[T], n: usize) -> impl Iterator<Item = &[T]> {
    let len = slice.len() / n;
    let rem = slice.len() % n;
    Split { slice, len, rem }
}

#[derive(Debug)]
struct Split<'a, T> {
    slice: &'a [T],
    len: usize,
    rem: usize,
}

impl<'a, T> Iterator for Split<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            return None;
        }
        let mut len = self.len;
        if self.rem > 0 {
            len += 1;
            self.rem -= 1;
        }
        let (chunk, rest) = self.slice.split_at(len);
        self.slice = rest;
        Some(chunk)
    }
}
