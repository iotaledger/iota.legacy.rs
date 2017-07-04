use curl::ProofOfWork;
use trytes::*;
use search::search_cpu;

pub struct CpuPoW;

impl ProofOfWork for CpuPoW {
    fn search(input: Trinary, weight: u8) -> Option<Trinary> {
        search_cpu(input, HASH_LENGTH, 0, |t: &[BCTrit]| {
            let mut probe = usize::max_value();
            let wt: usize = weight as usize;
            let start = t.len() - wt;

            for i in start..t.len(){
                probe &= !(t[i].0 ^ t[i].1);
                if probe == 0 {
                    return None;
                }
            }

            Some(probe.trailing_zeros() as usize)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::curl;
    use cpucurl::*;

    #[test]
    pub fn run_testsuite() {
        curl::tests::run_search::<CpuPoW, Trit, CpuCurl<Trit>>();
    }

}
