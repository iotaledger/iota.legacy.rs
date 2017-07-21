use curl::{STATE_LENGTH, ProofOfWork, Curl};
use cpucurl::CpuCurl;
use trytes::*;
use search::*;
use tmath::*;

pub struct CpuPoW;

fn prepare_search(input: &[Trit], out: &mut [BCTrit]) {
    let mut curl = CpuCurl::<Trit>::default();
    let size = if input.len() % HASH_LENGTH == 0 {
        input.len() - HASH_LENGTH
    } else {
        HASH_LENGTH * (input.len() / HASH_LENGTH)
    };
    curl.absorb(&input[..size]);

    for (&t, mut bct) in curl.state.iter().zip(out.iter_mut()) {
       *bct = trit_to_bct(t); 
    }

    (&mut out[0..4]).offset();
}


impl ProofOfWork<Trit> for CpuPoW {
    fn search(input: &[Trit], weight: u8, out: &mut [Trit]) -> bool {
        let mut bct : [BCTrit; STATE_LENGTH] = [(0,0); STATE_LENGTH];
        prepare_search(input, &mut bct);
        search_cpu(&mut bct, out, 0, move |t: &[BCTrit]| {
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
    use cpucurl::*;
    use curl_tests;

    #[test]
    pub fn run_testsuite() {
        curl_tests::run_search::<CpuPoW, CpuCurl<Trit>>();
    }

}
