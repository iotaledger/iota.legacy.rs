use trinary::*;
use constants::*;
use multiplex::constants::*;

use std::cmp::min;
use std::ops::Index;
use std::iter::IntoIterator;


pub struct TrinaryDemultiplexer {
    trinaries: Vec<Trinary>,
}

pub struct TrinaryDemultiplexerIter<'a> {
    pos: usize,
    demux: &'a TrinaryDemultiplexer,
}

impl<'a> Index<usize> for TrinaryDemultiplexer {
    type Output = Trinary;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.trinaries[idx]
    }
}

impl<'a> Iterator for TrinaryDemultiplexerIter<'a> {
    type Item = &'a Trinary;
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

/// Demultiplexes a slice of `BCTrit` into separate `Trinary` again.
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

    fn multiplexed_to_disjoint(bct: &[BCTrit]) -> Vec<Trinary> {
        let (l, h) = bct[0];

        let trinary_count = MAX_TRINARIES - min(l.leading_zeros(), h.leading_zeros()) as usize;

        let mut out: Vec<Vec<Trit>> = Vec::with_capacity(trinary_count);
        for _ in 0..trinary_count {
            out.push(Vec::new());
        }

        // Runs a bit-mask on each BCTrit to determine value for current trinary
        for &(l, h) in bct {
            for i in 0..trinary_count {
                let low = (l >> i) & 1;
                let high = (h >> i) & 1;

                if (low, high) == (1, 0) {
                    out[i].push(-1);
                } else if (low, high) == (0, 1) {
                    out[i].push(1);
                } else if (low, high) == (1, 1) {
                    out[i].push(0);
                }
            }

        }

        out.into_iter().map(|v| v.into_iter().collect()).collect()
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;
    use mux::*;

    fn t1() -> Trinary {
        Trinary::from_str("99JMJHGHGFVJHBJHGJLERDTFYGHUSDKJSDSIJO")
            .ok()
            .unwrap()
    }

    fn t2() -> Trinary {
        Trinary::from_str("ASDLKJQLWKEJLASJDFLAKDJFSLDKVJASDFJALK")
            .ok()
            .unwrap()
    }
    fn t3() -> Trinary {
        Trinary::from_str("XYZLKJHHGDUTRHYHQWAEAFDSKJHSDKJFWOEWE9")
            .ok()
            .unwrap()
    }

    #[test]
    fn test_demux_count() {
        let t1 = t1();
        let t2 = t2();
        let t3 = t3();

        let mut multi = TrinaryMultiplexer::new();
        multi.add(&t1).ok();
        multi.add(&t2).ok();
        multi.add(&t3).ok();

        let ex = multi.extract();
        let demux = TrinaryDemultiplexer::new(&ex);

        assert_eq!(multi.len(), demux.len());
    }

    #[test]
    fn test_demux_id() {
        let t1 = t1();
        let t2 = t2();
        let t3 = t3();

        let mut multi = TrinaryMultiplexer::new();
        multi.add(&t1).ok();
        multi.add(&t2).ok();
        multi.add(&t3).ok();

        let ex = multi.extract();
        let demux = TrinaryDemultiplexer::new(&ex);


        for i in 0..multi.len() {
            assert_eq!(multi[i], demux[i]);
        }
    }

    #[test]
    fn test_demux_iter() {
        let t1 = t1();
        let t2 = t2();
        let t3 = t3();

        let mut multi = TrinaryMultiplexer::new();
        multi.add(&t1).ok();
        multi.add(&t2).ok();
        multi.add(&t3).ok();

        let ex = multi.extract();
        let demux = TrinaryDemultiplexer::new(&ex);

        for (a, b) in multi.iter().zip(demux.iter()) {
            assert_eq!(a, b);
        }
    }
}
