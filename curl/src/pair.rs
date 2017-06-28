#[cfg(feature = "parallel")]
use rayon::prelude::*;

use constants::*;
use trytes::*;
use curl::*;
use indices::*;

#[inline(always)]
fn step(first: BCTrit, second: BCTrit) -> BCTrit {
    let (alpha, beta) = first;
    let (delta_0, gamma) = second;
    let delta = (alpha | (!gamma)) & (delta_0 ^ beta);
    (!delta, ((alpha ^ gamma) | delta))
}

/// Tuple implementation of the `Sponge` for Curl
impl Sponge for Curl<BCTrit> {
    #[cfg(feature = "parallel")]
    fn transform(&mut self) {
        use collections::Vec;

        let mut scratchpad: Vec<BCTrit> = self.state.iter().map(|&c| (c.0, c.1)).collect();

        for _ in 0..NUMBER_OF_ROUNDS {
            scratchpad = (0..STATE_LENGTH)
                .into_par_iter()
                .map(|i| {
                    step(
                        scratchpad[TRANSFORM_INDICES[i]],
                        scratchpad[TRANSFORM_INDICES[i + 1]],
                    )
                })
                .collect();
        }
        self.state.clone_from_slice(&scratchpad);
    }

    #[cfg(not(feature = "parallel"))]
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

impl Default for Curl<BCTrit> {
    fn default() -> Self {
        Curl::<BCTrit> { state: [(usize::max_value(), usize::max_value()); STATE_LENGTH] }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn run_testsuite() {
        use tests::testsuite;
        testsuite::run::<BCTrit>();
    }

}
