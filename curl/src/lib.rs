#![no_std]

extern crate iota_trytes as trytes;

use trytes::constants::HASH_LENGTH;
use trytes::Trit;
use trytes::BCTrit;

pub const NUMBER_OF_ROUNDS: usize = 27;
pub const STATE_LENGTH: usize = HASH_LENGTH * 3;
pub const TRANSACTION_LENGTH: usize = 2673;

pub trait Sponge {
    /// Transforms the sponge
    fn transform(&mut self);
    /// Resets the sponge's internal state.
    fn reset(&mut self);
}

pub trait Curl<T>
where
    Self: Default + Sponge + Clone + Send + 'static,
    T: Copy + Clone + Sized,
{
    /// Absorb a `&[Trit]` into the sponge
    fn absorb(&mut self, trits: &[T]);
    /// Squeeze the sponge and writes to the provided output slice.
    fn squeeze(&mut self, out: &mut [T]);
    /// Duplexes the sponge, absorbing a `&[Trit]` and writes to the provided output slice.
    fn duplex(&mut self, trits: &[T], out: &mut [T]);
    /// Exposes the first HASH_LENGTH trits of a state
    fn rate(&self) -> &[T];
    /// Exposes the complete state
    fn state(&self) -> &[T];
    /// Exposes the complete mutable state
    fn state_mut(&mut self) -> &mut [T];
    /// Returns the current number of rounds
    fn rounds(&self) -> u8;
    /// Sets the number of rounds
    fn set_rounds(&mut self, rounds: u8);
}

pub trait ProofOfWork<T: Copy> {
    /// Searches for a nonce, given a `length`. The trits should already be
    /// absorbed into the `tcurl` instance.
    /// Returns the length of the nonce
    fn search<C: Curl<T>, CB: Curl<BCTrit>>(
        weight: u8,
        offset: usize,
        length: usize,
        tcurl: &mut C,
        bcurl: &mut CB,
    ) -> Option<usize>;
}

pub trait HammingNonce<T: Copy> {
    /// Searches for a checksum given by hamming weight
    /// Returns true if it found a valid nonce for the checksum.
    /// It will start with a number of trits given by `length`, but may grow
    /// If security is 1, then the first 81 trits will sum to 0
    /// If security is 2, then the first 81 trits will not sum to 0, but the first 162 trits will.
    /// If security is 3, then neither the first 81 nor the first 162 trits will sum to zero, but
    /// the entire hash will sum to zero
    /// To prepare, you should absorb the length in trits
    fn search<C: Curl<T>, CB: Curl<BCTrit>>(
        security: u8,
        offset: usize,
        length: usize,
        tcurl: &mut C,
        bcurl: &mut CB,
    ) -> Option<usize>;
}
