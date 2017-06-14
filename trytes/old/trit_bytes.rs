use globals::*;

pub trait TritHasBytes {
    fn bytes_at_range(&self, offset: usize, size: usize) -> Vec<Trit>;
    fn bytes(&self) -> Vec<Trit>;
}


impl TritHasBytes for [Trit] {
    fn bytes_at_range(&self, offset: usize, size: usize) -> Vec<Trit> {
        let length = (size + NUMBER_OF_TRITS_IN_A_BYTE - 1) / NUMBER_OF_TRITS_IN_A_BYTE;
        let mut res = vec![0; length];
        let mut value: Trit;

        for i in 0..length {
            value = 0;
            for j in 0..
                     if size - i * NUMBER_OF_TRITS_IN_A_BYTE < 5 {
                         size - i * NUMBER_OF_TRITS_IN_A_BYTE
                     } else {
                         NUMBER_OF_TRITS_IN_A_BYTE
                     } {
                value = value * RADIX + self[offset + i * NUMBER_OF_TRITS_IN_A_BYTE + j];
            }
            res[i] = value;
        }
        res
    }
    fn bytes(&self) -> Vec<Trit> {
        self.bytes_at_range(0, self.len())
    }
}

impl TritHasBytes for Vec<Trit> {
    fn bytes_at_range(&self, offset: usize, size: usize) -> Vec<Trit> {
        let length = (size + NUMBER_OF_TRITS_IN_A_BYTE - 1) / NUMBER_OF_TRITS_IN_A_BYTE;
        let mut res = vec![0; length];
        let mut value: Trit;

        for i in 0..length {
            value = 0;
            for j in 0..
                     if size - i * NUMBER_OF_TRITS_IN_A_BYTE < 5 {
                         size - i * NUMBER_OF_TRITS_IN_A_BYTE
                     } else {
                         NUMBER_OF_TRITS_IN_A_BYTE
                     } {
                value = value * RADIX + self[offset + i * NUMBER_OF_TRITS_IN_A_BYTE + j];
            }
            res[i] = value;
        }
        res
    }
    fn bytes(&self) -> Vec<Trit> {
        self.bytes_at_range(0, self.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bytes_at_range_works() {
        let in_trytes = vec![0, 1, -1, 1, -1, 0, 1, 0, -1, 1, 0, -1, 1, 1, 1, 0, 0, 0, -1, -1, 1,
                             0, 0, 1, -1, 0, 1];
        let res = in_trytes.bytes_at_range(3, 21);
        let exp: [Trit; 5] = [57, -56, 108, -99, 1];
        assert_eq!(res.as_slice(), exp);
    }

    #[test]
    fn bytes_from_vec_works() {
        let in_trytes = vec![0, 1, -1, 1, -1, 0, 1, 0, -1, 1, 0, -1, 1, 1, 1, 0, 0, 0, -1, -1, 1,
                             0, 0, 1, -1, 0, 1];
        let res = in_trytes.bytes();
        let exp: [Trit; 6] = [20, 25, -14, -4, 83, 1];
        assert_eq!(res.as_slice(), exp);
    }

    #[test]
    fn bytes_range_from_vec_slice_works() {
        let in_trytes = vec![0, 1, -1, 1, -1, 0, 1, 0, -1, 1, 0, -1, 1, 1, 1, 0, 0, 0, -1, -1, 1,
                             0, 0, 1, -1, 0, 1];
        let res = in_trytes.as_slice().bytes_at_range(3, 21);
        let exp: [Trit; 5] = [57, -56, 108, -99, 1];
        assert_eq!(res, exp);
        //println!("{:?}", res);
    }

    #[test]
    fn bytes_from_vec_slice_works() {
        let in_trytes = vec![0, 1, -1, 1, -1, 0, 1, 0, -1, 1, 0, -1, 1, 1, 1, 0, 0, 0, -1, -1, 1,
                             0, 0, 1, -1, 0, 1];
        let in_slice = in_trytes.as_slice();
        let res = in_slice.bytes();
        let exp: [Trit; 6] = [20, 25, -14, -4, 83, 1];
        assert_eq!(res.as_slice(), exp);
        //println!("{:?}", res);
    }

    #[test]
    fn small_slice_bytes() {
        let in_trytes = vec![1, 1, 1, 1, 0, 0];
        let in_slice = in_trytes.as_slice();
        let res = in_slice.bytes();
        /*
        let exp: [Trit; 6] = [20, 25, -14, -4, 83, 1];
        assert_eq!(res.as_slice(), exp);
        */
        println!("{:?}", res);
    }
}
