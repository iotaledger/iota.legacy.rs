#![no_std]
#![feature(alloc)]

#[allow(unused_imports)]
#[macro_use]
extern crate alloc;

// Various constants
pub mod constants;
mod mappings;

// Helper methods
pub mod num;

// Different representations
pub mod byte;
pub mod string;
pub mod bct;

// Helpers for multiple trits encoded as BCTrit
pub mod multiplex;

pub use multiplex::*;
pub use bct::*;
pub use string::*;
pub use byte::*;

pub use constants::TRYTE_ALPHABET;
pub use constants::Trit;
pub use constants::BCTrit;
pub use constants::HASH_LENGTH;
pub use constants::TRITS_PER_TRYTE;
