const HASH_LENGTH: usize = 243;
const STATE_LENGTH: usize = HASH_LENGTH * 3;
const TRUTH_TABLE: [i32; 9] = [1, 0, -1, 1, -1, 0, -1, 1, 0];
const NUMBER_OF_ROUNDS: usize = 27;


trait Sponge {
    fn absorb(&self, trits: &[i32], length: usize) -> Self;
    fn squeeze(&self, length: usize) -> Vec<i32>;
}

trait Tansformable {
    fn transform(&self) -> Self;
}

impl Tansformable for [i32; STATE_LENGTH] {
    fn transform(&self) -> Self {
        let mut scratchpad = *self.clone();
        let mut out = *self.clone();
        let mut scratchpad_index: usize = 0;
        let mut scratchpad_index_save: usize;
        let mut round = 0;
        while round < NUMBER_OF_ROUNDS {
            round += 1;
            scratchpad.clone_from_slice(&out);
            for state_index in 0..STATE_LENGTH {
                scratchpad_index_save = scratchpad_index;
                if scratchpad_index < 365 {
                    scratchpad_index += 364;
                } else {
                    scratchpad_index -= 365;
                };
                out[state_index] = TRUTH_TABLE[(scratchpad[scratchpad_index_save] + scratchpad[scratchpad_index] * 3 +
                 4) as usize];
            }
        }
        out
    }
}

impl Sponge for [i32; STATE_LENGTH] {
    fn absorb(&self, trits: &[i32], length: usize) -> Self {
        let mut len = length;
        let mut state = *self.clone();
        let mut offset = 0;
        while len > HASH_LENGTH {
            state[0..HASH_LENGTH].clone_from_slice(&trits[offset..(offset + HASH_LENGTH)]);
            state = state.transform();
            offset += HASH_LENGTH;
            len -= HASH_LENGTH;
        }
        state[0..len].clone_from_slice(&trits[offset..offset + len]);
        state.transform()
    }
    fn squeeze(&self, length: usize) -> Vec<i32> {
        let mut len = length;
        let mut state = *self.clone();
        let mut out = vec![0i32; length];
        let mut offset = 0;
        while len > HASH_LENGTH {
            out[offset..(offset + HASH_LENGTH)].clone_from_slice(&state[0..HASH_LENGTH]);
            state = state.transform();
            offset += HASH_LENGTH;
            len -= HASH_LENGTH;
        }
        out[offset..len].clone_from_slice(&state[0..len]);
        out
    }
}

/*
fn state() -> [i32; STATE_LENGTH] {
    [0i32; STATE_LENGTH]
}
*/

