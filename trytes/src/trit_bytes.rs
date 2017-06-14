use globals::*;
use trinary::Trinary;

pub fn trits_at_range_to_bytes(trits: &[Trit], offset: usize, size: usize) -> Vec<i8> {
    let length = (size + NUMBER_OF_TRITS_IN_A_BYTE - 1) / NUMBER_OF_TRITS_IN_A_BYTE;
    let mut res: Vec<i8> = vec![0; length];
    let mut value: Trit;

    for i in 0..length {
        value = 0;
        for j in 0..
                 if size - i * NUMBER_OF_TRITS_IN_A_BYTE < 5 {
                     size - i * NUMBER_OF_TRITS_IN_A_BYTE
                 } else {
                     NUMBER_OF_TRITS_IN_A_BYTE
                 } {
            value = value * RADIX + trits[offset + i * NUMBER_OF_TRITS_IN_A_BYTE + j];
        }
        res[i] = value as i8;
    }
    res
}
pub fn trits_to_bytes(trits: &[Trit]) -> Vec<i8> {
    trits_at_range_to_bytes(trits, 0, trits.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bytes_at_range_works() {
        let in_trits = vec![0, 1, -1, 1, -1, 0, 1, 0, -1, 1, 0, -1, 1, 1, 1, 0, 0, 0, -1, -1, 1,
                            0, 0, 1, -1, 0, 1];
        let res = trits_at_range_to_bytes(in_trits.as_slice(), 3, 21);
        let exp: [i8; 5] = [57, -56, 108, -99, 1];
        assert_eq!(res.as_slice(), exp);
    }

    #[test]
    fn bytes_from_vec_works() {
        let in_trits = vec![0, 1, -1, 1, -1, 0, 1, 0, -1, 1, 0, -1, 1, 1, 1, 0, 0, 0, -1, -1, 1,
                            0, 0, 1, -1, 0, 1];
        let res = trits_to_bytes(in_trits.as_slice());
        let exp: [i8; 6] = [20, 25, -14, -4, 83, 1];
        assert_eq!(res.as_slice(), exp);
    }

    #[test]
    fn small_slice_bytes() {
        let in_trits = vec![1, 1, 1, 1, 0, 0];
        let res = trits_to_bytes(in_trits.as_slice());
        /*
        let exp: [Trit; 6] = [20, 25, -14, -4, 83, 1];
        assert_eq!(res.as_slice(), exp);
        */
        println!("{:?}", res);
    }
}
