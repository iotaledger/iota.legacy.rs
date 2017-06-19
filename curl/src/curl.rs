use constants::*;
use trytes::*;

#[derive(Copy)]
pub struct Curl {
    state: [Trit; STATE_LENGTH],
}

impl Default for Curl {
    fn default() -> Self {
        Curl { state: [0; STATE_LENGTH] }
    }
}

impl Clone for Curl {
    fn clone(&self) -> Curl {
        *self
    }
}

impl Curl {
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

    fn absorb(&mut self, trits: &Vec<Trit>, length: usize) {
        let mut len = length;
        let mut offset = 0;
        while {
                  let to = offset + if len < HASH_LENGTH { len } else { HASH_LENGTH };
                  self.state[0..HASH_LENGTH].clone_from_slice(&trits[offset..to]);

                  self.transform();

                  offset += HASH_LENGTH;
                  len -= HASH_LENGTH;
                  len >= HASH_LENGTH
              } {}
    }

    fn squeeze(&mut self, length: usize) -> Vec<Trit> {
        let mut len = length;
        let mut out: Vec<Trit> = Vec::with_capacity(length);
        let mut offset = 0;
        while {
                  out.extend_from_slice(&self.state[0..HASH_LENGTH]);
                  self.transform();

                  offset += HASH_LENGTH;
                  len -= HASH_LENGTH;
                  len > HASH_LENGTH
              } {}
        out
    }
}


#[cfg(test)]
mod tests {
    use super::*;
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

        let mut curl = Curl::default();
        curl.absorb(&trans.trits(), trans.len_trits());
        let hash: Trinary = curl.squeeze(HASH_LENGTH).into_iter().collect();

        assert_eq!(hash, ex_hash);
    }
}
