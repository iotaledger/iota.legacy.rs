use constants::*;
use trytes::*;
use curl::*;

const TRUTH_TABLE: [Trit; 11] = [1, 0, -1, 2, 1, -1, 0, 2, -1, 1, 0];

impl Sponge for Curl<Trit> {
    fn transform(&mut self) {
        // Required memory space type for computation
        let mut scratchpad: [Trit; STATE_LENGTH] = [0; STATE_LENGTH];
        let mut scratchpad_index: usize = 0;
        let mut scratchpad_index_save: usize;

        for _ in 0..NUMBER_OF_ROUNDS {
            //let state_space: Vec<Space> = self.state.iter().map(|&c| c as Space).collect();
            let state_space: Vec<Trit> = self.state.iter().cloned().collect();
            //scratchpad.clone_from_slice(state_space.as_slice());
            scratchpad.clone_from_slice(state_space.as_slice());
            for state_index in 0..STATE_LENGTH {
                scratchpad_index_save = scratchpad_index;
                if scratchpad_index < 365 {
                    scratchpad_index += 364;
                } else {
                    scratchpad_index -= 365;
                };
                self.state[state_index] = TRUTH_TABLE[(scratchpad[scratchpad_index_save] + (scratchpad[scratchpad_index] << 2) +
                 5) as usize];
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
        testsuite::run::<Curl<Trit>>();
    }

}
