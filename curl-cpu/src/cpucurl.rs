use trytes::*;
use curl::*;

#[derive(Copy)]
pub struct CpuCurl<T>
where
    T: Clone + Copy + Sized,
{
    pub state: [T; STATE_LENGTH],
}

impl<T> Clone for CpuCurl<T>
where
    T: Clone + Copy + Sized,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Curl<T> for CpuCurl<T>
where
    T: Clone + Copy + Sized,
    CpuCurl<T>: Sponge + Default,
{
    fn absorb(&mut self, trits: &[T]) {
        for c in trits.chunks(HASH_LENGTH) {
            self.state[0..c.len()].clone_from_slice(c);
            Sponge::transform(self);
        }
    }

    fn squeeze(&mut self, out: &mut [T]) {
        let trit_count = out.len();
        let hash_count = trit_count / HASH_LENGTH;

        for i in 0..hash_count {
            out[i * HASH_LENGTH..(i + 1) * HASH_LENGTH]
                .clone_from_slice(&self.state[0..HASH_LENGTH]);
            Sponge::transform(self);
        }

        let last = trit_count - hash_count * HASH_LENGTH;
        out[trit_count - last..].clone_from_slice(&self.state[0..last]);
        if trit_count % HASH_LENGTH != 0 {
            Sponge::transform(self);
        }
    }

    fn duplex(&mut self, trits: &[T], out: &mut [T]) {
        assert!(
            out.len() % HASH_LENGTH == 0,
            "Output length must be a multiple of HASH_LENGTH"
        );

        for (i, c) in trits.chunks(HASH_LENGTH).enumerate() {
            self.state[0..c.len()].clone_from_slice(c);
            Sponge::transform(self);
            out[i * HASH_LENGTH..(i + 1) * HASH_LENGTH]
                .clone_from_slice(&self.state[0..HASH_LENGTH]);
        }
    }

    fn rate(&self) -> &[T] {
        &self.state[0..HASH_LENGTH]
    }
}
