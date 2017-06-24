use constants::*;
use trytes::*;

/// All implementations of `Curl` must implement this trait.

pub trait Sponge {
    /// Transforms the sponge
    fn transform(&mut self);
    /// Resets the sponge's internal state.
    fn reset(&mut self);
}

#[derive(Copy)]
pub struct Curl<T>
    where T: Clone + Copy + Sized
{
    pub state: [T; STATE_LENGTH],
}

impl<T> Clone for Curl<T>
    where T: Clone + Copy + Sized
{
    fn clone(&self) -> Self {
        *self
    }
}


impl<T> Curl<T>
    where T: Clone + Copy + Sized,
          Curl<T>: Sponge
{
    /// Absorb a `&[Trit]` into the sponge
    pub fn absorb(&mut self, trits: &[T]) {
        let mut len = trits.len();
        let mut offset = 0;
        loop {
            let to = if len < HASH_LENGTH { len } else { HASH_LENGTH };
            self.state[0..to].clone_from_slice(&trits[offset..offset + to]);

            Sponge::transform(self);

            offset += HASH_LENGTH;
            if len <= HASH_LENGTH {
                break;
            }
            len -= HASH_LENGTH;
        }
    }

    /// Squeeze the sponge and return a `Vec<T>` with `trit_count` trits
    pub fn squeeze(&mut self, trit_count: usize) -> Vec<T> {
        let mut len = trit_count;
        let mut out: Vec<T> = Vec::with_capacity(trit_count);
        let mut offset = 0;

        loop {
            let to = if len < HASH_LENGTH { len } else { HASH_LENGTH };
            out.extend_from_slice(&self.state[0..to]);
            self.transform();

            offset += HASH_LENGTH;
            if len <= HASH_LENGTH {
                break;
            }
            len -= HASH_LENGTH;
        }

        out
    }
}
