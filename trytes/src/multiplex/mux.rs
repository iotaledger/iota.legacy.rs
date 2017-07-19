use types::*;
use constants::*;
use multiplex::constants::*;

use core::ops::AddAssign;
use alloc::Vec;

pub struct TrinaryMultiplexer<'a> {
    trinaries: Vec<&'a IntoTrits<Trit>>,
}

impl<'a> AddAssign<&'a IntoTrits<Trit>> for TrinaryMultiplexer<'a> {
    fn add_assign(&mut self, trinary: &'a IntoTrits<Trit>) {
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
    type Item = &'a IntoTrits<Trit>;
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
    pub fn add(&mut self, t: &'a IntoTrits<Trit>) -> usize {
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

    pub fn get(&self, idx: usize) -> &'a IntoTrits<Trit> {
        self.trinaries[idx]
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
    fn disjoint_to_multiplexed(ts: &[&IntoTrits<Trit>]) -> Vec<BCTrit> {
        let trit_count = ts[0].len_trits();
        let trinary_count = ts.len();

        let mut out: Vec<BCTrit> = Vec::with_capacity(trit_count);
        let trits: Vec<Vec<Trit>> = ts.iter().map(|&t| t.trits()).collect();

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

    // demux is already testing the other functionality

    #[test]
    #[should_panic]
    fn mux_should_assert_same_trit_size() {
        let t1: Vec<Trit> = "ABCD".trits();
        let t2: Vec<Trit> = "A".trits();

        let mut mux = TrinaryMultiplexer::default();

        mux += &t1;
        mux += &t2;
    }
}
