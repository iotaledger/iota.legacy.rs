use globals::*;
use mappings::*;

trait TrytesHaveTrits {
    fn trits(&self) -> Vec<Trit>;
}

trait TryteHasTrits {
    fn trits(&self) -> [Trit; NUMBER_OF_TRITS_IN_A_TRYTE];
}

impl<'a> TrytesHaveTrits for &'a str {
    fn trits(&self) -> Vec<Trit> {
        self.chars().map(|c| c.trits()).fold(Vec::new(), |mut acc, x| {
            acc.extend_from_slice(&x);
            acc
        })
    }
}

impl TryteHasTrits for char {
    fn trits(&self) -> [Trit; NUMBER_OF_TRITS_IN_A_TRYTE] {
        TRYTE_TO_TRITS_MAPPINGS[TRYTE_ALPHABET.find(*self).unwrap()]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tryte_to_trits_works() {
        let tryte = 'A';
        let exp: [Trit; 3] = [1, 0, 0];
        let res = tryte.trits();
        assert_eq!(res, exp);
    }

    #[test]
    fn trytes_to_trits_works() {
        let trytes = "ABC";
        let exp: [Trit; 9] = [1, 0, 0, -1, 1, 0, 0, 1, 0];
        let res = trytes.trits();
        assert_eq!(res.as_slice(), exp);
    }
}
