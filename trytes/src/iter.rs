use constants::*;
use trinary::*;
use util::*;

/// Constructs an iterator over all trits in a `&Trinary`
impl<'a> IntoIterator for &'a Trinary {
    type Item = Trit;
    type IntoIter = TrinaryIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TrinaryIterator {
            trinary: self,
            index: 0,
            emitted: 0,
            trits_index: 0,
        }
    }
}

pub struct TrinaryIterator<'a> {
    trinary: &'a Trinary,
    index: usize,
    emitted: usize,
    trits_index: usize,
}

impl<'a> Iterator for TrinaryIterator<'a> {
    type Item = Trit;
    fn next(&mut self) -> Option<Trit> {
        if self.emitted == self.trinary.len_trits() {
            return None;
        }

        if self.trits_index >= TRITS_PER_BYTE {
            self.index += 1;
            self.trits_index = 0;
        }

        let trit = byte_to_trits(self.trinary.bytes()[self.index])[self.trits_index];
        self.trits_index += 1;
        self.emitted += 1;

        Some(trit)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use super::*;
    #[test]
    fn test_iter() {
        let trinary = Trinary::from_str("ABCDASDQWEDASDAKJFASD9").unwrap();
        let trits_ex : Vec<Trit> = trinary.trits();
        let trits_iter : Vec<Trit> = (&trinary).into_iter().collect();

        assert_eq!(trits_ex, trits_iter);
    }
}
