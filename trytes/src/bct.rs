use std::iter::FromIterator;

use constants::BCTrit;
use constants::Trit;
use trinary::Trinary;
use trinary::IntoTrits;

/// Converts an `Iterator<Trit>` to an instance of `Trinary`
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
    #[test]
    fn test_trit_bc() {
        let t: Trinary = "H".chars().collect(); // trit: [-1,0,1]
        let bct : Vec<BCTrit> = t.trits();

        let high = usize::max_value();
        let low = usize::min_value();

        assert_eq!(bct, vec![(high, low), (high, high), (low, high)]);

        let tbc: Trinary = bct.iter().cloned().collect();
        assert_eq!(t, tbc);
    }
}
