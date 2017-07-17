use alloc::vec::Vec;
use alloc::string::String;

use constants::*;
use util::*;
use types::*;
use bct::trit_to_bct;

impl<'a> IntoTrits<Trit> for &'a str {
    fn len_trits(&self) -> usize {
        self.len() * TRITS_PER_TRYTE
    }

    fn trits(&self) -> Vec<Trit> {
        self.chars().flat_map(tryte_to_trits).cloned().collect()
    }
}

impl<'a> IntoTrits<BCTrit> for &'a str {
    fn len_trits(&self) -> usize {
        self.len() * TRITS_PER_TRYTE
    }

    fn trits(&self) -> Vec<BCTrit> {
        self.trits().into_iter().map(trit_to_bct).collect()
    }
}

#[allow(dead_code)]
pub fn trits_to_string(t: &[Trit]) -> Option<String> {
    if t.len() % 3 != 0 {
        return None;
    }

    Some(t.chunks(TRITS_PER_TRYTE).map(trits_to_char).collect())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fromtostr_test1() {
        let trytes = "UYSSM9KIH";
        let back = trytes.trits();
        assert_eq!(trits_to_string(&back).unwrap(), trytes);
    }

    #[test]
    fn fromtostr_test2() {
        let trytes = "LCUNQ99HCQM9HSATSQOPOWXXKGKDVZSEKVWJZRGWFRVLEQ9XG9INIPKAM9BQVGCIRNPZOS9LUZRBHB9P";
        let back = trytes.trits();
        assert_eq!(trits_to_string(&back).unwrap(), trytes);
    }

    #[test]
    fn fromtostr_test3() {
        let trytes = "RSWWSFXPQJUBJROQBRQZWZXZJWMUBVIVMHPPTYSNW9YQI";
        let back = trytes.trits();
        assert_eq!(trits_to_string(&back).unwrap(), trytes);
    }
}
