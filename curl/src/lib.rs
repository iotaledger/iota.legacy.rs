#![feature(alloc)]
#![no_std]
extern crate alloc;
extern crate iota_trytes as trytes;

use trytes::constants::HASH_LENGTH;
use trytes::Trinary;
use trytes::Trit;

pub mod tests;

pub const NUMBER_OF_ROUNDS: usize = 27;
pub const STATE_LENGTH: usize = HASH_LENGTH * 3;
pub const TRANSACTION_LENGTH: usize = 2673;

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

pub trait ProofOfWork {
    /// Searches for a nonce given an `input` that gives a hash with `weight` zeros
    /// Returns the nonce
    fn search(input: &[Trit], weight: u8) -> Option<Trinary>;
}

pub trait HammingNonce {
    /// Searches for a checksum given by hamming weight
    /// Returns the nonce to create checksum
    /// It will start with a number of trits given by `length`, but may grow
    /// If security is 1, then the first 81 trits will sum to 0
    /// If security is 2, then the first 81 trits will not sum to 0, but the first 162 trits will.
    /// If security is 3, then neither the first 81 nor the first 162 trits will sum to zero, but
    /// the entire hash will sum to zero
    fn search(input: &[Trit], length: u8, security: u8) -> Option<Trinary>;
}
