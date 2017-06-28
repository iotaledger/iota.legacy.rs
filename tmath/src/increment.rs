use trytes::*;

/// Trait for incrementing trinary arrays
pub trait TrinaryIncr {
    fn incr(&mut self);
}

impl<'a> TrinaryIncr for &'a mut [BCTrit] {
    fn incr(&mut self) {
        for i in 0..self.len() {
            let (low, hi) = self[i];
            self[i].0 = hi ^ low;
            self[i].1 = low;
            if hi & !low == 0 {
                break;
            }
        }
    }
}

impl<'a> TrinaryIncr for &'a mut [Trit] {
    fn incr(&mut self) {
        for i in 0..self.len() {
            self[i] += 1;

            if self[i] > 1 {
                self[i] = -1;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use collections::*;
    #[test]
    fn test_bct_incr() {
        let t: Trinary = "H".chars().collect(); // trit: [-1,0,1]
        let i: Trinary = "I".chars().collect(); // trit: [-1,0,1]
        let mut bct: Vec<BCTrit> = t.trits();

        bct.as_mut_slice().incr();

        let high = usize::max_value();
        let low = usize::min_value();
        assert_eq!(bct, vec![(high, high), (high, high), (low, high)]);

        let tbc: Trinary = bct.iter().cloned().collect();
        assert_eq!(i, tbc);
    }
    #[test]
    fn test_simple_incr() {
        let t: Trinary = "H".chars().collect(); // trit: [-1,0,1]
        let i: Trinary = "I".chars().collect(); // trit: [-1,0,1]
        let mut bct: Vec<Trit> = t.trits();

        bct.as_mut_slice().incr();

        let tbc: Trinary = bct.iter().cloned().collect();
        assert_eq!(i, tbc);
    }
}
