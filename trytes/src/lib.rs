#[macro_use]
extern crate error_chain;

pub mod errors {
    error_chain! {
        types {
            Error, ErrorKind, ResultExt, Result;
        }
        links {}
        foreign_links{}
        errors {
            InvalidInputChar(pos: usize, what: char) {
                description("Input character was not part of the `TRYTE_ALPHABET`")
                display("Invalid input character {} at {}", what, pos)
            }

            EmptyInputString {
                description("Input string was empty")
            }

            MaxMultiplexReached {
                description("Maximum supported number of trinaries already being multiplexed.")
            }

            TrinaryLengthNotEqual(expected: usize, actual: usize) {
                description("The `Trinary`'s trit length is unequal to the expected.")
                display("Expected trit length of {} but got {}", expected, actual)
            }
        }
    }
}

// constant definitions
pub mod constants;
pub mod mappings;
pub mod util;

// trinary type
pub mod trinary;

// trinary traits
pub mod string;
pub mod trits;
pub mod iter;
pub mod bct;
pub mod multiplex;

pub use constants::TRYTE_ALPHABET;
pub use constants::Trit;
pub use constants::BCTrit;
pub use constants::HASH_LENGTH;

pub use trinary::*;
pub use string::*;
pub use trits::*;
pub use iter::*;
pub use multiplex::*;
