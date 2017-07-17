use trytes::*;
use tmath::*;
use curl::*;
use curl_cpu::*;
use sign::iss;
use core::mem;
use alloc::*;

pub fn keys(seed: &[Trit], start: usize, count: usize) -> Vec<Vec<Trit>> {
    let mut trits: Vec<Trit> = seed.to_vec();
    for _ in 0..start {
        trits.as_mut_slice().incr();
    }
    let mut out: Vec<Vec<Trit>> = Vec::with_capacity(count);
    for _ in 0..count {
        let subseed = iss::subseed::<CpuCurl<Trit>>(&trits, 0);
        trits.as_mut_slice().incr();
        out.push(iss::key::<Trit, CpuCurl<Trit>>(&subseed));
    }
    out
}

pub fn siblings(addrs: &[Vec<Trit>], index: usize) -> Vec<Vec<Trit>> {
    let usize_size = mem::size_of::<usize>() * 8;
    let hash_count = usize_size - index.leading_zeros() as usize;
    let mut out: Vec<Vec<Trit>> = Vec::with_capacity(hash_count);
    let mut curl = CpuCurl::<Trit>::default();
    let mut hash_index = if index & 1 == 0 { index + 1 } else { index - 1 };
    let mut hashes: Vec<Vec<Trit>> = Vec::with_capacity(addrs.len());
    hashes.extend_from_slice(addrs);
    let mut length = hashes.len();
    while length > 1 {
        if length & 1 == 1 {
            hashes.push(vec![0; HASH_LENGTH]);
            length += 1;
        }
        out.push(hashes[hash_index].clone());
        hash_index = hash_index / 2;
        if hash_index & 1 == 0 {
            hash_index += 1;
        } else {
            hash_index -= 1;
        }
        hashes = {
            let mut combined: Vec<Vec<Trit>> = Vec::with_capacity(length / 2);
            length /= 2;
            for hash_chunks in hashes.chunks(2) {
                for hash in hash_chunks {
                    curl.absorb(hash);
                }
                combined.push(curl.squeeze(HASH_LENGTH));
                curl.reset();
            }
            combined
        };
    }
    out
}

pub fn root(address: &[Trit], hashes: &[Vec<Trit>], index: usize) -> Vec<Trit> {
    let mut curl = CpuCurl::<Trit>::default();
    let mut i = 1;
    let mut output: Vec<Trit> = Vec::with_capacity(address.len());
    output.extend_from_slice(address);
    hashes.into_iter().fold(output, |acc, hash| {
        curl.reset();
        if i & index == 0 {
            curl.absorb(&acc);
            curl.absorb(&hash);
        } else {
            curl.absorb(&hash);
            curl.absorb(&acc);
        }
        i <<= 1;
        curl.squeeze(HASH_LENGTH)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use sign::iss;
    use alloc::string::ToString;
    #[test]
    fn it_does_not_panic() {
        let seed: Trinary = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9\
                             ABCDEFGHIJKLMNOPQRSTUVWXYZ9\
                             ABCDEFGHIJKLMNOPQRSTUVWXYZ9"
            .chars()
            .collect();
        let start = 1;
        let count = 9;
        let keys = keys(&seed.trits(), start, count);
        let addresses: Vec<Vec<Trit>> = keys.iter()
            .map(|k| {
                iss::address::<Trit, CpuCurl<Trit>>(&iss::digest_key::<Trit, CpuCurl<Trit>>(&k))
            })
            .collect();
        let hashes = siblings(&addresses, 0);
        let expect: Trinary = root(&addresses[0], &hashes, 0).into_iter().collect();
        for index in 0..count {
            let hashes = siblings(&addresses, index);
            let root: Trinary = root(&addresses[index], &hashes, index)
                .into_iter()
                .collect();
            assert_eq!(root.to_string(), expect.to_string());
        }
    }
}
