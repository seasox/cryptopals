use hex::FromHex;

fn xor(x1: Vec<u8>, x2: Vec<u8>) -> Vec<u8> {
    assert_eq!(x1.len(), x2.len());
    let mut out = vec![0; x1.len()];
    for x in 0..x1.len() {
        out[x] = x1[x] ^ x2[x];
    }
    return out;
}

fn main() {
    let x1 = Vec::from_hex("1c0111001f010100061a024b53535009181c").expect("invalid hex string");
    let x2 = Vec::from_hex("686974207468652062756c6c277320657965").expect("invalid hex string");
    let expected = Vec::from_hex("746865206b696420646f6e277420706c6179").expect("invalid hex string");
    let res = xor(x1, x2);
    assert_eq!(expected, res);
    println!("passed");
}
