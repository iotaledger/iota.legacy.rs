use curl::*;
use copy::*;
use trytes::*;
use search::*;

pub struct CpuHam;

impl HammingNonce<Trit> for CpuHam {
    fn search<C: Curl<Trit>, CB: Curl<BCTrit>>(
        security: u8,
        offset: usize,
        length: usize,
        tcurl: &mut C,
        bcurl: &mut CB,
    ) -> Option<usize> {

        search_prepare_trits(tcurl, bcurl, offset);

        search_cpu(
            &mut tcurl.state_mut()[..HASH_LENGTH],
            bcurl,
            offset,
            length,
            0,
            move |t: &[BCTrit]| {
                let mux = TrinaryDemultiplexer::new(t);
                for i in 0..mux.len() {
                    let mut sum = 0;
                    for j in 0..security as usize {
                        sum += mux.get(i)
                            .skip(j * HASH_LENGTH / 3)
                            .take(HASH_LENGTH / 3)
                            .sum::<Trit>();
                        if sum == 0 && j < security as usize - 1 {
                            sum = 1;
                            break;
                        }
                    }
                    if sum == 0 {
                        return Some(i);
                    }
                }
                None
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpucurl::*;
    use curl_tests;

    #[test]
    pub fn run_testsuite() {
        curl_tests::run_ham_search::<CpuHam, CpuCurl<Trit>, CpuCurl<BCTrit>>();
    }

}
