use trytes::*;
use curl::*;
use curl_cpu::*;
use sign::iss;
use core::mem;
use alloc::*;

pub struct Merkle {
    root: Trinary,
    key: Vec<Trit>,
    hashes: Vec<Trinary>,
}

impl Merkle {
    fn new(seed: Trinary, start: usize, count: usize, index: usize, security: u8) -> Merkle {
        let usize_size = mem::size_of::<usize>() * 8;
        let loop_count = usize_size / count + if count % usize_size == 0 { 1 } else { 2 };

        let mut key: Vec<Trit>;
        let mut addresses: Vec<Vec<BCTrit>> =
            Vec::with_capacity(loop_count + if loop_count & 1 == 0 { 0 } else { 1 });
        {
            let mut keys: Vec<Vec<BCTrit>> = Vec::with_capacity(loop_count);
            for i in 0..loop_count {
                let subseeds: Vec<BCTrit> =
                    iss::subseeds::<CpuCurl<BCTrit>>(&seed.trits(), i * usize_size + start);
                keys.push(iss::key::<BCTrit, CpuCurl<BCTrit>>(&subseeds));
                addresses.push(iss::address::<BCTrit, CpuCurl<BCTrit>>(
                    &iss::digest_key::<BCTrit, CpuCurl<BCTrit>>(&keys[i]),
                ));
            }
            key = TrinaryDemultiplexer::new(&keys[index / usize_size])[index % usize_size].trits();
        }
        if loop_count & 1 != 0 {
            addresses.push(vec![(0, 0); HASH_LENGTH]);
        }

        let mut hashes: Vec<Trinary> = compute_sibling_hashes(&addresses, count, index);
        let root: Trinary = key.clone().into_iter().collect();
        Merkle { root, key, hashes }
    }
    fn root(address: Trinary, hashes: Vec<Trinary>, index: usize) -> Trinary {
        let mut curl = CpuCurl::<Trit>::default();
        let mut i = 1;
        let mut output = address.trits();
        for hash in hashes {
            match i & index {
                0 => {
                    curl.absorb(&output);
                    curl.absorb(&hash.trits());
                }
                _ => {
                    curl.absorb(&hash.trits());
                    curl.absorb(&output);
                }
            }
            i <<= 1;
            output = curl.squeeze(HASH_LENGTH);
            curl.reset();
        }
        output.into_iter().collect()
    }
}

fn compute_sibling_hashes(addresses: &[Vec<BCTrit>], count: usize, index: usize) -> Vec<Trinary> {
    let usize_size = mem::size_of::<usize>() * 8;
    let mut hashes: Vec<Trinary> = Vec::with_capacity(usize_size - count.leading_zeros() as usize);
    let mut end = count;
    let mut merkle_curl = CpuCurl::<BCTrit>::default();
    let mut remaining: Vec<Vec<BCTrit>> = Vec::with_capacity(addresses.len());
    remaining.extend_from_slice(addresses);
    let mut hash_index: usize = if index & 1 == 0 { index + 1 } else { index - 1 };
    while end != 1 {
        let mux = TrinaryDemultiplexer::new(&remaining[hash_index / usize_size]);
        hashes.insert(0, mux[hash_index % usize_size].clone());
        hash_index = if hash_index & 1 == 0 {
            hash_index >> 1 + 1
        } else {
            hash_index >> 1 - 1
        };
        let half = end >> 1;
        end = if half != 1 { half + end % 1 } else { half };
        remaining = reduce(&remaining, end)
            .iter()
            .map(|t| {
                merkle_curl.absorb(&t);
                let out = merkle_curl.squeeze(HASH_LENGTH);
                merkle_curl.reset();
                out
            })
            .collect();
    }
    hashes
}

fn reduce(hashes: &[Vec<BCTrit>], end: usize) -> Vec<Vec<BCTrit>> {
    let size = hashes.len();
    let usize_size = mem::size_of::<usize>() * 8;
    match size {
        1 => {
            let half = usize_size - end >> 1;
            let mask = usize::max_value() >> half;
            let mut combined: Vec<BCTrit> = Vec::with_capacity(hashes[0].len() * 2);
            let first: Vec<BCTrit> = hashes[0].iter().map(|a| (a.0 & mask, a.1 & mask)).collect();
            let second: Vec<BCTrit> = hashes[0]
                .iter()
                .map(|a| ((a.0 & !mask) >> half, (a.1 & !mask) >> half))
                .collect();
            combined.extend(first);
            combined.extend(second);
            vec![combined]
        }
        _ => {
            let half = usize_size >> 1;
            let mask = usize::max_value() >> half;
            (0..size)
                .take_while(|x| x & 1 == 0)
                .map(|i| {
                    let mut combined: Vec<BCTrit> = Vec::with_capacity(hashes[i].len() * 2);
                    let first: Vec<BCTrit> = hashes[i]
                        .iter()
                        .zip(hashes[i + 1].iter())
                        .map(|(a, b)| {
                            (
                                (a.0 & mask) | ((b.0 & mask) << half),
                                (a.1 & mask) | ((b.1 & mask) << half),
                            )
                        })
                        .collect();
                    let second: Vec<BCTrit> = hashes[i]
                        .iter()
                        .zip(hashes[i + 1].iter())
                        .map(|(a, b)| {
                            (
                                ((b.0 & !mask) | ((a.0 & !mask) >> half)),
                                ((b.1 & !mask) | ((a.1 & !mask) >> half)),
                            )
                        })
                        .collect();
                    combined.extend(first);
                    combined.extend(second);
                    combined
                })
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_does_not_panic() {
        let seed: Trinary = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9\
                             ABCDEFGHIJKLMNOPQRSTUVWXYZ9\
                             ABCDEFGHIJKLMNOPQRSTUVWXYZ9"
            .chars()
            .collect();
        let start = 1;
        let count = 67;
        let index = 65;
        let security = 1;
        Merkle::new(seed, start, count, index, security);
    }
}
