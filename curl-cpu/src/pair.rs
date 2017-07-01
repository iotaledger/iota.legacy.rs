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
        let mut state_clone: [BCTrit; STATE_LENGTH] = [(0, 0); STATE_LENGTH];

        for _ in 0..NUMBER_OF_ROUNDS {
            for i in 0..STATE_LENGTH {
                state_clone[i] = step(
                    self.state[TRANSFORM_INDICES[i]],
                    self.state[TRANSFORM_INDICES[i + 1]],
                );
            }

            self.state.copy_from_slice(&state_clone);
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

    #[test]
    pub fn run_testsuite() {
        curl::tests::run::<BCTrit, CpuCurl<BCTrit>>();
    }

}
