use trytes::*;
use curl::*;
use alloc::Vec;

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

    fn squeeze(&mut self, trit_count: usize) -> Vec<T> {
        let mut out: Vec<T> = Vec::new();

        let hash_count = trit_count / HASH_LENGTH;

        for _ in 0..hash_count {
            out.extend_from_slice(&self.state[0..HASH_LENGTH]);
            Sponge::transform(self);
        }

        out.extend_from_slice(&self.state[0..(trit_count - hash_count * HASH_LENGTH)]);
        if trit_count % HASH_LENGTH != 0 {
            Sponge::transform(self);
        }

        out
    }

    fn duplex(&mut self, trits: &[T]) -> Vec<T> {
        let mut out: Vec<T> = Vec::with_capacity(trits.len());

        for c in trits.chunks(HASH_LENGTH) {
            self.state[0..c.len()].clone_from_slice(c);
            Sponge::transform(self);
            out.extend_from_slice(&self.state[0..HASH_LENGTH]);
        }

        out
    }

    fn rate(&self) -> &[T] {
        &self.state[0..HASH_LENGTH]
    }
}
