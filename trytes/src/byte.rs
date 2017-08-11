use constants::{RADIX, Trit, TRITS_PER_BYTE};
use mappings::*;

/// Converts a slice of trits to a byte
/// `trits.len()` must be less or equal to `TRITS_PER_BYTE`
#[inline]
pub fn trits_to_byte(trits: &[Trit]) -> u8 {
    assert!(trits.len() <= TRITS_PER_BYTE);

    let mut value: Trit = 0;
    for j in (0..trits.len()).rev() {
        value = value * RADIX + trits[j];
    }

    value as u8
}

/// Converts a byte to `&[Trit]`
#[inline]
pub fn byte_to_trits(bu: u8) -> &'static [Trit; TRITS_PER_BYTE] {
    let b = bu as i8;
    let bpos: usize = (if b < 0 {
                           (b as isize) + (BYTE_TO_TRITS_MAPPINGS.len() as isize)
                       } else {
                           b as isize
                       }) as usize;
    &BYTE_TO_TRITS_MAPPINGS[bpos]
}

#[derive(Eq, PartialEq, Debug)]
pub struct ByteTritsSlice<'a> {
    data: &'a [u8],
    length: usize,
}

#[derive(Eq, PartialEq, Debug)]
pub struct ByteTritsSliceIter<'a> {
    slice: &'a ByteTritsSlice<'a>,
    pos: usize,
}

#[inline(always)]
fn trit_at_idx(bytes: &[u8], idx: usize) -> Trit {
    let q = idx / TRITS_PER_BYTE;
    let r = idx % TRITS_PER_BYTE;
    byte_to_trits(bytes[q])[r]
}

impl<'a> Iterator for ByteTritsSliceIter<'a> {
    type Item = Trit;
    fn next(&mut self) -> Option<Trit> {
        if self.pos >= self.len() {
            None
        } else {
            let ret = trit_at_idx(self.slice.data, self.pos);
            self.pos += 1;
            Some(ret)
        }
    }
}

impl<'a> ExactSizeIterator for ByteTritsSliceIter<'a> {
    fn len(&self) -> usize {
        self.slice.length
    }
}

impl<'s> ByteTritsSlice<'s> {
    pub fn from_raw<'a>(s: &'a [u8], length: usize) -> ByteTritsSlice<'a> {
        ByteTritsSlice::<'a> {
            data: s,
            length: length,
        }
    }

    /// Returns the underlying bytes.
    pub fn bytes(&self) -> &[u8] {
        self.data
    }

    /// The number of trits this slice represents.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns the trit value at the given index
    pub fn at(&self, idx: usize) -> Trit {
        assert!(self.len() > idx, "Index greater than available trits.");
        trit_at_idx(self.data, idx)
    }


    /// Efficient way to write the trits this `ByteTritsSlice` represents
    /// into a slice. 
    pub fn into_slice(&self, out: &mut [Trit]) {
        assert!(
            out.len() >= self.len(),
            "Output slice is shorter than trit count"
        );

        let rest = self.len() % TRITS_PER_BYTE;
        if rest == 0 {
            for i in 0..self.data.len() {
                out[i * TRITS_PER_BYTE..(i + 1) * TRITS_PER_BYTE]
                    .clone_from_slice(byte_to_trits(self.data[i]));
            }
        } else {
            let last = self.data.len() - 1;
            for i in 0..last {
                out[i * TRITS_PER_BYTE..(i + 1) * TRITS_PER_BYTE]
                    .clone_from_slice(byte_to_trits(self.data[i]));
            }

            out[last * TRITS_PER_BYTE..(last * TRITS_PER_BYTE + rest)]
                .clone_from_slice(&byte_to_trits(self.data[last])[..rest]);
        }
    }
}

/// Generic trait for creating a trit iterator out of a struct
pub trait ToTrits<'a> {
    type Iter: ExactSizeIterator<Item = Trit>;
    /// Returns a trits iterator for this instance.
    fn trits(&'a self) -> Self::Iter;
}

impl<'a> ToTrits<'a> for &'a ByteTritsSlice<'a> {
    type Iter = ByteTritsSliceIter<'a>;
    fn trits(&'a self) -> Self::Iter {
        ByteTritsSliceIter::<'a> {
            slice: self,
            pos: 0,
        }
    }
}

impl<'a> ToTrits<'a> for ByteTritsSlice<'a> {
    type Iter = ByteTritsSliceIter<'a>;
    fn trits(&'a self) -> Self::Iter {
        ByteTritsSliceIter::<'a> {
            slice: self,
            pos: 0,
        }
    }
}

