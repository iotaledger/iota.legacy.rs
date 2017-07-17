#![allow(dead_code)]

use constants::*;
use alloc::vec::Vec;

/// Converts a slice of trits to an `isize`.
/// Assumes little-endian notation.
pub fn trits2int(trits: &[Trit]) -> isize {
    let mut ret: isize = 0;

    for &t in trits.iter().rev() {
        ret = ret * 3 + (t as isize);
    }

    ret
}

pub fn int2trits(v: isize, size: u8) -> Vec<Trit> {
    let mut ret: Vec<Trit> = Vec::with_capacity(size as usize);
    let negative = v < 0;

    let mut value = if negative { -v } else { v };

    for _ in 0..size {
        if value == 0 {
            break;
        }

        let mut trit = ((value + 1) % (RADIX as isize)) as i8 - 1;
        if negative {
            trit = -trit;
        }

        ret.push(trit);
        value = (value + 1) / (RADIX as isize);
    }

    ret
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_trits2int() {
        let trits: Vec<Trit> = vec![0, 1, -1, 1, 1, -1, -1, 1, 1, 0, 0, 1, 0, 1, 1];
        assert_eq!(trits2int(&trits), 6562317);
    }

    #[test]
    fn test_int2trits_1() {

        let trits: Vec<Trit> = vec![0, 1, -1, 1, 1, -1, -1, 1, 1, 0, 0, 1, 0, 1, 1];
        let conv = int2trits(6562317, 15);

        assert_eq!(trits, conv);
    }

    #[test]
    fn test_int2trits_2() {
        let trits: Vec<Trit> = vec![-1, 1, 0, 1, -1, -1, -1];
        let conv = int2trits(-1024, 7);

        assert_eq!(trits, conv);
    }
}
