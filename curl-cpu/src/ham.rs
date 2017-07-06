use hamming::weight;
use curl::*;
use cpucurl::CpuCurl;
use search::Offset;
use trytes::*;
use search::*;
use alloc::*;
use core::mem;

//const word_length: usize = 64; //mem::size_of::<usize>() * 8;
#[inline(always)]
fn id(idx: usize) -> usize {
    1 << idx
}

pub struct CpuHam;
fn transpose(bctrits: &[BCTrit]) -> Vec<(Vec<u8>, Vec<u8>)> {
    let word_length = mem::size_of::<usize>() * 8;
    let byte_length = bctrits.len() / 8 + bctrits.len() % 8;
    (0..word_length)
        .into_iter()
        .map(|i| {
            let mut low = vec![0u8; byte_length];
            let mut hi = vec![0u8; byte_length];
            for j in 0..bctrits.len() {
                if bctrits[j].0 & id(i) != 0 {
                    low[j / 8] |= 1 << (j % 8);
                }
                if bctrits[j].1 & id(i) != 0 {
                    hi[j / 8] |= 1 << (j % 8);
                }
            }
            (low, hi)
        })
        .collect()
}

fn prepare_search(input: &[Trit]) -> Vec<BCTrit> {
    let mut curl = CpuCurl::<Trit>::default();
    let length_trits: Vec<Trit> = num::int2trits(input.len() as isize, 12);
    curl.absorb(length_trits.as_slice());
    curl.absorb(input);
    let trinary: Trinary = curl.state.iter().cloned().collect();
    let mut state: Vec<BCTrit> = trinary.trits();
    (&mut state[0..4]).offset();
    state
}

impl HammingNonce for CpuHam {
    fn search(input: &[Trit], length: u8, security: u8) -> Option<Trinary> {
        let state = prepare_search(input);
        search_cpu(state.as_slice(), length as usize, 0, move |t: &[BCTrit]| {
            let tp = transpose(t);
            for i in 0..tp.len() {
                if weight(tp[i].0.as_slice()) == weight(tp[i].1.as_slice()) {
                    return Some(i);
                }
            }
            None
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
        curl::tests::run_ham_search::<CpuHam, CpuCurl<Trit>>();
    }

}
