use trytes::*;
use cpucurl::*;
use curl::*;
use indices::*;

const TRUTH_TABLE: [Trit; 11] = [1, 0, -1, 2, 1, -1, 0, 2, -1, 1, 0];

impl Sponge for CpuCurl<Trit> {
    #[inline]
    fn transform(&mut self) {
        let mut local_state: [Trit; STATE_LENGTH] = [0; STATE_LENGTH];

        for _ in 0..NUMBER_OF_ROUNDS {
            for state_index in 0..STATE_LENGTH {
                let idx: usize = (self.state[TRANSFORM_INDICES[state_index]] as usize)
                    .wrapping_add(
                        (self.state[TRANSFORM_INDICES[state_index + 1]] as usize) << 2,
                    )
                    .wrapping_add(5);

                local_state[state_index] = TRUTH_TABLE[idx];
            }

            self.state = local_state;
        }
    }

    fn reset(&mut self) {
        self.state = [0; STATE_LENGTH];
    }
}

impl Default for CpuCurl<Trit> {
    fn default() -> Self {
        let x: Trit = 0 as Trit;
        CpuCurl::<Trit> { state: [x; STATE_LENGTH] }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::curl;

    #[test]
    pub fn run_testsuite() {
        curl::tests::run::<Trit, CpuCurl<Trit>>();
    }

}
