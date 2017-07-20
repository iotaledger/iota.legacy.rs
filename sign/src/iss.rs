// This is a straight clone off https://github.com/Come-from-Beyond/ISS/blob/master/src/cfb/iss/ISS.java for testing purposes.
// XXX DO NOT EXPECT THE METHODS OR CODE IN THIS MODULE TO PERSIST XXX
use alloc::Vec;

use trytes::*;
use tmath::*;
use trytes::constants::RADIX;
use curl::*;

const TRYTE_WIDTH: usize = 3;
const MAX_TRYTE_VALUE: i8 = 13;
const MIN_TRYTE_VALUE: i8 = -13;
pub const KEY_LENGTH: usize = ((HASH_LENGTH / 3) / RADIX as usize) * HASH_LENGTH;
const DIGEST_LENGTH: usize = HASH_LENGTH;
const ADDRESS_LENGTH: usize = HASH_LENGTH;
const SIGNATURE_LENGTH: usize = KEY_LENGTH;

pub fn subseed<C>(seed: &[Trit], index: usize) -> Vec<Trit>
where
    C: Curl<Trit>,
{
    let mut trits = seed.to_vec();
    let mut curl = C::default();

    for _ in 0..index {
        trits.as_mut_slice().incr();
    }

    curl.absorb(&trits);
    curl.squeeze(trits.len())
}

pub fn key<T, C>(subseed: &[T], security: u8) -> Vec<T>
where
    T: Copy,
    C: Curl<T>,
{
    let mut c = C::default();
    c.absorb(subseed);
    let length = security as usize * KEY_LENGTH;
    let mut key = c.squeeze(length);

    for div_offset in 0..(length / HASH_LENGTH) {
        let offset = div_offset * HASH_LENGTH;
        c.reset();
        c.absorb(&key[offset..offset + HASH_LENGTH]);

        key[offset..offset + HASH_LENGTH].clone_from_slice(c.rate());
    }
    key
}

pub fn digest_key<T, C>(key: &[T]) -> Vec<T>
where
    T: Copy + Clone + Sized,
    C: Curl<T>,
{
    assert_eq!(0, key.len() % KEY_LENGTH);
    let mut digest_curl = C::default();
    let mut key_fragment_curl = C::default();
    let mut buffer: [T; HASH_LENGTH] = [key[0]; HASH_LENGTH];

    for i in 0..(key.len() / HASH_LENGTH) {
        buffer.clone_from_slice(&key[i * HASH_LENGTH..(i + 1) * HASH_LENGTH]);

        for _ in 0..(MAX_TRYTE_VALUE - MIN_TRYTE_VALUE) {
            key_fragment_curl.reset();
            key_fragment_curl.absorb(&buffer);
            buffer.clone_from_slice(key_fragment_curl.rate());
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
    assert_eq!(HASH_LENGTH, bundle.len());

    let length = KEY_LENGTH * checksum_security(bundle);
    assert_eq!(length, key.len());

    let mut c = C::default();
    let mut signature = key.to_vec();

    for i in 0..(length / HASH_LENGTH) {
        for _ in 0..
            MAX_TRYTE_VALUE -
                (bundle[i * TRYTE_WIDTH] + bundle[i * TRYTE_WIDTH + 1] * 3 +
                     bundle[i * TRYTE_WIDTH + 2] * 9)
        {
            c.reset();
            c.absorb(&signature[i * HASH_LENGTH..(i + 1) * HASH_LENGTH]);
            &signature[i * HASH_LENGTH..(i + 1) * HASH_LENGTH].clone_from_slice(c.rate());
        }
    }

    signature
}

pub fn digest_bundle_signature<C>(bundle: &[Trit], signature: &[Trit]) -> Vec<Trit>
where
    C: Curl<Trit>,
{
    assert_eq!(DIGEST_LENGTH, bundle.len());

    let length = SIGNATURE_LENGTH * checksum_security(bundle);
    assert_eq!(length, signature.len());

    let mut digest_curl = C::default();
    let mut signature_fragment_curl = C::default();

    let mut buffer: [Trit; HASH_LENGTH] = [0; HASH_LENGTH];

    for i in 0..(length / HASH_LENGTH) {
        buffer.clone_from_slice(&signature[i * HASH_LENGTH..(i + 1) * HASH_LENGTH]);
        for _ in 0..
            (bundle[i * TRYTE_WIDTH] + bundle[i * TRYTE_WIDTH + 1] * 3 +
                 bundle[i * TRYTE_WIDTH + 2] * 9) - MIN_TRYTE_VALUE
        {
            signature_fragment_curl.reset();
            signature_fragment_curl.absorb(&buffer);
            buffer.clone_from_slice(signature_fragment_curl.rate());
        }
        digest_curl.absorb(&buffer);
    }

    digest_curl.squeeze(DIGEST_LENGTH)
}

pub fn checksum_security(hash: &[Trit]) -> usize {
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
}

#[cfg(test)]
mod test {
    use super::*;
    use curl_cpu::*;

    #[test]
    fn test_nothing_crashes() {
        let seed: Vec<Trit> = "WJRVZJOSSMRCGCJYFN9SSETWFLRCPWSCOEPPT9KNHWUTTW9BTELBWDPMHDRN9NTFGWESKAKZCFHGBJJQZ"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();

        let security = 1;
        let subseed = subseed::<CpuCurl<Trit>>(seed.as_slice(), 0);
        let key = key::<Trit, CpuCurl<Trit>>(subseed.as_slice(), security);
        let key_digest = digest_key::<Trit, CpuCurl<Trit>>(key.as_slice());
        let address: Vec<Trit> = address::<Trit, CpuCurl<Trit>>(key_digest.as_slice());

        address.len();
    }
    #[test]
    fn test_signature_matches_address() {
        let seed: Vec<Trit> = "WJRVZJOSSMRCGCJYFN9SSETWFLRCPWSCOEPPT9KNHWUTTW9BTELBWDPMHDRN9NTFGWESKAKZCFHGBJJQZ"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();
        let message_hash: Vec<Trit> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();
        let security = 1;
        let subseed = subseed::<CpuCurl<Trit>>(seed.as_slice(), 0);
        let key = key::<Trit, CpuCurl<Trit>>(subseed.as_slice(), security);
        let key_digest = digest_key::<Trit, CpuCurl<Trit>>(key.as_slice());
        let addr: Vec<Trit> = address::<Trit, CpuCurl<Trit>>(key_digest.as_slice());

        let sig: Vec<Trit> = signature::<CpuCurl<Trit>>(message_hash.as_slice(), &key);
        let digest: Vec<Trit> = digest_bundle_signature::<CpuCurl<Trit>>(message_hash.as_slice(), &sig);
        let out_address = address::<Trit, CpuCurl<Trit>>(digest.as_slice());
        assert_eq!(addr, out_address);
    }
}
