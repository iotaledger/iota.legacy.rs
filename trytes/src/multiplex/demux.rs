use types::*;
use constants::*;
use multiplex::constants::*;

use core::cmp::min;
use core::ops::Index;
use core::iter::IntoIterator;
use alloc::Vec;


pub struct TrinaryDemultiplexer {
    trinaries: Vec<Vec<Trit>>,
}

pub struct TrinaryDemultiplexerIter<'a> {
    pos: usize,
    demux: &'a TrinaryDemultiplexer,
}

impl<'a> Index<usize> for TrinaryDemultiplexer {
    type Output = Vec<Trit>;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.trinaries[idx]
    }
}

impl<'a> Iterator for TrinaryDemultiplexerIter<'a> {
    type Item = &'a Vec<Trit>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.demux.trinaries.len() {
            None
        } else {
            let ret = Some(&self.demux.trinaries[self.pos]);
            self.pos += 1;
            ret
        }
    }
}

/// Demultiplexes a slice of `BCTrit` into separate `Vec<Trit>` again.
impl TrinaryDemultiplexer {
    pub fn new(bct: &[BCTrit]) -> Self {
        TrinaryDemultiplexer { trinaries: Self::multiplexed_to_disjoint(bct) }
    }

    pub fn iter(&self) -> TrinaryDemultiplexerIter {
        TrinaryDemultiplexerIter {
            pos: 0,
            demux: &self,
        }
    }

    /// Number of encoded trinaries.
    pub fn len(&self) -> usize {
        self.trinaries.len()
    }

    fn multiplexed_to_disjoint(bct: &[BCTrit]) -> Vec<Vec<Trit>> {
        let (l, h) = bct[0];

        let trinary_count = MAX_TRINARIES - min(l.leading_zeros(), h.leading_zeros()) as usize;

        let mut out: Vec<Vec<Trit>> = Vec::with_capacity(trinary_count);
        for i in 0..trinary_count {
            out.push(
                bct.iter()
                    .map(|&(l, h)| {
                        let (low, high) = ((l >> i) & 1, (h >> i) & 1);
                        if (low, high) == (1, 0) {
                            -1
                        } else if (low, high) == (0, 1) {
                            1
                        } else if (low, high) == (1, 1) {
                            0
                        } else {
                            0
                        }
                    })
                    .collect(),
            );
        }

        out
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use multiplex::mux::*;

    static T1: &'static str = "99JMJHGHGFVJHBJHGJLERDTFYGHUSDKJSDSIJO";
    static T2: &'static str = "ASDLKJQLWKEJLASJDFLAKDJFSLDKVJASDFJALK";
    static T3: &'static str = "XYZLKJHHGDUTRHYHQWAEAFDSKJHSDKJFWOEWE9";


    #[test]
    fn test_demux_count() {

        let mut multi = TrinaryMultiplexer::new();
        multi += &T1;
        multi += &T2;
        multi += &T3;


        let ex = multi.extract();
        let demux = TrinaryDemultiplexer::new(&ex);

        assert_eq!(multi.len(), demux.len());
    }

    #[test]
    fn test_demux_id() {
        let mut multi = TrinaryMultiplexer::new();
        multi += &T1;
        multi += &T2;
        multi += &T3;

        let ex = multi.extract();
        let demux = TrinaryDemultiplexer::new(&ex);

        for i in 0..multi.len() {
            assert_eq!(multi.get(i).trits(), demux[i]);
        }
    }

    #[test]
    fn test_demux_iter() {
        let mut multi = TrinaryMultiplexer::new();
        multi += &T1;
        multi += &T2;
        multi += &T3;

        let ex = multi.extract();
        let demux = TrinaryDemultiplexer::new(&ex);

        for (a, b) in multi.iter().zip(demux.iter()) {
            assert_eq!(&a.trits(), b);
        }
    }
}
