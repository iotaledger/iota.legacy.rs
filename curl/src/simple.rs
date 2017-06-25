use constants::*;
use trytes::*;
use curl::*;

const TRUTH_TABLE: [Trit; 11] = [1, 0, -1, 2, 1, -1, 0, 2, -1, 1, 0];

impl Sponge for Curl<Trit> {
    fn transform(&mut self) {
        let mut scratchpad_index: usize = 0;

        for _ in 0..NUMBER_OF_ROUNDS {
            let scratchpad: Vec<Trit> = self.state.iter().cloned().collect();
            for state_index in 0..STATE_LENGTH {
                let scratchpad_index_save = scratchpad_index;
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
