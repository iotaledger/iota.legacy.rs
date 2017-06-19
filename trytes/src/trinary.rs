use std::fmt;
use constants::*;
use util::*;

/// `Trinary` holds an array of trinary values.
#[derive(Clone, Eq, PartialEq)]
pub struct Trinary {
    bytes: Vec<u8>,
    length: usize
    
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
trait IntoTrinary {
    fn trinary(self) -> Trinary;
}

impl IntoTrinary for Trinary {
    fn trinary(self) -> Trinary {
        self
    }
}

impl Trinary {
    /// Default `Trinary` constructor
    pub fn new(bytes: Vec<u8>, length: usize) -> Trinary {
        Trinary {
            bytes: bytes,
            length: length
        }
    }

    /// Returns a `Vec<Trit>` representation of this `Trinary` 
    pub fn trits(&self) -> Vec<Trit> {
        let mut trits: Vec<Trit> = Vec::new();
        let mut cnt = self.length;

        for b in &self.bytes {
            let mut t = byte_to_trits(*b);

            if cnt > TRITS_PER_BYTE {
                t.reverse();
                trits.append(&mut t);
            } else {
                trits.extend(t[0..cnt].iter().rev().cloned());
                break;
            }
            cnt -= TRITS_PER_BYTE;
        }

        trits
    }

    /// Returns a `Vec<char>` of the trytes of this `Trinary` 
    pub fn chars(&self) -> Vec<char> {
        self.trits().chunks(3).map(trits_to_char).collect()
    }

    /// Returns the `Vec<u8>` representation of this `Trinary` 
    pub fn bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Length of this `Trinary` in trits 
    pub fn len_trits(self) -> usize {
        self.length
    }

    /// Length of this `Trinary` in trytes
    pub fn len_chars(self) -> usize {
        self.length / TRITS_PER_TRYTE
    }
}



