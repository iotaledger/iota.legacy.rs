use curl::{ProofOfWork, Curl};
use cpucurl::CpuCurl;
use trytes::*;
use search::*;
use alloc::Vec;

pub struct CpuPoW;

fn prepare_search(input: &[Trit]) -> Vec<BCTrit> {
    let mut curl = CpuCurl::<Trit>::default();
    let size = if input.len() % HASH_LENGTH == 0 {
        input.len() - HASH_LENGTH
    } else {
        HASH_LENGTH * (input.len() / HASH_LENGTH)
    };
    curl.absorb(&input[..size]);
    let trinary: Trinary = curl.state.iter().cloned().collect();
    let mut state: Vec<BCTrit> = trinary.trits();
    (&mut state[0..4]).offset();
    state
}


impl ProofOfWork for CpuPoW {
    fn search(input: &[Trit], weight: u8) -> Option<Trinary> {
        let state = prepare_search(input);
        search_cpu(state.as_slice(), HASH_LENGTH, 0, move |t: &[BCTrit]| {
            let mut probe = usize::max_value();
            let wt: usize = weight as usize;
            let start = t.len() - wt;
            for i in (start)..t.len() {
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
        curl::tests::run_search::<CpuPoW, CpuCurl<Trit>>();
    }

}
