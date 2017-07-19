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

pub fn subseed<C>(seed: &IntoTrits<Trit>, index: usize) -> Vec<Trit>
where
    C: Curl<Trit>,
{
    let mut trits = seed.trits();
    let mut curl = C::default();

    for _ in 0..index {
        trits.as_mut_slice().incr();
    }

    curl.absorb(&trits);
    curl.squeeze(trits.len())
}

pub fn key<T, C>(subseed: &IntoTrits<T>, security: u8) -> Vec<T>
where
    T: Copy,
    C: Curl<T>,
{
    let mut c = C::default();
    let trits = subseed.trits();
    c.absorb(&trits);
    let length = security as usize * KEY_LENGTH;
    let mut key = c.squeeze(length);

    for div_offset in 0..(length / HASH_LENGTH) {
        let offset = div_offset * HASH_LENGTH;
        c.reset();
        c.absorb(&key[offset..offset + HASH_LENGTH]);

        key[offset..offset + HASH_LENGTH].clone_from_slice(c.squeeze(HASH_LENGTH).as_slice());
    }
    key
}

pub fn digest_key<T, C>(key: &IntoTrits<T>) -> Vec<T>
where
    T: Copy + Clone + Sized,
    C: Curl<T>,
{
    assert_eq!(0, key.len_trits() % KEY_LENGTH);
    let mut digest_curl = C::default();
    let mut key_fragment_curl = C::default();
    let trits: Vec<T> = key.trits();

    for i in 0..(key.len_trits() / HASH_LENGTH) {
        let mut buffer: Vec<T> = trits[i * HASH_LENGTH..(i + 1) * HASH_LENGTH]
            .iter()
            .cloned()
            .collect();

        for _ in 0..(MAX_TRYTE_VALUE - MIN_TRYTE_VALUE) {
            key_fragment_curl.reset();
            key_fragment_curl.absorb(buffer.as_slice());
            buffer.clone_from_slice(key_fragment_curl.squeeze(HASH_LENGTH).as_slice());
        }

        digest_curl.absorb(&buffer);
    }

    digest_curl.squeeze(DIGEST_LENGTH)
}

pub fn address<T, C>(digests: &IntoTrits<T>) -> Vec<T>
where
    T: Copy,
    C: Curl<T>,
{
    let mut c = C::default();
    c.absorb(digests.trits().as_slice());
    c.squeeze(ADDRESS_LENGTH)
}

pub fn signature<C>(bundle: &IntoTrits<Trit>, key: &IntoTrits<Trit>) -> Vec<Trit>
where
    C: Curl<Trit>,
{
    let bundle_trits: Vec<Trit> = bundle.trits();
    assert_eq!(HASH_LENGTH, bundle_trits.len());

    let mut signature = key.trits();
    let length = KEY_LENGTH * checksum_security(bundle);
    assert_eq!(length, signature.len());

    let mut c = C::default();

    for i in 0..(length / HASH_LENGTH) {
        for _ in 0..
            MAX_TRYTE_VALUE -
                (bundle_trits[i * TRYTE_WIDTH] + bundle_trits[i * TRYTE_WIDTH + 1] * 3 +
                     bundle_trits[i * TRYTE_WIDTH + 2] * 9)
        {
            c.reset();
            c.absorb(&signature[i * HASH_LENGTH..(i + 1) * HASH_LENGTH]);
            signature[i * HASH_LENGTH..(i + 1) * HASH_LENGTH]
                .clone_from_slice(c.squeeze(HASH_LENGTH).as_slice());
        }
    }

    signature
}

pub fn digest_bundle_signature<C>(
    bundle: &IntoTrits<Trit>,
    signature: &IntoTrits<Trit>,
) -> Vec<Trit>
where
    C: Curl<Trit>,
{
    assert_eq!(DIGEST_LENGTH, bundle.len_trits());
    let bundle_trits: Vec<Trit> = bundle.trits();
    let length = SIGNATURE_LENGTH * checksum_security(bundle);
    assert_eq!(length, signature.len_trits());

    let mut digest_curl = C::default();
    let mut signature_fragment_curl = C::default();

    for i in 0..(length / HASH_LENGTH) {
        let mut buffer: Vec<Trit> = signature.trits()[i * HASH_LENGTH..(i + 1) * HASH_LENGTH]
            .iter()
            .cloned()
            .collect();
        for _ in 0..
            (bundle_trits[i * TRYTE_WIDTH] + bundle_trits[i * TRYTE_WIDTH + 1] * 3 +
                 bundle_trits[i * TRYTE_WIDTH + 2] * 9) - MIN_TRYTE_VALUE
        {
            signature_fragment_curl.reset();
            signature_fragment_curl.absorb(&buffer);
            buffer.clone_from_slice(signature_fragment_curl.squeeze(HASH_LENGTH).as_slice());
        }
        digest_curl.absorb(&buffer);
    }

    digest_curl.squeeze(DIGEST_LENGTH)
}

pub fn checksum_security(hash: &IntoTrits<Trit>) -> usize {
    let trits = hash.trits();
    match trits[..(HASH_LENGTH / 3)].iter().fold(0, |acc, &i| acc + i) {
        0 => 1,
        _ => {
            match trits[..(2 * HASH_LENGTH / 3)].iter().fold(
                0,
                |acc, &i| acc + i,
            ) {
                0 => 2,
                _ => {
                    match trits.iter().fold(0, |acc, i| acc + i) {
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
            .trits();
        let security = 1;
        let subseed = subseed::<CpuCurl<Trit>>(&seed, 0);
        let key = key::<Trit, CpuCurl<Trit>>(&subseed, security);
        let key_digest = digest_key::<Trit, CpuCurl<Trit>>(&key);
        let address: Vec<Trit> = address::<Trit, CpuCurl<Trit>>(&key_digest);

        IntoTrits::<BCTrit>::len_trits(&address);
    }
    #[test]
    fn test_signature_matches_address() {
        let seed: Vec<Trit> = "WJRVZJOSSMRCGCJYFN9SSETWFLRCPWSCOEPPT9KNHWUTTW9BTELBWDPMHDRN9NTFGWESKAKZCFHGBJJQZ"
            .trits();
        let message_hash: Vec<Trit> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9"
            .trits();
        let security = 1;
        let subseed = subseed::<CpuCurl<Trit>>(&seed, 0);
        let key = key::<Trit, CpuCurl<Trit>>(&subseed, security);
        let key_digest = digest_key::<Trit, CpuCurl<Trit>>(&key);
        let addr: Vec<Trit> = address::<Trit, CpuCurl<Trit>>(&key_digest);

        let sig: Vec<Trit> = signature::<CpuCurl<Trit>>(&message_hash, &key);
        let digest: Vec<Trit> = digest_bundle_signature::<CpuCurl<Trit>>(&message_hash, &sig);
        let out_address = address::<Trit, CpuCurl<Trit>>(&digest);
        assert_eq!(addr, out_address);
    }
}
