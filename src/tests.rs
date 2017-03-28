use common::*;

#[test]
fn test_hex2b64_set1challenge1() {
    let hexval = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expect = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let result = hex2b64(hexval.to_string());

    assert!(result == expect.to_string());
}

#[test]
fn test_fixed_xor_set1challenge2() {
    let left = "1c0111001f010100061a024b53535009181c";
    let right = "686974207468652062756c6c277320657965";
    let expect = "746865206b696420646f6e277420706c6179";

    let result = fixed_xor(left.to_string(), right.to_string());

    assert!(result == expect.to_string());
}