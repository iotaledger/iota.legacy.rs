use alloc::vec::Vec;
use alloc::string::String;
use core::array::FixedSizeArray;

use core::marker;
use core::fmt;
use core::*;
use constants::*;
use util::*;
use bct::*;

/// `Trinary` holds an array of trinary values.
#[derive(Hash, Clone, Eq, PartialEq)]
pub struct Trinary {
    bytes: Vec<u8>,
    length: usize,
}

pub trait Offset {
    fn offset(&mut self);
}

impl<'a> Offset for &'a mut [Trit] {
    fn offset(&mut self) {}
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

pub trait FromTrinary
where
    Self: marker::Sized,
{
    type Err;
    fn from_trinary(t: &Trinary) -> Result<Self, Self::Err>;
}

impl FromTrinary for Trinary {
    type Err = ();
    fn from_trinary(t: &Trinary) -> Result<Self, Self::Err> {
        Ok(t.clone())
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

    /// Pads trinary to specified length in trinaries with specified `Trit`
    ///
    /// This method will return an unmodified Trinary if `length` is less or
    /// equal to the current length.
    pub fn pad(&self, length: usize, with: char) -> Trinary {
        if self.len_trytes() >= length {
            return self.clone();
        }

        let with_trits = tryte_to_trits(with);
        let count = length - self.len_trytes();
        let mut trits: Vec<Trit> = self.trits();

        for _ in 0..count {
            trits.extend_from_slice(with_trits);
        }

        trits.into_iter().collect()
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use alloc::string::ToString;
    #[test]
    fn combine_multiple_trinaries() {
        let t1: Trinary = "ABC".chars().collect();
        let t2: Trinary = "DEF".chars().collect();
        let t3: Trinary = "GH9".chars().collect();

        let ts = [t1, t2, t3];

        let combined: Trinary = ts.as_slice().trinary();

        assert_eq!(combined.to_string(), "ABCDEFGH9")
    }

    #[test]
    fn trinary_from_trinary() {
        let t1: Trinary = "AGBC".chars().collect();

        assert_eq!(t1, Trinary::from_trinary(&t1).ok().unwrap());
    }

    #[test]
    fn pad() {
        let t1: Trinary = "AZ".chars().collect();

        assert_eq!(t1.pad(5, '9').to_string(), "AZ999");
    }
}