#[cfg(test)]
mod tests {
    use super::{Sponge, HASH_LENGTH, STATE_LENGTH};
    const TRITS: [i32; STATE_LENGTH] =
        [1, 1, 1, 1, 1, 0, 0, 0, 1, -1, 0, -1, 0, 0, 0, 0, 1, 0, 1, 0, 1, -1, 1, 1, 1, 1, 1, 1, 1,
         0, 1, 0, -1, 1, 0, -1, -1, 1, 1, 0, 1, 1, 1, 1, 0, -1, 0, 1, 0, -1, 0, 0, 1, 1, 0, 0, 0,
         0, -1, 0, 0, 1, 0, -1, 1, 0, 1, 1, 1, -1, 1, 1, 1, 1, 1, 0, 1, 1, -1, 1, 1, 1, -1, 0, 1,
         0, 1, -1, -1, 0, 1, 1, 0, 1, 0, -1, 1, 1, 1, 0, 1, 0, 1, 1, -1, -1, -1, -1, -1, 1, -1, 0,
         1, 1, 0, 1, 0, -1, -1, -1, -1, 0, 0, 1, 1, 0, 0, 1, -1, 0, 1, 1, 1, -1, 0, -1, 0, -1, 1,
         1, 1, 0, 1, 1, -1, 0, -1, -1, -1, 0, 0, 1, -1, -1, 0, 0, -1, 1, -1, 1, 0, 0, -1, 1, 1, 1,
         -1, -1, 1, -1, 0, 0, -1, 1, -1, -1, 0, 0, 1, 0, 0, 1, 0, -1, -1, -1, 1, 1, -1, 0, -1, 1,
         -1, 0, 0, 0, 1, -1, 1, 0, 1, 1, 1, -1, -1, 0, 1, -1, 1, 0, 0, 0, 1, -1, 0, -1, 1, 1, 0,
         0, -1, -1, 1, -1, 0, -1, 0, 1, 0, 1, 0, -1, 0, 1, -1, 0, 1, 1, 0, -1, -1, -1, -1, 1, 0,
         -1, 1, 0, -1, 0, 0, 0, -1, 0, -1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, -1, 1, 1, -1, 0, 0,
         -1, 1, -1, -1, -1, -1, -1, -1, 0, 0, -1, -1, 0, -1, 1, 0, 1, 1, 1, -1, 0, 1, 0, 1, 1, -1,
         -1, -1, 0, 1, 1, -1, 1, 0, 0, 0, 1, 1, 0, -1, 0, 1, 1, 1, 0, 0, 0, 1, -1, 1, 0, 0, 0, 0,
         -1, 1, 0, 0, 1, 0, 0, 0, -1, -1, 1, 0, 1, 0, 0, 1, -1, -1, 0, 0, 0, 1, 0, -1, 0, -1, 1,
         -1, -1, 0, 1, 1, 0, -1, 1, -1, -1, 1, 1, 0, -1, 0, 1, -1, 1, -1, 1, 1, 1, 0, 0, -1, 0, 0,
         1, -1, 1, 0, -1, 0, 1, 1, 1, 0, 1, 0, -1, 1, 1, -1, -1, -1, 1, -1, -1, 1, 1, -1, 0, 1,
         -1, 0, -1, 1, 1, 1, 1, 0, 1, -1, 0, 0, 1, -1, 1, -1, 1, -1, -1, 0, -1, 1, 0, 0, 0, 1, 0,
         0, 0, 1, 0, 1, 0, -1, -1, -1, 0, 0, -1, 1, -1, -1, 1, 1, 0, -1, 1, 0, 0, 1, -1, 1, 0, 1,
         0, 1, 0, 1, 1, 0, 1, 0, 0, -1, 0, -1, 1, 1, 1, 1, 1, 1, 1, 0, -1, 0, 0, 0, -1, 1, 1, 1,
         1, 0, 1, 0, 1, 0, 1, 0, -1, 1, 1, -1, 1, 1, -1, 1, -1, -1, 1, -1, -1, -1, -1, 1, -1, -1,
         0, -1, 0, 1, 1, 0, -1, 1, 1, 0, 1, 1, -1, 0, 0, 0, -1, 1, -1, 1, -1, 1, -1, 0, 1, -1, -1,
         -1, 0, 0, 1, -1, -1, -1, 0, 0, 1, 0, 1, 0, -1, 0, 0, 0, -1, 0, 0, 1, 1, 0, -1, 0, 1, 1,
         -1, 0, 0, 1, 0, 1, 1, -1, 1, -1, 1, 0, -1, 0, 0, -1, 1, 0, 1, -1, -1, 0, 1, 0, 1, 0, 0,
         -1, 1, -1, -1, 1, 0, -1, -1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, -1, 1, 1, -1, 0, -1, 1, -1,
         1, 1, 1, 1, 1, -1, -1, 0, 1, 0, 1, -1, 1, 1, -1, -1, -1, 0, -1, 1, 1, 0, 0, 0, 0, 0, 1,
         0, -1, -1, -1, 0, 0, 0, -1, -1, -1, -1, 1, -1, 0, -1, 0, 0, 0, 1, 1, 0, -1, 1, 1, -1, 0,
         1, 0, 0, -1, 0, 0, 0, 1, 1, 0, 1, -1, 1, -1, 1, -1, 0, 1, 1, -1, -1, -1, -1, 1, 1, 0, 0,
         1, 0, -1, 0, 0, 1, 0, 0, 0, 0, 1, 1, -1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, -1, 0, 1];
    #[test]
    fn it_works() {
        let hash = [0i32; STATE_LENGTH].absorb(&TRITS, STATE_LENGTH).squeeze(HASH_LENGTH);
        assert_eq!(hash.len(), HASH_LENGTH);
        println!("{:?}", hash);
    }
}
