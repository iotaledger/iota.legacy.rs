use curl::*;
use cpucurl::CpuCurl;
use trytes::offset::*;
use trytes::*;
use search::*;
use alloc::*;
use core::mem;

pub struct CpuHam;

fn prepare_search(input: &IntoTrits<Trit>) -> Vec<BCTrit> {
    let mut curl = CpuCurl::<Trit>::default();
    let trits: Vec<Trit> = input.trits();
    let length_trits: Vec<Trit> = {
        let l = (trits.len() / TRITS_PER_TRYTE) as isize;
        num::int2trits(l, num::min_trits(l))
    };
    curl.absorb(length_trits.as_slice());
    curl.absorb(trits.as_slice());
    let mut state: Vec<BCTrit> = curl.state.to_vec().trits();
    (&mut state[0..4]).offset();
    state
}

impl HammingNonce<Trit> for CpuHam {
    fn search(input: &IntoTrits<Trit>, length: u8, security: u8) -> Option<Vec<Trit>> {
        let state = prepare_search(&input.trits());
        search_cpu(state.as_slice(), length as usize, 0, move |t: &[BCTrit]| {
            let mux = TrinaryDemultiplexer::<Vec<Trit>>::new(t);
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
