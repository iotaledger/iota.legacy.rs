use trytes::*;
use collections::String;

pub fn get_pow_check(min_weight: usize) -> impl Fn(&[BCTrit]) -> u32 {
    move |&state| {
        let index;
        let mut nonce_probe: usize = usize::max_value();
        for i in (HASH_LENGTH - min_weight)..HASH_LENGTH {
            nonce_probe &= !(state[i].0 ^ state[i].1);
            if nonce_probe == 0 {
                break;
            }
        }
        if nonce_probe != 0 {
            nonce_probe.trailing_zeros() + 1
        } else {
            0
        }
    }
}
