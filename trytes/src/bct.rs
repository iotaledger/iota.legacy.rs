use alloc::Vec;
use core::iter::FromIterator;

use constants::BCTrit;
use constants::Trit;
use trinary::{Trinary, Offset};

impl<'a> Offset for &'a mut [BCTrit] {
    fn offset(&mut self) {
        self[0].0 = 0b1101101101101101101101101101101101101101101101101101101101101101;
        self[0].1 = 0b1011011011011011011011011011011011011011011011011011011011011011;
        self[1].0 = 0b1111000111111000111111000111111000111111000111111000111111000111;
        self[1].1 = 0b1000111111000111111000111111000111111000111111000111111000111111;
        self[2].0 = 0b0111111111111111111000000000111111111111111111000000000111111111;
        self[2].1 = 0b1111111111000000000111111111111111111000000000111111111111111111;
        self[3].0 = 0b1111111111000000000000000000000000000111111111111111111111111111;
        self[3].1 = 0b0000000000111111111111111111111111111111111111111111111111111111;
    }
}

/// Converts an `Iterator<BCTrit>` to an instance of `Trinary`
impl FromIterator<BCTrit> for Trinary {
    fn from_iter<I: IntoIterator<Item = BCTrit>>(iter: I) -> Self {

        let mut trits: Vec<Trit> = Vec::new();

        for t in iter {
            trits.push(bct_to_trit(t));
        }

        trits.into_iter().collect()
    }
}

#[inline(always)]
pub fn trit_to_bct(t: Trit) -> BCTrit {
    let high = usize::max_value();
    let low = usize::min_value();
    match t {
        -1 => (high, low),
        1 => (low, high),
        0 => (high, high),
        _ => panic!("Invalid Trit: {:?}", t),
    }
}

#[inline(always)]
pub fn bct_to_trit(t: BCTrit) -> Trit {
    let high = usize::max_value();
    let low = usize::min_value();

    if t == (high, low) {
        return -1;
    } else if t == (low, high) {
        return 1;
    } else if t == (high, high) {
        return 0;
    } else {
        panic!("Invalid BCTrit: {:?}", t);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use trinary::IntoTrits;
    use alloc::*;

    #[test]
    fn test_trit_bc() {
        let t: Trinary = "H".chars().collect(); // trit: [-1,0,1]
        let bct: Vec<BCTrit> = t.trits();

        let high = usize::max_value();
        let low = usize::min_value();

        assert_eq!(bct, vec![(high, low), (high, high), (low, high)]);

        let tbc: Trinary = bct.iter().cloned().collect();
        assert_eq!(t, tbc);
    }
}
