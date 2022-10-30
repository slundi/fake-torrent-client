use rand::Rng;
use std::convert::TryFrom;

use crate::{KEY_LENGTH, PEER_ID_LENGTH};

#[derive(Debug, Clone)]
pub(crate) enum Algorithm {
    Regex,
    Hash,
    HashNoLeadingZero,
    ///inclusive bounds: from 1 to 2147483647
    DigitRangeTransformedToHexWithoutLeadingZeroes,
    RandomPoolWithChecksum,
}

const HASH_SYMBOLS: &str = "abcdef0123456789ABCDEF";

pub(crate) fn hash(no_leading_zero: bool, uppercase: Option<bool>) -> String {
    let mut rng = rand::thread_rng();
    let mut h = String::with_capacity(KEY_LENGTH);
    while h.len() < KEY_LENGTH {
        let i: usize = rng.gen_range(0usize..15usize);
        if i == 0 && no_leading_zero {
            continue;
        }
        if uppercase == None || uppercase.unwrap() {
            h.push(HASH_SYMBOLS.chars().nth(i + 6).unwrap());
        } else {
            h.push(HASH_SYMBOLS.chars().nth(i).unwrap());
        }
    }
    h
}
/// Generate a string from a regex pattern
pub(crate) fn regex(pattern: String) -> String {
    let mut rng = rand::thread_rng();
    let gen = rand_regex::Regex::compile(&pattern, 64).unwrap();
    (&mut rng).sample_iter(&gen).nth(64).unwrap()
}
/// Return a hex string from a random integer between 1 and 2147483647
pub(crate) fn digit_range_transformed_to_hex_without_leading_zero() -> String {
    let mut rng = rand::thread_rng();
    format!("{:x}", rng.gen_range(1u32..2147483647u32)).to_uppercase()
}
/// Used for some peer ID generation
pub(crate) fn random_pool_with_checksum(prefix: &str, characters_pool: &str) -> String {
    //println!("random_pool_with_checksum: prefix={}, \t characters_pool: {}", prefix, characters_pool);
    if prefix.is_empty() {
        panic!("algorithm prefix must not be null or empty.");
    }
    if characters_pool.is_empty() {
        panic!("algorithm character pool must not be null or empty.");
    }
    let mut out = String::with_capacity(PEER_ID_LENGTH);
    out.push_str(prefix);
    //TODO:
    let random_bytes = rand::thread_rng().gen::<[u8; PEER_ID_LENGTH]>();
    let base = characters_pool.chars().count();
    for b in random_bytes {
        out.push(characters_pool.chars().nth(usize::try_from(b).unwrap() % base).expect("Cannot get char in characters pool"));
        if out.len() == PEER_ID_LENGTH {break;}
    }
    out
}
//******************************************* TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    #[test]
    fn is_hash_ok() {
        for _ in 0usize..16usize {
            assert_eq!(hash(false, None).len(), 8usize);
            let h = hash(true, None);
            assert_eq!(h.len(), 8usize);
            assert!(!h.starts_with('0'));
        }
    }
    #[test]
    fn is_hash_case_ok() {
        let re_uc = Regex::new(r"[A-Z0-9]{8}").unwrap();
        let re_lc = Regex::new(r"[a-z0-9]{8}").unwrap();
        for _ in 0usize..16usize {
            assert!(re_uc.is_match(&hash(false, None)));
            assert!(re_uc.is_match(&hash(false, Some(true))));
            assert!(re_lc.is_match(&hash(false, Some(false))));
        }
    }
    #[test]
    fn is_regex_ok() {
        let mut re = Regex::new("-lt0D60-[\u{0001}-\u{00ff}]{12}").unwrap();
        assert!(re.is_match(&regex("-lt0D60-[\u{0001}-\u{00ff}]{12}".to_owned())));
        re = Regex::new("-AZ5750-[a-zA-Z0-9]{12}").unwrap();
        assert!(re.is_match(&regex("-AZ5750-[a-zA-Z0-9]{12}".to_owned())));
    }

    #[test]
    fn is_regex_ok_2() {
        let s = String::from(r"-DE13D0-[A-Za-z0-9_~\\(\\)\\!\\.\\*-]{12}");
        let re = Regex::new(&s).unwrap();
        assert!(re.is_match(&regex(s)));
    }

    #[test]
    fn is_digit_range_to_hex_ok() {
        let re = Regex::new(r"[A-Z0-9]+").unwrap();
        for _ in 0usize..16usize {
            assert!(re.is_match(&digit_range_transformed_to_hex_without_leading_zero()));
        }
    }
}
