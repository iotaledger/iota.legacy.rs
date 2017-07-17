// This is a straight clone off https://github.com/Come-from-Beyond/ISS/blob/master/src/cfb/iss/ISS.java for testing purposes.
// XXX DO NOT EXPECT THE METHODS OR CODE IN THIS MODULE TO PERSIST XXX
use alloc::Vec;

use trytes::*;
use tmath::*;
use trytes::constants::RADIX;
use curl::*;

const TRYTE_WIDTH: usize = 3;
const MAX_TRYTE_VALUE: i8 = 1;
const MIN_TRYTE_VALUE: i8 = -1;
pub const KEY_LENGTH: usize = ((HASH_LENGTH / 3) / RADIX as usize) * HASH_LENGTH;
const DIGEST_LENGTH: usize = HASH_LENGTH;
const ADDRESS_LENGTH: usize = HASH_LENGTH;
const SIGNATURE_LENGTH: usize = KEY_LENGTH;

pub fn subseed<C>(seed: &[Trit], index: usize) -> Vec<Trit>
where
    C: Curl<Trit>,
{
    let mut trits: Vec<Trit> = seed.clone().to_vec();
    let mut curl = C::default();

    for _ in 0..index {
        trits.as_mut_slice().incr();
    }

    curl.absorb(trits.as_slice());
    curl.squeeze(trits.len()).into_iter().collect()
}

pub fn key<T, C>(subseed: &[T]) -> Vec<T>
where
    T: Copy,
    C: Curl<T>,
{
    let mut c = C::default();
    c.absorb(subseed);
    let mut key = c.squeeze(KEY_LENGTH);

    for div_offset in 0..(KEY_LENGTH / HASH_LENGTH) {
        let offset = div_offset * HASH_LENGTH;
        c.reset();
        c.absorb(&key[offset..offset + HASH_LENGTH]);

        let squeezed = c.squeeze(HASH_LENGTH);
        key[offset..offset + squeezed.len()].clone_from_slice(squeezed.as_slice());
    }
    key
}

pub fn digest_key<T, C>(key: &[T]) -> Vec<T>
where
    T: Copy + Clone + Sized,
    C: Curl<T>,
{
    let mut digest_curl = C::default();
    let mut key_fragment_curl = C::default();

    for i in 0..(KEY_LENGTH / HASH_LENGTH) {
        //let mut buffer: &[T] = (&key[i * HASH_LENGTH..(i + 1) * HASH_LENGTH]).clone();
        let mut buffer: Vec<T> = Vec::with_capacity(HASH_LENGTH);
        buffer.extend((&key[i * HASH_LENGTH..(i + 1) * HASH_LENGTH]).clone());

        for _ in 0..(MAX_TRYTE_VALUE - MIN_TRYTE_VALUE) {
            key_fragment_curl.reset();
            key_fragment_curl.absorb(&buffer);
            buffer.clone_from_slice(&key_fragment_curl.squeeze(HASH_LENGTH));
        }

        digest_curl.absorb(&buffer);
    }

    digest_curl.squeeze(DIGEST_LENGTH)
}

pub fn address<T, C>(digests: &[T]) -> Vec<T>
where
    T: Copy,
    C: Curl<T>,
{
    let mut c = C::default();
    c.absorb(digests);
    c.squeeze(ADDRESS_LENGTH)
}

pub fn signature<C>(bundle: &[Trit], key: &[Trit]) -> Vec<Trit>
where
    C: Curl<Trit>,
{

    let mut c = C::default();

    let mut signature: Vec<Trit> = Vec::with_capacity(key.len());
    signature.extend_from_slice(key);

    for i in 0..(SIGNATURE_LENGTH / HASH_LENGTH) {
        let hashing_chain_length = MAX_TRYTE_VALUE -
            (bundle[i * TRYTE_WIDTH] + bundle[i * TRYTE_WIDTH + 1] * 3 +
                 bundle[i * TRYTE_WIDTH + 2] * 9);
        for _ in hashing_chain_length..0 {
            c.reset();
            c.absorb(&signature[i * HASH_LENGTH..(i + 1) * HASH_LENGTH]);
            signature[i * HASH_LENGTH..(i + 1) * HASH_LENGTH]
                .clone_from_slice(c.squeeze(HASH_LENGTH).as_slice());
        }
    }

    signature
}

pub fn checksum_security(hash: &[Trit]) -> usize {
    /*
    match hash[..(HASH_LENGTH / 3)].iter().fold(0, |acc, &i| acc + i) {
        0 => 1,
        _ => {
            match hash[..(2 * HASH_LENGTH / 3)].iter().fold(
                0,
                |acc, &i| acc + i,
            ) {
                0 => 2,
                _ => {
                    match hash.iter().fold(0, |acc, i| acc + i) {
                        0 => 3,
                        _ => 0,
                    }
                }
            }
        }
    }
    */
    let mut sum = 0;
    for i in hash[..(HASH_LENGTH / 3)].iter() {
        sum += *i;
    }
    if sum == 0 {
        1
    } else {
        sum = 0;
        for i in hash[..(2 * HASH_LENGTH / 3)].iter() {
            sum += *i;
        }
        if sum == 0 {
            2
        } else {
            sum = 0;
            for i in hash {
                sum += *i;
            }
            if sum == 0 { 3 } else { 0 }
        }
    }
}

pub fn digest_bundle_signature<C>(bundle: &[Trit], signature: &[Trit]) -> Vec<Trit>
where
    C: Curl<Trit>,
{
    let mut digest_curl = C::default();
    let mut signature_fragment_curl = C::default();

    let signature_trits = signature;

    for i in 0..(SIGNATURE_LENGTH / HASH_LENGTH) {
        let hashing_chain_length = MAX_TRYTE_VALUE -
            (bundle[i * TRYTE_WIDTH] + bundle[i * TRYTE_WIDTH + 1] * 3 +
                 bundle[i * TRYTE_WIDTH + 2] * 9);

        let mut buffer: Vec<Trit> = signature_trits[i * HASH_LENGTH..(i + 1) * HASH_LENGTH]
            .iter()
            .cloned()
            .collect();
        for _ in hashing_chain_length..0 {
            signature_fragment_curl.reset();
            signature_fragment_curl.absorb(
                &signature_trits[i * HASH_LENGTH..
                                     (i + 1) *
                                         HASH_LENGTH],
            );
            buffer.clone_from_slice(signature_fragment_curl.squeeze(HASH_LENGTH).as_slice());
        }

        digest_curl.absorb(&buffer);
    }

    digest_curl.squeeze(DIGEST_LENGTH)
}


#[cfg(test)]
mod test {
    use super::*;
    use curl_cpu::*;

    #[test]
    fn test_nothing_crashes() {
        let seed: Trinary = "WJRVZJOSSMRCGCJYFN9SSETWFLRCPWSCOEPPT9KNHWUTTW9BTELBWDPMHDRN9NTFGWESKAKZCFHGBJJQZ"
            .chars()
            .collect();
        let subseed = subseed::<CpuCurl<Trit>>(&seed.trits(), 0);
        let key = key::<Trit, CpuCurl<Trit>>(&subseed);
        let key_digest = digest_key::<Trit, CpuCurl<Trit>>(&key);
        let address: Vec<Trit> = address::<Trit, CpuCurl<Trit>>(&key_digest);
        let addr_trinary: Trinary = address.into_iter().collect();

        addr_trinary.len_trits();
    }
}
