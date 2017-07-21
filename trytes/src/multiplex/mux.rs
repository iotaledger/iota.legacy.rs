use constants::*;
use multiplex::constants::*;

use core::ops::AddAssign;

pub struct TrinaryMultiplexer<'a> {
    trinaries: [&'a [Trit]; MAX_TRINARIES],
    idx: usize,
}

impl<'a> AddAssign<&'a [Trit]> for TrinaryMultiplexer<'a> {
    fn add_assign(&mut self, trinary: &'a [Trit]) {
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
    type Item = &'a [Trit];
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

static EMPTY_MUX: [Trit; 1] = [0];

/// A helper class to encode multiple `Trinary` in a single vector of `BCTrit`
/// Essentially: for the i-th `Trit` in a `Trinary`, the i-th bit of the `BCTrit` is set to the corresponding bit representation.
impl<'a> TrinaryMultiplexer<'a> {
    pub fn new() -> Self {
        TrinaryMultiplexer {
            trinaries: [&EMPTY_MUX; MAX_TRINARIES],
            idx: 0,
        }
    }

    /// Adds a `Trinary` to the multiplexer.
    /// This will `assert!` that `len() != max_len()`
    pub fn add(&mut self, t: &'a [Trit]) -> usize {
        assert!(
            self.idx <= Self::max_len(),
            "Maximum supported number of trinaries already being multiplexed."
        );

        assert!(
            self.idx == 0 || (self.trinaries.len() == 0 || t.len() == self.trinaries[0].len()),
            "Different Trinary trit lengths are not supported."
        );

        self.trinaries[self.idx] = t;
        self.idx += 1;

        self.idx - 1
    }

    pub fn get(&self, idx: usize) -> &'a [Trit] {
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
        self.idx - 1
    }

    pub fn len_trits(&self) -> usize {
        assert!(
            self.idx > 0,
            "No multiplex target was added before calling len_trits()."
        );
        self.trinaries[0].len()
    }

    /// Performs the actual multiplexing.
    /// Returns a multiplexed vector of `BCTrit`.
    pub fn extract(&self, out: &mut [BCTrit]) {
        assert!(
            out.len() == self.len_trits(),
            "Extraction target is not of correct length."
        );
        Self::disjoint_to_multiplexed(self.len(), &self.trinaries, out);
    }

    /// Internal method for the multiplexing logic.
    fn disjoint_to_multiplexed(trinary_count: usize, trits: &[&[Trit]], out: &mut [BCTrit]) {
        let trit_count = trits[0].len();

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

            out[i] = (low, high);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use string::*;
    use alloc::Vec;

    // demux is already testing the other functionality

    #[test]
    #[should_panic]
    fn mux_should_assert_same_trit_size() {
        let t1: Vec<Trit> = "ABCD".chars().flat_map(char_to_trits).cloned().collect();
        let t2: Vec<Trit> = "A".chars().flat_map(char_to_trits).cloned().collect();

        let mut mux = TrinaryMultiplexer::default();

        mux += &t1;
        mux += &t2;
    }
}
