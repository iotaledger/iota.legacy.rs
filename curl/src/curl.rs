use alloc::Vec;

/// All implementations of `Curl` must implement this trait.

pub trait Sponge {
    /// Transforms the sponge
    fn transform(&mut self);
    /// Resets the sponge's internal state.
    fn reset(&mut self);
}

pub trait Curl<T> {
    fn absorb(&mut self, trits: &[T]);
    fn squeeze(&mut self, count: usize) -> Vec<T>;
}
