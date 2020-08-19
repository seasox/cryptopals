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

    // 5: use 3 smallest keysize values
    let num_candidates = 3;
    let mut candidates = vec![0; num_candidates];
    let mut iter = keysizes.iter();
    for i in 0..num_candidates {
        let vector = iter.next().expect("empty").1;
        assert!(vector.len() == 1); // we do not support uckets with > 1 entry for our picks
        candidates[i] = vector[0];
    }
    println!("picked {} candidates: {:?}", num_candidates, candidates);
    Ok(())
}
