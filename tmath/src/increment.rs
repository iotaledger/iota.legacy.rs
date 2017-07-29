use trytes::*;

/// Trait for incrementing trinary arrays
pub trait TrinaryIncr {
    /// Take a trinary array, increment it, and return the last index visited.
    fn incr(&mut self) -> usize;
}

impl<'a> TrinaryIncr for &'a mut [BCTrit] {
    fn incr(&mut self) -> usize {
        for i in 0..self.len() {
            let (low, hi) = self[i];
            self[i].0 = hi ^ low;
            self[i].1 = low;
            if hi & !low == 0 {
                return self.len();
            }
        }
        self.len() + 1
    }
}

impl<'a> TrinaryIncr for &'a mut [Trit] {
    fn incr(&mut self) -> usize {
        for i in 0..self.len() {
            self[i] += 1;

            if self[i] > 1 {
                self[i] = -1;
            } else {
                return self.len();
            }
        }
        self.len() + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::*;
    #[test]
    fn test_bct_incr() {
        let t = 'H';
        let i = 'I';
        let mut bct: Vec<BCTrit> = char_to_trits(t).iter().cloned().map(trit_to_bct).collect();
        let it: Vec<Trit> = char_to_trits(i).to_vec();

        bct.as_mut_slice().incr();

        let high = usize::max_value();
        let low = usize::min_value();
        assert_eq!(bct, vec![(high, high), (high, high), (low, high)]);

        assert_eq!(it, bct.into_iter().map(bct_to_trit).collect::<Vec<Trit>>());
    }
    #[test]
    fn test_simple_incr() {
        let t = 'H';
        let i = 'I';
        let mut tt: Vec<Trit> = char_to_trits(t).to_vec();
        let it: Vec<BCTrit> = char_to_trits(i).iter().cloned().map(trit_to_bct).collect();

        tt.as_mut_slice().incr();

        assert_eq!(tt.into_iter().map(trit_to_bct).collect::<Vec<BCTrit>>(), it);
    }
}
