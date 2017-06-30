use constants::*;
use trytes::*;
use curl::*;
use indices::*;

const TRUTH_TABLE: [Trit; 11] = [1, 0, -1, 2, 1, -1, 0, 2, -1, 1, 0];

impl Sponge for Curl<Trit> {
    #[inline]
    fn transform(&mut self) {
        let mut state_clone: [Trit; STATE_LENGTH] = [0; STATE_LENGTH];

        for _ in 0..NUMBER_OF_ROUNDS {
            for state_index in 0..STATE_LENGTH {
                let idx: usize = (self.state[TRANSFORM_INDICES[state_index]] as usize)
                    .wrapping_add(
                        (self.state[TRANSFORM_INDICES[state_index + 1]] as usize) << 2,
                    )
                    .wrapping_add(5);

                state_clone[state_index] = TRUTH_TABLE[idx];
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
