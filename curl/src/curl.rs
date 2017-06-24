use constants::*;
//use trytes::Trit;
use trytes::*;
use trytes::Trinary;

/// All implementations of `Curl` must implement this trait.

pub trait Sponge {
    fn transform(&mut self);
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
    fn absorb(&mut self, trits: &[T], offset_in: usize, length_in: usize) {
        let mut len = length_in;
        let mut offset = offset_in;
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

    fn squeeze(&mut self, offset_in: usize, trit_count: usize) -> Vec<T> {
        let mut len = trit_count;
        let mut out: Vec<T> = Vec::with_capacity(trit_count);
        let mut offset = offset_in;

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
