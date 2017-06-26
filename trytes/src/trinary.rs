use collections::vec::Vec;
use collections::string::String;

use core::fmt;
use constants::*;
use util::*;
use bct::*;

/// `Trinary` holds an array of trinary values.
#[derive(Hash, Clone, Eq, PartialEq)]
pub struct Trinary {
    bytes: Vec<u8>,
    length: usize,
}

impl fmt::Display for Trinary {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.chars().into_iter().collect();
        fmt.write_str(s.as_str())
    }
}

impl fmt::Debug for Trinary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Trinary({}, {}, {:?})", self, self.length, self.bytes)
    }
}

/// Default trait for serialisation into a `Trinary`
pub trait IntoTrinary {
    fn trinary(&self) -> Trinary;
}

impl IntoTrinary for Trinary {
    fn trinary(&self) -> Trinary {
        self.clone()
    }
}

pub trait IntoTrits<T> {
    fn trits(&self) -> Vec<T>;
}

impl IntoTrits<BCTrit> for Trinary {
    /// Returns a binary-coded representation of the trits of this `Trinary`.
    /// See http://homepage.divms.uiowa.edu/~jones/ternary/bct.shtml for further details.
    fn trits(&self) -> Vec<BCTrit> {
        self.trits()
            .into_iter()
            .map(trit_to_bct)
            .collect()
    }
}

impl IntoTrits<Trit> for Trinary {
    /// Returns a `Vec<Trit>` representation of this `Trinary`
    fn trits(&self) -> Vec<Trit> {
        let mut trits: Vec<Trit> = Vec::with_capacity(self.len_trits());
        let mut cnt = self.length;

        for b in &self.bytes {
            let t = byte_to_trits(*b);

            if cnt > TRITS_PER_BYTE {
                trits.extend_from_slice(t);
            } else {
                trits.extend_from_slice(&t[0..cnt]);
                break;
            }
            cnt -= TRITS_PER_BYTE;
        }

        trits
    }
}

impl Trinary {
    /// Default `Trinary` constructor
    pub fn new(bytes: Vec<u8>, length: usize) -> Trinary {
        Trinary {
            bytes: bytes,
            length: length,
        }
    }

    /// Returns a `Vec<char>` of the trytes of this `Trinary`
    pub fn chars(&self) -> Vec<char> {
        self.trits().chunks(3).map(trits_to_char).collect()
    }

    /// Returns the `&[u8]` representation of this `Trinary`
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Length of this `Trinary` in trits
    pub fn len_trits(&self) -> usize {
        self.length
    }

    /// Length of this `Trinary` in trytes
    pub fn len_chars(&self) -> usize {
        self.length / TRITS_PER_TRYTE
    }

    /// Length of this `Trinary` in bytes
    pub fn len_bytes(&self) -> usize {
        self.bytes.len()
    }
}
