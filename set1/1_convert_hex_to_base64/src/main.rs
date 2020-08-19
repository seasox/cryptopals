use hex::FromHex;

fn encode_hex(h: &str) -> Vec<u8> {
    let b64chars = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/").into_bytes();
    let mut input = Vec::from_hex(h).expect("invalid hex string");
    let mut pad: Vec<u8> = vec![];
    let c = input.len() % 3;
    if c != 0 {
        for _ in 0..c {
            pad.push(61); // ASCII 61 is '='
            input.push(0);
        }
    }
    assert!(input.len() % 3 == 0);

    let mut out: Vec<u8> = vec![0; input.len()/3*4];

    let mut i = 0;
    let step = 3;
    while i < input.len() {
        let n: u32 = (input[i] as u32) << 16 | (input[i+1] as u32) << 8 | (input[i+2] as u32);
        let n1 = n >> 18 & 63;
        let n2 = n >> 12 & 63;
        let n3 = n >> 6 & 63;
        let n4 = n & 63;
        out[i] = b64chars[n1 as usize];
        out[i+1] = b64chars[n2 as usize];
        out[i+2] = b64chars[n3 as usize];
        out[i+3] = b64chars[n4 as usize];
        i += step;
    }
    out.extend(pad.iter().copied());

    return out;
}

fn main() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let result = encode_hex(hex);
    let result_str = String::from_utf8(result).unwrap();
    println!("{}", result_str);
}
