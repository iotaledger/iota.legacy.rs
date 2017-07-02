use tmath::*;
use trytes::*;
use alloc::Vec;
use curl::*;
use cpucurl::*;
//use collections::String;
//

trait Offset {
    fn offset(&mut self);
}

impl<'a> Offset for &'a mut [BCTrit] {
    fn offset(&mut self) {
        self[0].0 = 0b1101101101101101101101101101101101101101101101101101101101101101;
        self[0].1 = 0b1011011011011011011011011011011011011011011011011011011011011011;
        self[1].0 = 0b1111000111111000111111000111111000111111000111111000111111000111;
        self[1].1 = 0b1000111111000111111000111111000111111000111111000111111000111111;
        self[2].0 = 0b0111111111111111111000000000111111111111111111000000000111111111;
        self[2].1 = 0b1111111111000000000111111111111111111000000000111111111111111111;
        self[3].0 = 0b1111111111000000000000000000000000000111111111111111111111111111;
        self[3].1 = 0b0000000000111111111111111111111111111111111111111111111111111111;
    }
}

fn min(this: usize, that: usize) -> usize {
    if this < that { this } else { that }
}

pub fn search_cpu<F>(input: Trinary, length: usize, group: usize, check: F) -> Option<Trinary>
where
    F: Fn(&[BCTrit]) -> Option<usize>,
{
    let mut curl = CpuCurl::<BCTrit>::default();
    let bct: Vec<BCTrit> = input.trits();
    curl.absorb(&bct); // should end before last absorb
    let mut end = min(length, HASH_LENGTH);
    (&mut curl.state[0..4]).offset();
    for _ in 0..group {
        (&mut curl.state[(end / 3)..(end * 2 / 3)]).incr();
    }
    let mut index: Option<usize>;
    loop {
        end = min((&mut curl.state[(end * 2 / 3)..end]).incr(), HASH_LENGTH);
        let mut curl_copy = curl.clone();
        curl_copy.transform();
        index = check(&curl_copy.state[0..end]);
        if index.is_some() {
            break;
        }
    }

    let mux = TrinaryDemultiplexer::new(curl.squeeze(end).as_slice());

    Some(mux[index.unwrap()].clone())
}
