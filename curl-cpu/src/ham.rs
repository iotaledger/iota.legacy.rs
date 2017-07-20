use curl::*;
use cpucurl::CpuCurl;
use tmath::*;
use trytes::*;
use search::*;
use alloc::*;

pub struct CpuHam;

fn prepare_search(input: &[Trit]) -> Vec<BCTrit> {
    let mut curl = CpuCurl::<Trit>::default();
    let length_trits: Vec<Trit> = {
        let l = (input.len() / TRITS_PER_TRYTE) as isize;
        num::int2trits(l, num::min_trits(l))
    };
    curl.absorb(length_trits.as_slice());
    curl.absorb(input);
    let mut state: Vec<BCTrit> = curl.state.iter().cloned().map(trit_to_bct).collect();
    (&mut state[0..4]).offset();
    state
}

impl HammingNonce<Trit> for CpuHam {
    fn search(input: &[Trit], length: u8, security: u8) -> Option<Vec<Trit>> {
        let state = prepare_search(input);
        search_cpu(state.as_slice(), length as usize, 0, move |t: &[BCTrit]| {
            let mux = TrinaryDemultiplexer::new(t);
            for i in 0..mux.len() {
                let trits: Vec<Trit> = mux.get(i).collect();
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
