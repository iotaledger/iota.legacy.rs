#[cfg(feature = "parallel")]
use rayon::prelude::*;

use collections::Vec;
use constants::*;
use trytes::*;
use curl::*;
use indices::*;

const TRUTH_TABLE: [Trit; 11] = [1, 0, -1, 2, 1, -1, 0, 2, -1, 1, 0];

impl Sponge for Curl<Trit> {
    #[cfg(feature = "parallel")]
    fn transform(&mut self) {
        let mut scratchpad: Vec<Trit> = self.state.iter().cloned().collect();

        for _ in 0..NUMBER_OF_ROUNDS {
            scratchpad = (0..STATE_LENGTH)
                .into_par_iter()
                .map(|i| {
                    TRUTH_TABLE[(scratchpad[TRANSFORM_INDICES[i]] +
                                     (scratchpad[TRANSFORM_INDICES[i + 1]] << 2) +
                                     5) as usize]
                })
                .collect();
        }

        self.state.copy_from_slice(&scratchpad);
    }

    #[cfg(not(feature = "parallel"))]
    fn transform(&mut self) {
        let mut scratchpad_index: usize = 0;
        let mut state_clone: [Trit; STATE_LENGTH] = [0; STATE_LENGTH];

        for _ in 0..NUMBER_OF_ROUNDS {
            for state_index in 0..STATE_LENGTH {
                state_clone[state_index] =
                    TRUTH_TABLE[(self.state[TRANSFORM_INDICES[state_index]] +
                                     (self.state[TRANSFORM_INDICES[state_index + 1]] << 2) +
                                     5) as usize];
                /*let scratchpad_index_save = scratchpad_index;
                if scratchpad_index < 365 {
                    scratchpad_index += 364;
                } else {
                    scratchpad_index -= 365;
                };
                state_clone[state_index] = TRUTH_TABLE[(self.state[scratchpad_index_save] +
                                                            (self.state[scratchpad_index] << 2) +
                                                            5) as
                                                           usize];*/
            }

            self.state.copy_from_slice(&state_clone);
        }
    }

    fn reset(&mut self) {
        self.state = [0; STATE_LENGTH];
    }
}

impl Default for Curl<Trit> {
    fn default() -> Self {
        let x: Trit = 0 as Trit;
        Curl::<Trit> { state: [x; STATE_LENGTH] }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn run_testsuite() {
        use tests::testsuite;
        testsuite::run::<Trit>();
    }

}
