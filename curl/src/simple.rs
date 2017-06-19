use constants::*;
use trytes::*;
use curl::*;

/// Sponge size
const STATE_LENGTH: usize = HASH_LENGTH * 3;
const TRUTH_TABLE: [Trit; 9] = [1, 0, -1, 1, -1, 0, -1, 1, 0];

/// Basic unoptimised implementation of the `Curl` hashing algorithm
#[derive(Copy)]
pub struct SimpleCurl {
    /// The sponge state
    state: [Trit; STATE_LENGTH],
}

impl Default for SimpleCurl {
    fn default() -> Self {
        SimpleCurl { state: [0; STATE_LENGTH] }
    }
}

impl Clone for SimpleCurl {
    fn clone(&self) -> Self {
        *self
    }
}

impl SimpleCurl {
    fn transform(&mut self) {
        // Required memory space type for computation
        type Space = i32;

        let mut scratchpad: [Space; STATE_LENGTH] = [0; STATE_LENGTH];
        let mut scratchpad_index: usize = 0;
        let mut scratchpad_index_save: usize;

        for _ in 0..NUMBER_OF_ROUNDS {
            let state_space: Vec<Space> = self.state.iter().map(|&c| c as Space).collect();
            scratchpad.clone_from_slice(state_space.as_slice());
            for state_index in 0..STATE_LENGTH {
                scratchpad_index_save = scratchpad_index;
                if scratchpad_index < 365 {
                    scratchpad_index += 364;
                } else {
                    scratchpad_index -= 365;
                };
                self.state[state_index] = TRUTH_TABLE[(scratchpad[scratchpad_index_save] +
                                                       scratchpad[scratchpad_index] * 3 +
                                                       4) as
                                                      usize];
            }
        }
    }
}

impl Curl for SimpleCurl {
    fn absorb(&mut self, trinary: Trinary) {
        let mut len = trinary.len_trits();
        let trits = trinary.trits();
        let mut offset = 0;
        loop {
            let to = offset + if len < HASH_LENGTH { len } else { HASH_LENGTH };
            self.state[0..HASH_LENGTH].clone_from_slice(&trits[offset..to]);

            self.transform();

            offset += HASH_LENGTH;
            len -= HASH_LENGTH;

            if len < HASH_LENGTH {
                break;
            }
        }
    }

    fn squeeze(&mut self, trit_count: usize) -> Trinary {
        let mut len = trit_count;
        let mut out: Vec<Trit> = Vec::with_capacity(trit_count);
        let mut offset = 0;

        loop {
            out.extend_from_slice(&self.state[offset..offset + HASH_LENGTH]);
            self.transform();

            offset += HASH_LENGTH;
            len -= HASH_LENGTH;
            if !(len > HASH_LENGTH) {
                break;
            }
        }

        out.into_iter().collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use curl::Curl;

    #[test]
    fn hash_works() {
        let trans: Trinary = "9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              999999999999999999999999999999T999999999999999999999999999999\
                              99999999999999999999999OLOB99999999999999999999999"
            .chars()
            .collect();

        let ex_hash: Trinary = "TAQCQAEBHLLYKAZWMNSXUPWQICMFSKWPEGQBNM9AQMGLFZGME9REOZTQIJQRKYH\
                             DANIYSMFYPVABX9999"
            .chars()
            .collect();

        let mut curl = SimpleCurl::default();
        curl.absorb(trans);
        let hash: Trinary = curl.squeeze(HASH_LENGTH);

        assert_eq!(hash, ex_hash);
    }
}
