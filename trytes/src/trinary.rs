use std::fmt;
use constants::*;
use util::*;

#[derive(Clone, Eq, PartialEq)]
pub struct Trinary {
    bytes: Vec<i8>,
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

impl Trinary {
    pub fn new(bytes: Vec<i8>, length: usize) -> Trinary {
        Trinary {
            bytes: bytes,
            length: length
        }
    }

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
    pub fn chars(&self) -> Vec<char> {
        self.trits().chunks(3).map(trits_to_char).collect()
    }

    pub fn bytes(self) -> Vec<i8> {
        self.bytes
    }

    pub fn len_trits(self) -> usize {
        self.length
    }

    pub fn len_chars(self) -> usize {
        self.length / TRITS_PER_TRYTE
    }
}



