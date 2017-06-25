use hamming::weight;
use num::BigUint;

const identity: [usize; mem::size_of::<usize>()] = (0..mem::size_of::<usize>())
    .iter()
    .map(|i| 1 << i)
    .collect();

pub fn transpose(state: &[BCTrit]) {
    let mut output: [(BigUint, BigUint); mem::size_of::<usize>()];
    let bytes: [u8; state.len()] = [0u8; state.len()];
    for i in 0..mem::size_of::<usize>() {
        output[i].0 = BigUint::from_bytes_be(&bytes);
        output[i].1 = BigUint::from_bytes_be(&bytes);
        for j in 0..state.len() {
            if state[j].0 & identity[i] != 0 {
                output[i].0 |= 1 << j;
            }
            if state[j].1 & identity[i] != 0 {
                output[i].1 |= 1 << j;
            }
        }
    }
    output
}

pub fn get_checksum_check(length: usize) -> impl Fn(&[BCTrit]) -> u32 {
    move |&state| {
        let checks = transpose(state);
        let mut out = -1;
        for i in 0..mem::size_of::<usize>() {
            if hamming::weight(checks[i].0) == hamming::weight(checks[i].1) {
                out = i;
                break;
            }
        }
        out
    }
}
