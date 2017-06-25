extern crate iota_trytes as trytes;
extern crate iota_curl as curl;
#[macro_use]
extern crate error_chain;

mod constants;
mod pow;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
pub mod errors {
	// Create the Error, ErrorKind, ResultExt, and Result types
	error_chain!{
        types {
            Error, ErrorKind, ResultExt, Result;
        }
		links {
	    }
        foreign_links {
                Fmt(::std::fmt::Error);
                Io(::std::io::Error) #[cfg(unix)];
        }
        errors {
            InvalidInputSize(i: usize) {
                description("Input trinary is not of `TRANSACTION_LENGTH`")
                display("Invalid trinary Size: '{}'", i)
            }
            InvalidMinWeightMagnitude(i: usize) {
                description("Min weight magnitude exceeds `HASH_LENGT`")
                display("Invalid Weight Size: '{}'", i)                
            }
        }
    }
}

pub use constants::TRANSACTION_LENGTH;
pub use pow::*;
