use alloc::vec::Vec;
use alloc::string::String;
use core::array::FixedSizeArray;

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

/// Construct a single trinary from a &[Trinary]
impl<'a> IntoTrinary for &'a [Trinary] {
    fn trinary(&self) -> Trinary {
        self.iter().flat_map(|t| t).collect()
    }
}

/// Construct a single trinary from a &[&Trinary]
impl<'a> IntoTrinary for &'a [&'a Trinary] {
    fn trinary(&self) -> Trinary {
        self.iter().flat_map(|&t| t).collect()
    }
}

/// Construct a single trinary from a [&Trinary; _]
impl<'a, Array> IntoTrinary for &'a Array
    where
    Array: FixedSizeArray<&'a Trinary>,
{
    fn trinary(&self) -> Trinary {
        self.as_slice().iter().flat_map(|&t| t).collect()
    }
}

pub trait IntoTrits<T> {
    fn trits(&self) -> Vec<T>;
}

impl IntoTrits<BCTrit> for Trinary {
    /// Returns a binary-coded representation of the trits of this `Trinary`.
    /// See http://homepage.divms.uiowa.edu/~jones/ternary/bct.shtml for further details.
    fn trits(&self) -> Vec<BCTrit> {
        self.trits().into_iter().map(trit_to_bct).collect()
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
    pub fn len_trytes(&self) -> usize {
        self.length / TRITS_PER_TRYTE
    }

    /// Length of this `Trinary` in bytes
    pub fn len_bytes(&self) -> usize {
        self.bytes.len()
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use alloc::*;
    use alloc::string::ToString;
    #[test]
    fn combine_multiple_trinaries() {
        let t1: Trinary = "ABC".chars().collect();
        let t2: Trinary = "DEF".chars().collect();
        let t3: Trinary = "GH9".chars().collect();

        let ts = vec![t1, t2, t3];

        let combined: Trinary = ts.as_slice().trinary();

        assert_eq!(combined.to_string(), "ABCDEFGH9")

    }
}
