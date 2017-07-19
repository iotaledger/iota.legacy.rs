use trytes::*;
use cpucurl::*;
use core::cmp::min;
use alloc::Vec;

#[cfg(not(feature = "parallel"))]
mod cpu_search {
    use super::*;
    use curl::{Curl, Sponge};
    use tmath::*;
    pub fn search_cpu<F>(input: &[BCTrit], length: usize, group: usize, check: F) -> Option<Vec<Trit>>
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
                num::round_third(
                    size * 2 / 3 + (&mut curl.state[(size * 2 / 3)..size]).incr(),
                ),
                HASH_LENGTH,
            );
            let mut cpy = curl.clone();
            cpy.transform();
            index = check(&cpy.state[..HASH_LENGTH]);
        }

        let mux = TrinaryDemultiplexer::<Vec<Trit>>::new(curl.squeeze(size).as_slice());

        Some(mux[index.unwrap()].clone())
    }
}

#[cfg(feature = "parallel")]
mod cpu_search {
    use super::*;
    use tmath::*;
    use curl::{Curl, Sponge};
    use std::thread;
    use std::sync::mpsc::channel;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use core::marker::*;
    pub fn search_cpu<F>(input: &[BCTrit], length: usize, group: usize, check: F) -> Option<Trinary>
    where
        F: Fn(&[BCTrit]) -> Option<usize> + 'static + Send + Sync,
    {
        let running = AtomicBool::new(true);
        let check_arc = Arc::new(check);
        let (tx, rx) = channel();
        let handles: Vec<thread::JoinHandle<_>> = (0..8)
            .into_iter()
            .map(|i| {
                let mut curl = CpuCurl::<BCTrit>::default();
                curl.state.clone_from_slice(input);
                let mut size = min(length, HASH_LENGTH);
                let child_tx = tx.clone();
                let child_group = i + group;
                let check_clone = check_arc.clone();
                thread::spawn(move || {
                    for _ in 0..child_group {
                        (&mut curl.state[(size / 3)..(size * 2 / 3)]).incr();
                    }
                    let mut index: Option<usize> = None;
                    while index.is_none() && running.load(Ordering::SeqCst) {
                        size = min(
                            num::round_third(
                                size * 2 / 3 + (&mut curl.state[(size * 2 / 3)..size]).incr(),
                            ),
                            HASH_LENGTH,
                        );
                        let mut cpy = curl.clone();
                        cpy.transform();
                        index = check_clone(&cpy.state[..HASH_LENGTH]);
                    }
                    if running.load(Ordering::SeqCst) && index.is_some() {
                        running.store(false, Ordering::SeqCst);
                        let mux = TrinaryDemultiplexer::new(curl.squeeze(size).as_slice());
                        child_tx.send(Some(mux[index.unwrap()].clone())).unwrap();
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        rx.recv().unwrap()
    }
}

pub use self::cpu_search::search_cpu;