#[cfg(any(test, feature = "alloc"))]
mod with_alloc {
    use super::*;
    use alloc::Vec;
    use core::iter::FromIterator;

    #[derive(Eq, PartialEq, Debug)]
    pub struct ByteTrits {
        data: Vec<u8>,
        length: usize,
    }

    impl ByteTrits {
        /// Transforms a `Vec<Trit>` to `ByteTrits` in place without additional
        /// allocations.
        pub fn inplace(mut ts: Vec<Trit>) -> ByteTrits {
            use core::mem;

            let trit_count = ts.len();
            let bytecount = trit_count / TRITS_PER_BYTE +
                (trit_count % TRITS_PER_BYTE > 0) as usize;
            let mut bytevec = unsafe {
                Vec::from_raw_parts(ts.as_mut_ptr() as *mut u8, bytecount, ts.capacity())
            };

            for (i, chunk) in ts.chunks(TRITS_PER_BYTE).enumerate() {
                if chunk.len() == TRITS_PER_BYTE {
                    bytevec[i] = trits_to_byte(chunk);
                } else {
                    let mut trits = [0 as Trit; TRITS_PER_BYTE];
                    trits[..chunk.len()].clone_from_slice(chunk);
                    bytevec[i] = trits_to_byte(&trits);
                }
            }

            mem::forget(ts);
            bytevec.shrink_to_fit();
            ByteTrits {
                data: bytevec,
                length: trit_count,
            }
        }

        pub fn as_slice<'a>(&'a self) -> ByteTritsSlice<'a> {
            ByteTritsSlice {
                data: &self.data,
                length: self.length,
            }
        }

        /// Returns a view of the underlying bytes
        pub fn data(&self) -> &[u8] {
            &self.data
        }

        /// Returns the number of trits represented by this `ByteTrits` instance
        pub fn len(&self) -> usize {
            self.length
        }
    }

    /// Allows to `.collect()` into a ByteTrits
    impl<'a> FromIterator<&'a Trit> for ByteTrits {
        fn from_iter<I: IntoIterator<Item = &'a Trit>>(iter: I) -> Self {
            let mut space = [0 as Trit; TRITS_PER_BYTE];
            let mut cur = 0;
            let mut total = 0;
            let mut data: Vec<u8> = Vec::new();

            for &i in iter {
                space[cur] = i;
                total += 1;
                cur += 1;

                if cur == TRITS_PER_BYTE {
                    data.push(trits_to_byte(&space));
                    cur = 0;
                }
            }

            if cur != 0 {
                for i in (cur + 1)..TRITS_PER_BYTE {
                    space[i] = 0;
                }
                data.push(trits_to_byte(&space));
            }


            data.shrink_to_fit();
            ByteTrits {
                length: total,
                data: data,
            }
        }
    }

}

#[cfg(any(test, feature = "alloc"))]
pub use self::with_alloc::*;


#[cfg(test)]
mod test {
    use super::*;
    use alloc::Vec;

    static IN: [Trit; 27] = [
        0,
        1,
        -1,
        1,
        -1,
        0,
        1,
        0,
        -1,
        1,
        0,
        -1,
        1,
        1,
        1,
        0,
        0,
        0,
        -1,
        1,
        1,
        0,
        0,
        1,
        -1,
        0,
        1,
    ];

    static BYTES: [u8; 6] = [-60_i8 as u8, 57, 114, 54, -53_i8 as u8, 3];

    #[test]
    fn trits_bytes_inplace() {
        use core::mem;

        let trits = IN.to_vec();
        let trits_shadow = unsafe {
            Vec::from_raw_parts(trits.as_ptr() as *mut u8, trits.len(), trits.capacity())
        };

        let bytes = ByteTrits::inplace(trits);

        assert_eq!(
            bytes.data().to_vec(),
            trits_shadow[..bytes.data().len()]
                .iter()
                .map(|&t| t as u8)
                .collect::<Vec<u8>>()
        );

        // can't dealloc twice.
        mem::forget(trits_shadow);

    }

    #[test]
    fn bytes_to_trits() {
        let mut into_trits = [0 as Trit; 27];
        let slice = ByteTritsSlice::from_raw(&BYTES, 27);
        let vec: Vec<Trit> = slice.trits().collect();
        let bytetrits: ByteTrits = IN.iter().collect();

        slice.into_slice(&mut into_trits);

        assert_eq!(vec, IN.to_vec());
        assert_eq!(into_trits, IN);
        assert_eq!(bytetrits.data().to_vec(), BYTES.to_vec());
        assert_eq!(bytetrits.len(), 27);
    }
}
