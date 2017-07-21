#![no_std]

extern crate iota_trytes as trytes;

use trytes::constants::HASH_LENGTH;
use trytes::Trit;

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
    Self: Default + Sponge,
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
}

pub trait ProofOfWork<T> {
    /// Searches for a nonce given an `input` that gives a hash with `weight` zeros
    /// Returns true if it found a nonce.
    fn search(input: &[T], weight: u8, out: &mut [Trit]) -> bool;
}

pub trait HammingNonce<T> {
    /// Searches for a checksum given by hamming weight
    /// Returns true if it found a valid nonce for the checksum.
    /// It will start with a number of trits given by `length`, but may grow
    /// If security is 1, then the first 81 trits will sum to 0
    /// If security is 2, then the first 81 trits will not sum to 0, but the first 162 trits will.
    /// If security is 3, then neither the first 81 nor the first 162 trits will sum to zero, but
    /// the entire hash will sum to zero
    fn search(input: &[T], security: u8, out: &mut [Trit]) -> bool;
}
