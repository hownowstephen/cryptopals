extern crate rustc_serialize as serialize;

use self::serialize::hex::{FromHex, ToHex};
use self::serialize::base64::{self, ToBase64};

pub fn hex2b64(hexval: String) -> String {
    return hexval.from_hex().unwrap().to_base64(base64::STANDARD);
}

pub fn fixed_xor(a: String, b: String) -> String {

    assert!(a.len() == b.len());

    let avec = a.from_hex().unwrap();
    let bvec = b.from_hex().unwrap();

    let result : Vec<u8> = avec.iter()
            .zip(bvec.iter())
            .map(|(x, y)| (x ^ y))
            .collect();

    return result.to_hex();
}