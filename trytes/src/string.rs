#[cfg(any(test, feature = "alloc"))]
use alloc::string::String;

use constants::*;
use mappings::TRYTE_TO_TRITS_MAPPINGS;

/// Converts a tryte to `TRITS_PER_TRYTE` trits
#[inline]
pub fn char_to_trits(tryte: char) -> &'static [Trit; TRITS_PER_TRYTE] {
    for i in 0..TRYTE_SPACE {
        if TRYTE_ALPHABET[i] == tryte {
            return &TRYTE_TO_TRITS_MAPPINGS[i];
        }
    }

    &TRYTE_TO_TRITS_MAPPINGS[0]
}

/// Converts a slice of trits to a tryte
/// `trits.len()` must be less or equal to `TRITS_PER_TRYTE`
#[inline]
pub fn trits_to_char(trits: &[Trit]) -> char {
    assert!(trits.len() <= TRITS_PER_TRYTE);
    match TRYTE_TO_TRITS_MAPPINGS.iter().position(|&x| x == trits) {
        Some(p) => TRYTE_ALPHABET[p],
        None => '-',
    }
}

#[cfg(any(test, feature = "alloc"))]
pub fn trits_to_string(t: &[Trit]) -> Option<String> {
    if t.len() % 3 != 0 {
        return None;
    }

    Some(t.chunks(TRITS_PER_TRYTE).map(trits_to_char).collect())
}


#[cfg(test)]
mod tests {
    use super::*;
    use alloc::Vec;

    #[test]
    fn fromtostr_test1() {
        let trytes = "UYSSM9KIH";

        let as_trits : Vec<Trit> = trytes.chars().flat_map(char_to_trits).cloned().collect();
        let as_string = trits_to_string(as_trits.as_slice()).unwrap();
        assert_eq!(as_string, trytes);
    }

    #[test]
    fn fromtostr_test2() {
        let trytes = "LCUNQ99HCQM9HSATSQOPOWXXKGKDVZSEKVWJZRGWFRVLEQ9XG9INIPKAM9BQVGCIRNPZOS9LUZRBHB9P";

        let as_trits : Vec<Trit> = trytes.chars().flat_map(char_to_trits).cloned().collect();
        let as_string = trits_to_string(as_trits.as_slice()).unwrap();
        assert_eq!(as_string, trytes);
    }

    #[test]
    fn fromtostr_test3() {
        let trytes = "RSWWSFXPQJUBJROQBRQZWZXZJWMUBVIVMHPPTYSNW9YQI";

        let as_trits : Vec<Trit> = trytes.chars().flat_map(char_to_trits).cloned().collect();
        let as_string = trits_to_string(as_trits.as_slice()).unwrap();
        assert_eq!(as_string, trytes);
    }
}
