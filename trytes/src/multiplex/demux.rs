use constants::*;
use multiplex::constants::*;

use core::cmp::min;
use core::iter::ExactSizeIterator;


#[derive(PartialEq, Clone, Eq)]
pub struct TrinaryDemultiplexer<'a> {
    trits: &'a [BCTrit],
}

#[derive(PartialEq, Clone, Eq)]
pub struct TrinaryDemultiplexerIter<'a> {
    pos: usize,
    idx: usize,
    demux: &'a TrinaryDemultiplexer<'a>,
}

impl<'a> Iterator for TrinaryDemultiplexerIter<'a> {
    type Item = Trit;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.demux.trits.len() {
            None
        } else {
            let (l, h) = self.demux.trits[self.pos];

            let (low, high) = ((l >> self.idx) & 1, (h >> self.idx) & 1);
            self.pos += 1;

            Some(if (low, high) == (1, 0) {
                -1
            } else if (low, high) == (0, 1) {
                1
            } else if (low, high) == (1, 1) {
                0
            } else {
                0
            })
        }
    }
}

impl<'a> ExactSizeIterator for TrinaryDemultiplexerIter<'a> {
    fn len(&self) -> usize {
        self.demux.trits.len()
    }
}

/// Demultiplexes a slice of `BCTrit` into separate `Iterator<Item Trit>` again.
impl<'a> TrinaryDemultiplexer<'a> {
    pub fn new(bct: &'a [BCTrit]) -> Self {
        TrinaryDemultiplexer { trits: bct }
    }

    pub fn get(&self, idx: usize) -> TrinaryDemultiplexerIter {
        TrinaryDemultiplexerIter {
            pos: 0,
            idx: idx,
            demux: &self,
        }
    }

    /// Number of encoded trinaries.
    pub fn len(&self) -> usize {
        let (l, h) = self.trits[0];
        MAX_TRINARIES - min(l.leading_zeros(), h.leading_zeros()) as usize
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use multiplex::mux::*;
    use string::char_to_trits;
    use alloc::Vec;

    static T1: &'static str = "99JMJHGHGFVJHBJHGJLERDTFYGHUSDKJSDSIJO";
    static T2: &'static str = "ASDLKJQLWKEJLASJDFLAKDJFSLDKVJASDFJALK";
    static T3: &'static str = "XYZLKJHHGDUTRHYHQWAEAFDSKJHSDKJFWOEWE9";


    #[test]
    fn test_demux_count() {
        let t1: Vec<Trit> = T1.chars().flat_map(char_to_trits).cloned().collect();
        let t2: Vec<Trit> = T2.chars().flat_map(char_to_trits).cloned().collect();
        let t3: Vec<Trit> = T3.chars().flat_map(char_to_trits).cloned().collect();

        let mut multi = TrinaryMultiplexer::new();
        multi += &t1;
        multi += &t2;
        multi += &t3;


        let mut ex = vec![(0,0); multi.len_trits()];
        multi.extract(&mut ex);
        let demux = TrinaryDemultiplexer::new(&ex);

        assert_eq!(multi.len(), demux.len());
    }

    #[test]
    fn test_demux_id() {
        let t1: Vec<Trit> = T1.chars().flat_map(char_to_trits).cloned().collect();
        let t2: Vec<Trit> = T2.chars().flat_map(char_to_trits).cloned().collect();
        let t3: Vec<Trit> = T3.chars().flat_map(char_to_trits).cloned().collect();

        let mut multi = TrinaryMultiplexer::new();
        multi += &t1;
        multi += &t2;
        multi += &t3;

        let mut ex = vec![(0,0); multi.len_trits()];
        multi.extract(&mut ex);
        let demux = TrinaryDemultiplexer::new(&ex);

        for i in 0..multi.len() {
            assert_eq!(multi.get(i).to_vec(), demux.get(i).collect::<Vec<Trit>>());
        }
    }
}
