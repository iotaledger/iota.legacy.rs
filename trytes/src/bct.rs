use constants::BCTrit;
use constants::Trit;

#[inline(always)]
pub fn trit_to_bct(t: Trit) -> BCTrit {
    let high = usize::max_value();
    let low = usize::min_value();
    match t {
        -1 => (high, low),
        1 => (low, high),
        0 => (high, high),
        _ => panic!("Invalid Trit: {:?}", t),
    }
}

#[inline(always)]
pub fn bct_to_trit(t: BCTrit) -> Trit {
    let high = usize::max_value();
    let low = usize::min_value();

    if t == (high, low) {
        return -1;
    } else if t == (low, high) {
        return 1;
    } else if t == (high, high) {
        return 0;
    } else {
        panic!("Invalid BCTrit: {:?}", t);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::*;
    use string::char_to_trits;

    #[test]
    fn test_trit_bc() {
        let t = "H";
        let bct: Vec<BCTrit> = t.chars().flat_map(char_to_trits).cloned().map(trit_to_bct).collect();

        let high = usize::max_value();
        let low = usize::min_value();

        assert_eq!(bct, vec![(high, low), (high, high), (low, high)]);
    }
}
