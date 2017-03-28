extern crate rustc_serialize as serialize;

use self::serialize::hex::FromHex;
use std::str;
use std::usize;

fn set1_challenge3(){

    // surprise, this is the alphabet
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();

    let stdorder = "ETAOINSHRDLCUMWFGYPBVKJXQZ".to_string();

    let encrypted = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let len = encrypted.to_string().len();

    let mut result_score : i32 = 26 * 26;
    let mut result_key : String = String::new();
    let mut result_message : String = String::new();

    #[derive(Debug, Copy, Clone)]
    struct TFCharacter {
        count: i32,
        value: char,
    }

    for key in alphabet.chars() {

        let unhexed = encrypted.to_string().from_hex().unwrap();
        let crepeat = (0..len).map(|_| key.to_string()).collect::<String>();

        let result : Vec<u8> = unhexed.iter().zip(crepeat.as_bytes()).map(|(x, y)| (x ^ y)).collect();

        let mut f : [TFCharacter; 26] = [TFCharacter{count: 0, value: 'a'}; 26];

        for (i, v) in alphabet.chars().enumerate() {
            f[i].value = v
        }

        for c in result.as_slice() {
            let mut v = *c;
            // to_uppercase the hard way
            if v >= 97 && v <= 122 {
                v = v - 32;
            }
            if v >= 65 && v <= 90 {
                f[usize::from(v - 65)].count += 1;
            }
        }

        // sort inline
        f.sort_by(|a, b| b.count.cmp(&a.count));

        let mut score : i32 = 0;
        for (i, got) in f.iter().map(|x| x.value).enumerate() {
            for (j, want) in stdorder.chars().enumerate() {
                if want == got {
                    let mut distance = j as i32 - i as i32;
                    if distance < 0 {
                        distance *= -1;
                    }
                    score += distance;
                }
            }
        }
        
        if score < result_score {
            result_score = score;
            result_key = key.to_string();
            result_message = str::from_utf8(result.as_slice()).unwrap().to_string();
        }
        
    }

    println!("{} {} {:?}", result_score, result_key, result_message)

}

fn main() {
    set1_challenge3();
}