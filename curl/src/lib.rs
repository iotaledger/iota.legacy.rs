#![cfg(test)]
#![feature(test)]

#![feature(alloc)]
#![no_std]
extern crate alloc as collections;


#[cfg(test)]
extern crate test;

#[cfg(feature = "parallel")]
extern crate rayon;
extern crate iota_trytes as trytes;

mod indices;
pub mod constants;
pub mod curl;
pub mod simple;
pub mod pair;
mod tests;

#[cfg(test)]
mod bench;

pub use tests::testsuite;
pub use curl::*;

use trytes::BCTrit;
pub type DefaultCurl = Curl<BCTrit>;
