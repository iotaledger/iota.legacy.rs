use trinary::*;
use constants::*;

use std::cmp::min;
use std::ops::AddAssign;
use std::ops::Index;
use std::iter::IntoIterator;

#[cfg(target_pointer_width = "16")]
const MAX_TRINARIES: usize = 16;
#[cfg(target_pointer_width = "32")]
const MAX_TRINARIES: usize = 32;
#[cfg(target_pointer_width = "64")]
const MAX_TRINARIES: usize = 64;
#[cfg(target_pointer_width = "128")]
const MAX_TRINARIES: usize = 128;

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

impl<'a> TrinaryMultiplexer<'a> {
    pub fn new() -> Self {
        TrinaryMultiplexer { trinaries: Vec::new() }
    }

    pub fn add(&mut self, t: &'a Trinary) -> usize {
        if self.trinaries.len() > 0 {
            assert!(
                t.len_trits() == self.trinaries[0].len_trits(),
                "Different Trinary trit lengths are not supported."
            );
        }

        self.trinaries.push(t);
        self.trinaries.len()
    }

    pub fn count(&self) -> usize {
        self.trinaries.len()
    }

    pub fn extract(&self) -> Vec<BCTrit> {
        Self::disjoint_to_multiplexed(self.trinaries.as_slice())
    }

    fn disjoint_to_multiplexed(ts: &[&Trinary]) -> Vec<BCTrit> {
        let mut out : Vec<BCTrit> = Vec::new();
        let trits : Vec<Vec<Trit>> = ts.iter().map(|&t| t.trits()).collect();
        let trit_count = trits[0].len();
        let trinary_count = ts.len();

        for i in 0..trit_count{
            let (mut low, mut high) : BCTrit = (0, 0);
            
            for j in 0..trinary_count {
                match trits[j][i] {
                    -1 => {
                        low |= 1 << j;
                    },
                    1 => {
                        high |= 1 << j;
                    },
                    0 => {
                        low |= 1 << j;
                        high |= 1 << j;
                    },
                    _ => panic!("Invalid Trit.")
                }
            }
            
            out.push((low, high));
        }

        out
    }
}

// ============================================================================

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

    pub fn count(&self) -> usize {
        self.trinaries.len()
    }

    fn multiplexed_to_disjoint(bct: &[BCTrit]) -> Vec<Trinary> {
        let (l, h) = bct[0];

        let trinary_count = MAX_TRINARIES - min(l.leading_zeros(), h.leading_zeros()) as usize;

        let mut out: Vec<Vec<Trit>> = Vec::with_capacity(trinary_count);
        for _ in 0..trinary_count {
            out.push(Vec::new());
        }

        for &(l,h) in bct {
            for i in 0..trinary_count {
                let low = (l >> i) & 1;
                let high = (h >> i) & 1;

                if (low,high) == (1,0) {
                    out[i].push(-1);
                }
                else if (low,high) == (0,1) {
                    out[i].push(1);
                }
                else if (low,high) == (1,1) {
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

    #[test]
    fn test_multiplex() {
        let t1 = Trinary::from_str("ABC").ok().unwrap();
        let t2 = Trinary::from_str("XYZ").ok().unwrap();

        let mut multi = TrinaryMultiplexer::default();

        multi += &t1;
        multi += &t2;

        let t1t : Vec<Trit> = t1.trits();
        let t2t : Vec<Trit> = t2.trits();

        let ex = multi.extract();
        let demux = TrinaryDemultiplexer::new(&ex);

        assert_eq!(t1, demux[0]);
        assert_eq!(t2, demux[1]);
    }
}
