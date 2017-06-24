use rayon::prelude::*;
use constants::*;
use trytes::*;
use curl::*;
use indices::*;

fn step(first: BCTrit, second: BCTrit) -> BCTrit {
    let (alpha, beta) = first;
    let (delta_0, gamma) = second;
    let delta = (alpha | (!gamma)) & (delta_0 ^ beta);
    (!delta, (alpha ^ gamma) | delta)
}

/// Tuple implementation of the `Sponge` for Curl
impl Sponge for Curl<BCTrit> {
    fn transform(&mut self) {
        let mut scratchpad: Vec<BCTrit> = self.state.iter().map(|&c| (c.0, c.1)).collect();

        for _ in 0..NUMBER_OF_ROUNDS {
            scratchpad = (0..STATE_LENGTH)
                .into_par_iter()
                .map(|i| {
                         step(scratchpad[TRANSFORM_INDICES[i]],
                              scratchpad[TRANSFORM_INDICES[i + 1]])
                     })
                .collect();
        }
        self.state.clone_from_slice(&scratchpad);
    }

    fn reset(&mut self) {
        self.state = [(0, 0); STATE_LENGTH];
    }
}

impl Default for Curl<BCTrit> {
    fn default() -> Self {
        let x: BCTrit = (0, 0);
        Curl::<BCTrit> { state: [x; STATE_LENGTH] }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn run_testsuite() {
        use tests::testsuite;
        testsuite::run::<Curl<BCTrit>>();
    }

}
