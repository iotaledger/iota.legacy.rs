use trytes::bct::Offset;
use trytes::*;
use curl::*;
use curl_cpu::*;
use core::mem;

const carry: Vec<Trit> = vec![0; HASH_LENGTH];

pub struct Merkle {
    root: Trinary,
    key: Trinary,
    hashes: Vec<Trinary>,
}

impl Merkle {
    fn new(seed: Trinary, start: usize, count: usize, index: usize, security: u8) -> Merkle {
        let usize_size = mem::size_of::<usize>() * 8;
        let count = usize_size / count + if count % usize_size == 0 { 1 } else { 2 };
        let mut index_copy = index;
        let mut merkle_curl = CpuCurl::<BCTrit>::default();
        let mut keys: Vec<Vec<BCTrit>> = Vec::with_capacity(count);
        for i in 0..count {
            let mut subseed: Vec<BCTrit> = sign::subseed(seed, i * usize_size + start).trits();
            subseed[..4].offset();
            keys[i] = key::<BCTrit, CpuCurl<BCTrit>>(&subseed)
            let addresses: Vec<BCTrit> =
                address::<BCTrit, CpuCurl<BCTrit>>(&digest_key::<BCTrit, CpuCurl<BCTrit>>(&keys[i]));
            merkle_curl.absorb(&addresses);
        }
        let mut key: Trinary = TrinaryDemultiplexer::new(&keys[index/usize_size])[index % usize_size];
        Merkle { root, key, hashes }
    }
    fn root(address: Trinary, hashes: Vec<Trinary>, index: usize) -> Trinary {
        let curl = Curl::<Trit>::default();
        let mut i = 1;
        let mut output = address;
        for hash in hashes {
            match i & index {
                0 => {
                    curl.absorb(output);
                    curl.absorb(hash);
                }
                _ => {
                    curl.absorb(hash);
                    curl.absorb(output);
                }
            }
            i <<= 1;
            output = curl.squeeze(HASH_LENGTH);
            curl.reset();
        }
        output
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
