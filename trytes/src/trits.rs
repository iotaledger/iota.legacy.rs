use constants::{Trit, TRITS_PER_BYTE};
use byte::*;

pub struct ByteTritsSlice<'a> {
    data: &'a [u8],
    length: usize,
}

pub struct ByteTritsSliceIter<'a> {
    slice: &'a ByteTritsSlice<'a>,
    pos: usize,
}

impl<'a> Iterator for ByteTritsSliceIter<'a> {
    type Item = Trit;
    fn next(&mut self) -> Option<Trit> {
        if self.pos >= self.len() {
            None
        } else {
            let q = self.pos / TRITS_PER_BYTE;
            let r = self.pos % TRITS_PER_BYTE;
            let ret = byte_to_trits(self.slice.data[q])[r];
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
}

/// Generic trait for creating a trit iterator out of a struct
trait ToTrits<'a> {
    type Iter: ExactSizeIterator<Item = Trit>;
    fn trits(&'a self) -> Self::Iter;
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

    pub struct ByteTrits {
        data: Vec<u8>,
        length: usize,
    }

    impl ByteTrits {
        pub fn as_slice<'a>(&'a self) -> ByteTritsSlice<'a> {
            ByteTritsSlice {
                data: &self.data,
                length: self.length,
            }
        }

        pub fn data(&self) -> &[u8] {
            &self.data
        }

        pub fn len(&self) -> usize {
            self.length
        }
    }

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

    #[test]
    fn bytes_to_trits() {
        let bytes: [u8; 6] = [-60_i8 as u8, 57, 114, 54, -53_i8 as u8, 3];
        let exp: [Trit; 27] = [
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


        let slice = ByteTritsSlice::from_raw(&bytes, 27);
        let vec: Vec<Trit> = slice.trits().collect();
        let bytetrits: ByteTrits = exp.iter().collect();


        assert_eq!(vec, exp.to_vec());
        assert_eq!(bytetrits.data().to_vec(), bytes.to_vec());
        assert_eq!(bytetrits.len(), 27);
    }


}
