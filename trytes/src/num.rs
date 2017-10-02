#![allow(dead_code)]

use constants::*;

/// Converts a slice of trits to an `isize`.
/// Assumes little-endian notation.
pub fn trits2int(trits: &[Trit]) -> i64 {
    let mut ret: i64 = 0;

    for &t in trits.iter().rev() {
        ret = ret * 3 + (t as i64);
    }

    ret
}

pub fn int2trits(v: i64, out: &mut [Trit]) {
    let size = out.len();
    let negative = v < 0;

    let mut value = if negative { -v } else { v };


    for i in 0..size {
        if value == 0 {
            break;
        }

        let mut trit = ((value + 1) % (RADIX as i64)) as i8 - 1;
        if negative {
            trit = -trit;
        }

        out[i] = trit;
        value = (value + 1) / (RADIX as i64);
    }
}

/// Given an integer `input`, it rounds up to the nearest multiple of `TRITS_PER_TRYTE`
pub fn round_third(input: i64) -> i64 {
    let rem = input % TRITS_PER_TRYTE as i64;
    if rem == 0 {
        input
    } else {
        input + TRITS_PER_TRYTE as i64 - rem
    }
}

/// given an integer `i`, returns the minimum number of trits required to convert to balanced
/// ternary
pub fn min_trits(i: i64 ) -> usize {
    let mut num: usize = 1;
    let v_abs = i.wrapping_abs();
    while v_abs > (RADIX as i64).pow(num as u32 - 1) {
        num += 1;
    }
    num
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::vec::Vec;

    #[test]
    fn test_min_trits() {
        let tests: [i64; 5] = [0, 1, -1, 3, 8];
        let expect: [usize; 5] = [1, 1, 1, 2, 3];
        for (&t, &e) in tests.iter().zip(expect.iter()) {
            assert_eq!(min_trits(t), e);
        }
    }

    #[test]
    fn test_trits2int() {
        let trits: Vec<Trit> = vec![0, 1, -1, 1, 1, -1, -1, 1, 1, 0, 0, 1, 0, 1, 1];
        assert_eq!(trits2int(&trits), 6562317);
    }

    #[test]
    fn test_int2trits_1() {

        let trits: Vec<Trit> = vec![0, 1, -1, 1, 1, -1, -1, 1, 1, 0, 0, 1, 0, 1, 1];
        let mut out = [0 as Trit; 15];
        int2trits(6562317, &mut out);

        assert_eq!(trits, out);
    }

    #[test]
    fn test_int2trits_2() {
        let trits: Vec<Trit> = vec![-1, 1, 0, 1, -1, -1, -1];
        let mut out = [0 as Trit; 7];
        int2trits(-1024, &mut out);

        assert_eq!(trits, out);
    }
}
