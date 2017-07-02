use tmath::*;
use trytes::*;
use search::Search;
use curl::*;
use alloc::Vec;
use core;
//use collections::String;

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

struct ThreadSearch;

impl Search<BCTrit> for ThreadSearch {
    fn search(
        trits: &[Trit],
        length: usize,
        group: usize,
        check: fn(&[BCTrit]) -> Option<usize>,
    ) -> Option<Trinary> {
        let mut curl = DefaultCurl::default();
        (&mut curl.state[0..3]).offset();
        for _ in 0..group {
            (&mut curl.state[HASH_LENGTH / 3..HASH_LENGTH * 2 / 3]).incr();
        }
        let mut index: Option<usize>;
        loop {
            (&mut curl.state[HASH_LENGTH * 2 / 3..HASH_LENGTH]).incr();
            let mut curl_copy = curl.clone();
            curl_copy.transform();
            index = check(&curl_copy.state[0..length]);
            if index.is_some() {
                break;
            }
        }

        let mux = TrinaryDemultiplexer::new(curl.squeeze(HASH_LENGTH).as_slice());

        Some(mux[index.unwrap()].clone())
    }
}
