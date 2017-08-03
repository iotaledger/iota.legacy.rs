use trytes::*;
use core::cmp::min;

/*
 * For more information of how the logic of a trinary adder works, see:
 * http://homepage.divms.uiowa.edu/~jones/ternary/arith.shtml
*/

#[inline]
pub fn add_assign(out: &mut [Trit], v: isize) {
    let size = out.len();
    let negative = v < 0;

    let mut value = if negative { -v } else { v };
    let mut res = (0, 0);

    for i in 0..size {
        if value == 0 && res.1 == 0 {
            break;
        }

        let mut trit = ((value + 1) % (constants::RADIX as isize)) as i8 - 1;
        if negative {
            trit = -trit;
        }

        res = trit_full_add(out[i], trit, res.1);
        out[i] = res.0;
        value = (value + 1) / (constants::RADIX as isize);
    }
}

/// Takes in `lh` slice of trits, writes out the sum with `rh` slice.
#[inline]
pub fn add_trits(lh: &[Trit], rh: &mut [Trit]) {
    let mut c = 0;
    for i in 0..min(lh.len(), rh.len()) {
        let (s, d) = trit_full_add(lh[i], rh[i], c);
        c = d;
        rh[i] = s;
    }
}

/// Adds values `a` with `b` with a carry `c`, and returns (sum, carry)
#[inline]
pub fn trit_full_add(a: Trit, b: Trit, c: Trit) -> (Trit, Trit) {
    let s_ab = trit_sum(a, b);
    return (
        trit_sum(s_ab, c),
        (trit_cons(a, b) + trit_cons(s_ab, c)).signum(),
    );
}

/// Returns the consensus of `a` and `b`
#[inline]
pub fn trit_cons(a: Trit, b: Trit) -> Trit {
    if a == b { a } else { 0 }
}

/// Trit sum of `a` and `b`
#[inline]
pub fn trit_sum(a: Trit, b: Trit) -> Trit {
    let s = a + b;
    match s {
        2 => -1,
        -2 => 1,
        _ => s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::*;
    use alloc::string::ToString;
    #[test]
    fn test_simple_add() {
        let t = 'H';
        let i = 'Z';
        let tt: Vec<Trit> = char_to_trits(t).to_vec();
        let mut it: Vec<Trit> = char_to_trits(i).to_vec();

        add_trits(tt.as_slice(), it.as_mut_slice());

        let s: String = it.chunks(TRITS_PER_TRYTE).map(trits_to_char).collect();
        let mut exp: String = "G".to_string();

        assert_eq!(exp, s);

        add_assign(it.as_mut_slice(), 1);
        let o: String = it.chunks(TRITS_PER_TRYTE).map(trits_to_char).collect();
        assert_eq!(t.to_string(), o);

        //assert_eq!(tt.into_iter().map(trit_to_bct).collect::<Vec<BCTrit>>(), it);
    }
}
