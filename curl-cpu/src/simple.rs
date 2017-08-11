use trytes::*;
use cpucurl::*;
use curl::*;
use indices::*;

const TRUTH_TABLE: [Trit; 11] = [1, 0, -1, 2, 1, -1, 0, 2, -1, 1, 0];

impl Sponge for CpuCurl<Trit> {
    #[inline]
    fn transform(&mut self) {
        let mut local_state: [Trit; STATE_LENGTH] = [0; STATE_LENGTH];

        for round in 0..self.rounds {
            let (mut state_out, &state) = if round % 2 == 0 {
                (&mut local_state, &self.state)
            } else {
                (&mut self.state, &local_state)
            };

            for state_index in 0..STATE_LENGTH {
                let idx: usize = (state[TRANSFORM_INDICES[state_index]] as usize)
                    .wrapping_add((state[TRANSFORM_INDICES[state_index + 1]] as usize) << 2)
                    .wrapping_add(5);

                state_out[state_index] = TRUTH_TABLE[idx];
            }
        }

        if self.rounds % 2 == 1 {
            self.state = local_state;
        }
    }

    fn reset(&mut self) {
        self.state = [0; STATE_LENGTH];
    }
}

impl Default for CpuCurl<Trit> {
    fn default() -> Self {
        CpuCurl::<Trit> {
            state: [0 as Trit; STATE_LENGTH],
            rounds: NUMBER_OF_ROUNDS as u8,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use alloc::Vec;
    use curl_tests;

    struct TritTransformer;

    impl curl_tests::TransformerFn<Trit> for TritTransformer {
        fn transform(&self, t: &[Trit]) -> Vec<Trit> {
            t.to_vec()
        }
    }

    #[test]
    pub fn run_testsuite() {
        curl_tests::run::<Trit, CpuCurl<Trit>>(&TritTransformer);
    }

}
