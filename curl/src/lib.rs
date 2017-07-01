#![feature(alloc)]
#![no_std]
extern crate alloc;
extern crate iota_trytes as trytes;

use trytes::constants::HASH_LENGTH;

pub mod tests;

pub const NUMBER_OF_ROUNDS: usize = 27;
pub const STATE_LENGTH: usize = HASH_LENGTH * 3;

use alloc::Vec;

pub trait Sponge {
    /// Transforms the sponge
    fn transform(&mut self);
    /// Resets the sponge's internal state.
    fn reset(&mut self);
}

pub trait Curl<T>
    where
    Self: Default + Sponge,
    T: Copy + Clone + Sized,

{
    /// Absorb a `&[Trit]` into the sponge
    fn absorb(&mut self, trits: &[T]);
    /// Squeeze the sponge and return a `Vec<T>` with `trit_count` trits
    fn squeeze(&mut self, count: usize) -> Vec<T>;
}
