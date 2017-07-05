use tmath::*;
use trytes::*;
use alloc::Vec;
use curl::*;
use cpucurl::*;
use core::cmp::min;

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

pub fn prepare_search(input: &[BCTrit]) -> Vec<BCTrit> {
    let mut curl = CpuCurl::<BCTrit>::default();
    let size = if input.len() % HASH_LENGTH == 0 {
        input.len() - HASH_LENGTH
    } else {
        HASH_LENGTH * (input.len() / HASH_LENGTH)
    };
    curl.absorb(&input[..size]);
    (&mut curl.state[0..4]).offset();
    curl.state.into_iter().cloned().collect()
}

pub fn search_cpu<F>(input: &[BCTrit], length: usize, group: usize, check: F) -> Option<Trinary>
where
    F: Fn(&[BCTrit]) -> Option<usize>,
{
    let mut curl = CpuCurl::<BCTrit>::default();
    curl.state.clone_from_slice(input);
    let mut size = min(length, HASH_LENGTH);
    for _ in 0..group {
        (&mut curl.state[(size / 3)..(size * 2 / 3)]).incr();
    }
    let mut index: Option<usize> = None;
    while index.is_none() {
        size = min(
            size * 2 / 3 + (&mut curl.state[(size * 2 / 3)..size]).incr(),
            HASH_LENGTH,
        );
        let mut cpy = curl.clone();
        cpy.transform();
        index = check(&cpy.state[0..size]);
    }

    let mux = TrinaryDemultiplexer::new(curl.squeeze(size).as_slice());

    Some(mux[index.unwrap()].clone())
}
