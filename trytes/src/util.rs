use constants::*;
use mappings::*;

/// Converts a tryte to `TRITS_PER_TRYTE` trits
pub fn tryte_to_trits(trit: char) -> [Trit; TRITS_PER_TRYTE] {
    TRYTE_TO_TRITS_MAPPINGS[TRYTE_ALPHABET.find(trit).unwrap()]
}

/// Converts a slice of trits to a byte
/// `trits.len()` must be less or equal to `TRITS_PER_BYTE`
pub fn trits_to_byte(tritss: &[Trit]) -> u8 {
    assert!(tritss.len() <= TRITS_PER_BYTE);

    let trits: Vec<Trit> = tritss.iter().cloned().rev().collect();

    let mut value: Trit = 0;
    for j in trits {
        value = value * RADIX + j;
    }

    value as u8
}

/// Converts a byte to `Vec<Trit>`
pub fn byte_to_trits(bu: u8) -> &'static [Trit; TRITS_PER_BYTE] {
    let b = bu as i8;
    let bpos: usize = (if b < 0 {
                           (b as isize) + (BYTE_TO_TRITS_MAPPINGS.len() as isize)
                       } else {
                           b as isize
                       }) as usize;
    &BYTE_TO_TRITS_MAPPINGS[bpos]
}

/// Converts a slice of trits to a tryte
/// `trits.len()` must be less or equal to `TRITS_PER_TRYTE`
pub fn trits_to_char(trits: &[Trit]) -> char {
    assert!(trits.len() <= TRITS_PER_TRYTE);
    match TRYTE_TO_TRITS_MAPPINGS.iter().position(|&x| x == trits) {
        Some(p) => TRYTE_ALPHABET.chars().nth(p).unwrap(),
        None => '-',
    }
}

#[cfg(test)]
mod test {
    use trinary::*;
    use super::*;
    #[test]
    fn test_char_to_trit() {
        for (i, c) in TRYTE_ALPHABET.chars().enumerate() {
            let ts = TRYTE_TO_TRITS_MAPPINGS[i];
            let m = tryte_to_trits(c);

            assert_eq!(ts, m);

        }
    }


    #[test]
    fn bytes_to_trits() {
        let bytes: [u8; 6] = [20, 25, -14_i8 as u8, -2_i8 as u8, 83, 1];
        let exp: Trinary = "TJHLYYRAD".chars().collect();
        let trinary = Trinary::new(bytes.iter().cloned().collect(), 27);

        assert_eq!(exp, trinary);

    }
}
