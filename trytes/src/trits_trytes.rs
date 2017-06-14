use globals::*;
use mappings::*;

pub fn trits_to_tryte(trits: &[Trit; NUMBER_OF_TRITS_IN_A_TRYTE]) -> char {
    TRYTE_ALPHABET
        .get(TRYTE_TO_TRITS_MAPPINGS.index_of(trits))
        .unwrap()
}
pub fn trits_to_trytes(trits: &[Trit]) -> String {
    trits
        .split(NUMBER_OF_TRITS_IN_A_TRYTE)
        .map(|t| trits_to_tryte(t))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tryte_to_trits_works() {
        let trits: [Trit; 3] = [1, 0, 0];
        let exp = 'A';
        let res = trits_to_tryte(trit);
        assert_eq!(res, exp);
    }

    #[test]
    fn trytes_to_trits_works() {
        let trits: [Trit; 9] = [1, 0, 0, -1, 1, 0, 0, 1, 0];
        let exp = "ABC";
        let res = trits_to_trytes(trits);
        assert_eq!(res.as_slice(), exp);
    }
}
