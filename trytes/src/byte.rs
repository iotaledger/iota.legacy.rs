#![allow(dead_code)]

use constants::*;
use mappings::*;


/// Converts a slice of trits to a byte
/// `trits.len()` must be less or equal to `TRITS_PER_BYTE`
pub fn trits_to_byte(trits: &[Trit]) -> u8 {
    assert!(trits.len() <= TRITS_PER_BYTE);

    let mut value: Trit = 0;
    for j in (0..trits.len()).rev() {
        value = value * RADIX + trits[j];
    }

    value as u8
}

/// Converts a byte to `&[Trit]`
pub fn byte_to_trits(bu: u8) -> &'static [Trit; TRITS_PER_BYTE] {
    let b = bu as i8;
    let bpos: usize = (if b < 0 {
                           (b as isize) + (BYTE_TO_TRITS_MAPPINGS.len() as isize)
                       } else {
                           b as isize
                       }) as usize;
    &BYTE_TO_TRITS_MAPPINGS[bpos]
}

#[cfg(test)]
mod test {
    use super::*;
    use string::char_to_trits;
    #[test]
    fn test_char_to_trit() {
        for (i, c) in TRYTE_ALPHABET.iter().enumerate() {
            let ts = TRYTE_TO_TRITS_MAPPINGS[i];
            let m = char_to_trits(*c);

            assert_eq!(&ts, m);

        }
    }


}
