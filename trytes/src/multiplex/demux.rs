use types::*;
use constants::*;
use multiplex::constants::*;

use core::cmp::min;
use core::ops::Index;
use core::iter::IntoIterator;
use alloc::Vec;


pub struct TrinaryDemultiplexer<T>
where
    T: FromTrits<Trit>,
{
    trinaries: Vec<T>,
}

pub struct TrinaryDemultiplexerIter<'a, T>
where
    T: FromTrits<Trit> + 'a,
{
    pos: usize,
    demux: &'a TrinaryDemultiplexer<T>,
}

impl<'a, T> Index<usize> for TrinaryDemultiplexer<T>
where
    T: FromTrits<Trit>,
{
    type Output = T;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.trinaries[idx]
    }
}

impl<'a, T> Iterator for TrinaryDemultiplexerIter<'a, T>
where
    T: FromTrits<Trit> + 'a,
{
    type Item = &'a T;
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
impl<T> TrinaryDemultiplexer<T>
where
    T: FromTrits<Trit>,
{
    pub fn new(bct: &[BCTrit]) -> Self {
        TrinaryDemultiplexer { trinaries: Self::multiplexed_to_disjoint(bct) }
    }

    pub fn iter(&self) -> TrinaryDemultiplexerIter<T> {
        TrinaryDemultiplexerIter {
            pos: 0,
            demux: &self,
        }
    }

    /// Number of encoded trinaries.
    pub fn len(&self) -> usize {
        self.trinaries.len()
    }

    fn multiplexed_to_disjoint(bct: &[BCTrit]) -> Vec<T> {
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

        out.into_iter()
            .map(|v| T::from_trits(&v).ok().unwrap())
            .collect()
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
        let demux = TrinaryDemultiplexer::<Vec<Trit>>::new(&ex);

        assert_eq!(multi.len(), demux.len());
    }

    #[test]
    fn test_demux_id() {
        let mut multi = TrinaryMultiplexer::new();
        multi += &T1;
        multi += &T2;
        multi += &T3;

        let ex = multi.extract();
        let demux = TrinaryDemultiplexer::<Vec<Trit>>::new(&ex);

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
        let demux = TrinaryDemultiplexer::<Vec<Trit>>::new(&ex);

        for (a, b) in multi.iter().zip(demux.iter()) {
            assert_eq!(&a.trits(), b);
        }
    }
}
