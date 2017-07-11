use curl::*;
use cpucurl::CpuCurl;
use trytes::*;
use search::*;
use alloc::*;
use core::mem;

pub struct CpuHam;

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
            let mux = TrinaryDemultiplexer::new(t);
            for i in 0..(mem::size_of::<usize>() * 8) {
                let trits: Vec<Trit> = mux[i].trits();
                match security {
                    1 => {
                        if trits[..(t.len() / 3)].iter().fold(0, |acc, x| acc + x) == 0 {
                            return Some(i);
                        }
                    }
                    2 => {
                        if trits[..(t.len() / 3)].iter().fold(0, |acc, x| acc + x) != 0 {
                            if trits[..(2 * t.len() / 3)].iter().fold(0, |acc, x| acc + x) == 0 {
                                return Some(i);
                            }
                        }
                    }
                    3 => {
                        if trits[..(t.len() / 3)].iter().fold(0, |acc, x| acc + x) != 0 {
                            if trits[..(2 * t.len() / 3)].iter().fold(0, |acc, x| acc + x) != 0 {
                                if trits.iter().fold(0, |acc, x| acc + x) == 0 {
                                    return Some(i);
                                }
                            }
                        }
                    }
                    _ => {}
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
