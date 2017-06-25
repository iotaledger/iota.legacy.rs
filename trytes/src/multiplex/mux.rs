use trinary::*;
use constants::*;
use multiplex::constants::*;

use core::ops::AddAssign;
use core::ops::Index;
use collections::Vec;

pub struct TrinaryMultiplexer<'a> {
    trinaries: Vec<&'a Trinary>,
}

impl<'a> Index<usize> for TrinaryMultiplexer<'a> {
    type Output = Trinary;
    fn index(&self, idx: usize) -> &Self::Output {
        self.trinaries[idx]
    }
}

impl<'a> AddAssign<&'a Trinary> for TrinaryMultiplexer<'a> {
    fn add_assign(&mut self, trinary: &'a Trinary) {
        self.add(trinary);
    }
}

impl<'a> Default for TrinaryMultiplexer<'a> {
    fn default() -> Self {
        TrinaryMultiplexer::new()
    }
}

pub struct TrinaryMultiplexerIter<'a> {
    pos: usize,
    mux: &'a TrinaryMultiplexer<'a>,
}

impl<'a> Iterator for TrinaryMultiplexerIter<'a> {
    type Item = &'a Trinary;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.mux.trinaries.len() {
            None
        } else {
            let ret = Some(self.mux.trinaries[self.pos]);
            self.pos += 1;
            ret
        }
    }
}

/// A helper class to encode multiple `Trinary` in a single vector of `BCTrit`
/// Essentially: for the i-th `Trit` in a `Trinary`, the i-th bit of the `BCTrit` is set to the corresponding bit representation.
impl<'a> TrinaryMultiplexer<'a> {
    pub fn new() -> Self {
        TrinaryMultiplexer { trinaries: Vec::new() }
    }

    /// Adds a `Trinary` to the multiplexer.
    /// This will `assert!` that `len() != max_len()`
    pub fn add(&mut self, t: &'a Trinary) -> usize {
        assert!(
            self.trinaries.len() < Self::max_len(),
            "Maximum supported number of trinaries already being multiplexed."
        );

        assert!(
            self.trinaries.len() == 0 || t.len_trits() == self.trinaries[0].len_trits(),
            "Different Trinary trit lengths are not supported."
        );

        self.trinaries.push(t);
        self.trinaries.len()
    }

    pub fn iter(&self) -> TrinaryMultiplexerIter {
        TrinaryMultiplexerIter { pos: 0, mux: &self }
    }

    /// The maximum number of `Trinary` that can be multiplexed on this architecture.
    pub fn max_len() -> usize {
        MAX_TRINARIES
    }

    /// Returns how many `Trinary` are currently being multiplexed.
    pub fn len(&self) -> usize {
        self.trinaries.len()
    }

    /// Performs the actual multiplexing.
    /// Returns a multiplexed vector of `BCTrit`.
    pub fn extract(&self) -> Vec<BCTrit> {
        Self::disjoint_to_multiplexed(self.trinaries.as_slice())
    }

    /// Internal method for the multiplexing logic.
    fn disjoint_to_multiplexed(ts: &[&Trinary]) -> Vec<BCTrit> {
        let mut out: Vec<BCTrit> = Vec::new();
        let trits: Vec<Vec<Trit>> = ts.iter().map(|&t| t.trits()).collect();
        let trit_count = trits[0].len();
        let trinary_count = ts.len();

        for i in 0..trit_count {
            let (mut low, mut high): BCTrit = (0, 0);

            for j in 0..trinary_count {
                match trits[j][i] {
                    -1 => {
                        low |= 1 << j;
                    }
                    1 => {
                        high |= 1 << j;
                    }
                    0 => {
                        low |= 1 << j;
                        high |= 1 << j;
                    }
                    _ => panic!("Invalid Trit."),
                }
            }

            out.push((low, high));
        }

        out
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::str::FromStr;
    
    // demux is already testing the other functionality

    #[test]
    #[should_panic]
    fn mux_should_assert_same_trit_size() {

        let t1 = Trinary::from_str("ABCD").ok().unwrap();
        let t2 = Trinary::from_str("A").ok().unwrap();

        let mut mux = TrinaryMultiplexer::default();

        mux += &t1;
        mux += &t2;
    }
}
