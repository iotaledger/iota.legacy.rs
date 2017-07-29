use trytes::*;

#[inline]
pub fn add_trits(lh: &[Trit], rh: &mut [Trit]) {
    let mut c = 0;
    for i in 0..rh.len() {
        let (s, d) = trit_full_add(lh[i], rh[i], c);
        c = d;
        rh[i] = s;
    }
}

#[inline]
pub fn trit_full_add(a: Trit, b: Trit, c: Trit) -> (Trit, Trit) {
    let s_ab = trit_sum(a, b);
    return (
        trit_sum(s_ab, c),
        (trit_cons(a, b) + trit_cons(s_ab, c)).signum(),
    );
}

#[inline]
pub fn trit_cons(a: Trit, b: Trit) -> Trit {
    if a == b { a } else { 0 }
}

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
        let exp: String = "G".to_string();

        assert_eq!(exp, s);

        //assert_eq!(tt.into_iter().map(trit_to_bct).collect::<Vec<BCTrit>>(), it);
    }
}
