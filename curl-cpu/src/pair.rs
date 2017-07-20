use trytes::*;
use curl::*;
use cpucurl::*;
use indices::*;

#[inline(always)]
fn step(first: BCTrit, second: BCTrit) -> BCTrit {
    let (alpha, beta) = first;
    let (delta_0, gamma) = second;
    let delta = (alpha | (!gamma)) & (delta_0 ^ beta);
    (!delta, ((alpha ^ gamma) | delta))
}

/// Tuple implementation of the `Sponge` for CpuCurl
impl Sponge for CpuCurl<BCTrit> {
    fn transform(&mut self) {
        let mut local_state: [BCTrit; STATE_LENGTH] = [(0, 0); STATE_LENGTH];

        for round in 0..NUMBER_OF_ROUNDS {
            let (mut state_out, &state) = if round % 2 == 0 {
                (&mut local_state, &self.state)
            } else {
                (&mut self.state, &local_state)
            };

            for i in 0..STATE_LENGTH {
                state_out[i] = step(
                    state[TRANSFORM_INDICES[i]],
                    state[TRANSFORM_INDICES[i + 1]],
                );
            }

        }

        if NUMBER_OF_ROUNDS % 2 == 1 {
            self.state = local_state;
        }
    }

    fn reset(&mut self) {
        self.state = [(usize::max_value(), usize::max_value()); STATE_LENGTH];
    }
}

impl Default for CpuCurl<BCTrit> {
    fn default() -> Self {
        CpuCurl::<BCTrit> { state: [(usize::max_value(), usize::max_value()); STATE_LENGTH] }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::curl;
    use alloc::Vec;

    struct BCTritTransformer;

    impl curl::tests::TransformerFn<BCTrit> for BCTritTransformer {
        fn transform(&self, t: &[Trit]) -> Vec<BCTrit> {
            t.iter().cloned().map(trit_to_bct).collect()
        }
    }

    #[test]
    pub fn run_testsuite() {
        curl::tests::run::<BCTrit, CpuCurl<BCTrit>> (&BCTritTransformer);
    }
}
