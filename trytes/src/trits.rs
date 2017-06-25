use collections::Vec;
use core::iter::FromIterator;

use trinary::*;
use constants::*;
use util::*;

/// Converts an `Iterator<Trit>` to an instance of `Trinary`
impl FromIterator<Trit> for Trinary {
    fn from_iter<I: IntoIterator<Item = Trit>>(iter: I) -> Self {
        let mut trits: Vec<Trit> = Vec::new();
        let mut bytes: Vec<u8> = Vec::new();
        let mut n: usize = 0;

        for t in iter {
            n += 1;
            trits.push(t);

            if trits.len() >= TRITS_PER_BYTE {
                bytes.push(trits_to_byte(&trits[0..TRITS_PER_BYTE]));
                trits = trits.split_off(TRITS_PER_BYTE);
            }
        }

        bytes.push(trits_to_byte(trits.as_slice()));
        Trinary::new(bytes, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use collections::*;
    use collections::string::ToString;

    #[test]
    fn from_iterator_trit() {
        let trits : Vec<Trit> = vec![0,0,0,-1,0,1,1,0,-1];
        let expected = "9HS";

        let trinary : Trinary = trits.iter().cloned().collect();
        assert_eq!(expected, trinary.to_string());
    }
}
