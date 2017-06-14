use globals::*;
use mappings::*;

pub fn tryte_to_trits(trit: char) -> [Trit; NUMBER_OF_TRITS_IN_A_TRYTE] {
    TRYTE_TO_TRITS_MAPPINGS[TRYTE_ALPHABET.find(trit).unwrap()]
}
pub fn trytes_to_trits(trytes: &str) -> Vec<Trit> {
    trytes
        .chars()
        .map(|c| tryte_to_trits(c))
        .fold(Vec::new(), |mut acc, x| {
            acc.extend_from_slice(&x);
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tryte_to_trits_works() {
        let tryte = 'A';
        let exp: [Trit; 3] = [1, 0, 0];
        let res = tryte_to_trits(tryte);
        assert_eq!(res, exp);
    }

    #[test]
    fn trytes_to_trits_works() {
        let trytes = "ABC";
        let exp: [Trit; 9] = [1, 0, 0, -1, 1, 0, 0, 1, 0];
        let res = trytes_to_trits(trytes);
        assert_eq!(res.as_slice(), exp);
    }
}
