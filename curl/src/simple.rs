use constants::*;
use trytes::*;
use curl::*;

/// Sponge size
const STATE_LENGTH: usize = HASH_LENGTH * 3;
const TRUTH_TABLE: [Trit; 11] = [1, 0, -1, 2, 1, -1, 0, 2, -1, 1, 0];

/// Basic unoptimised implementation of the `Curl` hashing algorithm
#[derive(Copy)]
pub struct SimpleCurl {
    /// The sponge state
    state: [Trit; STATE_LENGTH],
}

impl Default for SimpleCurl {
    fn default() -> Self {
        SimpleCurl { state: [0; STATE_LENGTH] }
    }
}

impl Clone for SimpleCurl {
    fn clone(&self) -> Self {
        *self
    }
}


impl Curl for SimpleCurl {
    fn transform(&mut self) {
        // Required memory space type for computation
        type Space = i8;

        let mut scratchpad: [Space; STATE_LENGTH] = [0; STATE_LENGTH];
        let mut scratchpad_index: usize = 0;
        let mut scratchpad_index_save: usize;

        for _ in 0..NUMBER_OF_ROUNDS {
            let state_space: Vec<Space> = self.state.iter().map(|&c| c as Space).collect();
            scratchpad.clone_from_slice(state_space.as_slice());
            for state_index in 0..STATE_LENGTH {
                scratchpad_index_save = scratchpad_index;
                if scratchpad_index < 365 {
                    scratchpad_index += 364;
                } else {
                    scratchpad_index -= 365;
                };
                self.state[state_index] = TRUTH_TABLE[(scratchpad[scratchpad_index_save] +
                                                           (scratchpad[scratchpad_index] << 2) +
                                                           5) as
                                                          usize];
            }
        }
    }

    fn reset(&mut self) {
        self.state = [0; STATE_LENGTH];
    }

    fn absorb(&mut self, trits: &[Trit]) {
        let mut len = trits.len();
        let mut offset = 0;
        loop {
            let to = if len < HASH_LENGTH { len } else { HASH_LENGTH };
            self.state[0..to].clone_from_slice(&trits[offset..offset+to]);

            self.transform();

            offset += HASH_LENGTH;
            if len <= HASH_LENGTH {
                break;
            }
            len -= HASH_LENGTH;
        }
    }

    fn squeeze(&mut self, trit_count: usize) -> Vec<Trit> {
        let mut len = trit_count;
        let mut out: Vec<Trit> = Vec::with_capacity(trit_count);
        let mut offset = 0;

        loop {
            let to = if len < HASH_LENGTH { len } else { HASH_LENGTH };
            out.extend_from_slice(&self.state[0..to]);
            self.transform();

            offset += HASH_LENGTH;
            if len <= HASH_LENGTH {
                break;
            }
            len -= HASH_LENGTH;
        }

        out
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn run_testsuite() {
        use tests::testsuite;
        testsuite::run::<SimpleCurl>();
    }

}
