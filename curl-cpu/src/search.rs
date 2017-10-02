use trytes::*;
use core::cmp::min;

#[cfg(not(feature = "parallel"))]
mod cpu_search {
    use super::*;
    use curl::Curl;
    use tmath::*;
    pub fn search_cpu<F, CB: Curl<BCTrit>>(
        out: &mut [Trit],
        curl: &mut CB,
        offset: usize,
        length: usize,
        group: usize,
        check: F,
    ) -> Option<usize>
    where
        F: Fn(&[BCTrit]) -> Option<usize>,
    {
        let mut size = min(length, HASH_LENGTH) - offset;
        for _ in 0..group {
            (&mut curl.state_mut()[offset + size / 3..offset + size * 2 / 3]).incr();
        }
        let mut index: Option<usize> = None;
        while index.is_none() {
            size = min(
                num::round_third(
                    (offset + size * 2 / 3 +
                        (&mut curl.state_mut()[offset + size * 2 / 3..offset + size]).incr()) as i64,
                ) as usize,
                HASH_LENGTH,
            ) - offset;
            let mut cpy = curl.clone();
            cpy.transform();
            index = check(&cpy.state()[..HASH_LENGTH]);
        }

        let mux = TrinaryDemultiplexer::new(&curl.rate()[0..size]);

        for (i, v) in mux.get(index.unwrap()).enumerate() {
            out[i] = v;
        }

        Some(size)
    }
}

#[cfg(feature = "parallel")]
mod cpu_search {
    use super::*;
    use std::thread;
    use std::sync::mpsc::channel;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::vec::Vec;
    use core::marker::*;
    use num_cpus;

    use tmath::*;
    use curl::Curl;

    pub fn search_cpu<F, CB: Curl<BCTrit>>(
        out: &mut [Trit],
        curl: &mut CB,
        offset: usize,
        length: usize,
        group: usize,
        check: F,
    ) -> Option<usize>
    where
        F: Fn(&[BCTrit]) -> Option<usize> + 'static + Send + Sync,
    {
        let running = AtomicBool::new(true);
        let check_arc = Arc::new(check);
        let running_arc = Arc::new(running);
        let (tx, rx) = channel();
        let handles: Vec<thread::JoinHandle<_>> = (0..num_cpus::get())
            .into_iter()
            .map(|i| {
                let mut curl = curl.clone();
                let mut size = min(length, HASH_LENGTH) - offset;
                let child_tx = tx.clone();
                let child_group = i + group;
                let check_clone = check_arc.clone();
                let running_clone = running_arc.clone();

                thread::spawn(move || {
                    for _ in 0..child_group {
                        (&mut curl.state_mut()[offset + size / 3..offset + size * 2 / 3]).incr();
                    }
                    let mut index: Option<usize> = None;
                    while index.is_none() && running_clone.load(Ordering::SeqCst) {
                        size = min(
                            num::round_third(
                                offset + size * 2 / 3 +
                                    (&mut curl.state_mut()[offset + size * 2 / 3..offset + size])
                                        .incr(),
                            ),
                            HASH_LENGTH,
                        ) - offset;
                        let mut cpy = curl.clone();
                        cpy.transform();
                        index = check_clone(&cpy.state()[..HASH_LENGTH]);
                    }
                    if running_clone.load(Ordering::SeqCst) && index.is_some() {
                        running_clone.store(false, Ordering::SeqCst);
                        let mut tmp = vec![(0, 0); size];
                        curl.squeeze(tmp.as_mut_slice());
                        let mux = TrinaryDemultiplexer::new(tmp.as_slice());
                        child_tx
                            .send(mux.get(index.unwrap()).collect::<Vec<Trit>>())
                            .unwrap();
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }

        if let Some(nonce) = rx.recv().ok() {
            out.clone_from_slice(nonce.as_slice());
            Some(nonce.len())
        } else {
            None
        }
    }
}

pub use self::cpu_search::search_cpu;
