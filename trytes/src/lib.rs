#![no_std]
#![feature(alloc)]
#![feature(fixed_size_array)]

#[macro_use]
extern crate alloc;

// constant definitions
pub mod constants;
pub mod mappings;
pub mod util;

// trinary type
pub mod trinary;
pub mod num;

// trinary traits
pub mod string;
pub mod trits;
pub mod iter;
pub mod bct;
pub mod multiplex;
pub mod pascal;

pub use constants::TRYTE_ALPHABET;
pub use constants::Trit;
pub use constants::BCTrit;
pub use constants::HASH_LENGTH;
pub use constants::TRITS_PER_TRYTE;

pub use bct::*;
pub use trinary::*;
pub use string::*;
pub use trits::*;
pub use iter::*;
pub use multiplex::*;
