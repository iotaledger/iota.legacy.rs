// This is a straight clone off https://github.com/Come-from-Beyond/ISS/blob/master/src/cfb/iss/ISS.java for testing purposes.
// XXX DO NOT EXPECT THE METHODS OR CODE IN THIS MODULE TO PERSIST XXX

use trytes::*;
use tmath::*;
use trytes::constants::RADIX;
use curl::*;

const TRYTE_WIDTH: usize = 3;
const MAX_TRYTE_VALUE: i8 = 13;
const MIN_TRYTE_VALUE: i8 = -13;
pub const KEY_LENGTH: usize = ((HASH_LENGTH / 3) / RADIX as usize) * HASH_LENGTH;
pub const DIGEST_LENGTH: usize = HASH_LENGTH;
pub const ADDRESS_LENGTH: usize = HASH_LENGTH;
pub const SIGNATURE_LENGTH: usize = KEY_LENGTH;

pub fn subseed<C>(seed: &[Trit], index: usize, out: &mut [Trit], curl: &mut C)
where
    C: Curl<Trit>,
{
    assert_eq!(out.len(), HASH_LENGTH);

    let mut idx: [Trit; HASH_LENGTH] = [0; HASH_LENGTH];
    num::int2trits(index as isize, &mut idx);
    add_trits(seed, &idx, out);

    curl.absorb(out);
    curl.squeeze(out)
}

// Note that this will y
pub fn key<T, C>(subseed: &[T], security_space: &mut [T], key_space: &mut [T], curl: &mut C)
where
    T: Copy,
    C: Curl<T>,
{
    let length = security_space.len();

    assert!(
        length % KEY_LENGTH == 0,
        "Security space size must be a multiple of KEY_LENGTH"
    );
    assert!(
        length == key_space.len(),
        "Key space size must be equal to security space size"
    );

    curl.absorb(subseed);
    curl.squeeze(&mut key_space[0..length]);

    for div_offset in 0..(length / HASH_LENGTH) {
        let offset = div_offset * HASH_LENGTH;
        curl.reset();
        curl.absorb(&key_space[offset..offset + HASH_LENGTH]);

        key_space[offset..offset + HASH_LENGTH].clone_from_slice(curl.rate());
    }
}

pub fn digest_key<T, C>(
    key: &[T],
    digest_space: &mut [T],
    digest_curl: &mut C,
    key_fragment_curl: &mut C,
) where
    T: Copy + Clone + Sized,
    C: Curl<T>,
{
    assert_eq!(0, key.len() % KEY_LENGTH);
    assert!(
        digest_space.len() == DIGEST_LENGTH,
        "Digest space size must be qual to DIGEST_LENGTH"
    );

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

    digest_curl.squeeze(digest_space);
}

pub fn address<T, C>(digests: &[T], address_space: &mut [T], curl: &mut C)
where
    T: Copy,
    C: Curl<T>,
{
    assert!(
        address_space.len() == ADDRESS_LENGTH,
        "Address space size must be equal to ADDRESS_LENGTH"
    );

    curl.absorb(digests);
    curl.squeeze(address_space);
}

pub fn checksum_security(hash: &[Trit]) -> usize {
    match hash[..(HASH_LENGTH / 3)].iter().sum() {
        0 => 1,
        _ => {
            match hash[..(2 * HASH_LENGTH / 3)].iter().sum() {
                0 => 2,
                _ => {
                    match hash.iter().sum() {
                        0 => 3,
                        _ => 0,
                    }
                }
            }
        }
    }
}

pub fn signature<C>(bundle: &[Trit], key: &[Trit], signature_space: &mut [Trit], curl: &mut C)
where
    C: Curl<Trit>,
{
    assert_eq!(HASH_LENGTH, bundle.len());

    let length = KEY_LENGTH * checksum_security(bundle);
    assert_eq!(length, key.len());

    signature_space.clone_from_slice(&key);

    for i in 0..(length / HASH_LENGTH) {
        for _ in 0..
            MAX_TRYTE_VALUE -
                (bundle[i * TRYTE_WIDTH] + bundle[i * TRYTE_WIDTH + 1] * 3 +
                     bundle[i * TRYTE_WIDTH + 2] * 9)
        {
            curl.reset();
            curl.absorb(&signature_space[i * HASH_LENGTH..(i + 1) * HASH_LENGTH]);
            signature_space[i * HASH_LENGTH..(i + 1) * HASH_LENGTH].clone_from_slice(curl.rate());
        }
    }

}

pub fn digest_bundle_signature<C>(
    bundle: &[Trit],
    signature: &[Trit],
    digest_space: &mut [Trit],
    digest_curl: &mut C,
    signature_fragment_curl: &mut C,
) where
    C: Curl<Trit>,
{
    assert_eq!(DIGEST_LENGTH, bundle.len());
    assert_eq!(DIGEST_LENGTH, digest_space.len());

    let length = SIGNATURE_LENGTH * checksum_security(bundle);
    assert_eq!(length, signature.len());

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

    digest_curl.squeeze(digest_space)
}


#[cfg(test)]
mod test {
    use super::*;
    use curl_cpu::*;
    use alloc::Vec;

    #[test]
    fn test_nothing_crashes() {
        let seed: Vec<Trit> = "WJRVZJOSSMRCGCJYFN9SSETWFLRCPWSCOEPPT9KNHWUTTW9BTELBWDPMHDRN9NTFGWESKAKZCFHGBJJQZ"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();

        let security = 1;
        let mut subseed_space = vec![0; seed.len()];
        let mut security_space = vec![0; security * KEY_LENGTH];
        let mut key_space = vec![0; KEY_LENGTH];
        let mut key_digest_space = vec![0; DIGEST_LENGTH];
        let mut address_space = vec![0; ADDRESS_LENGTH];

        let mut c1 = CpuCurl::<Trit>::default();
        let mut c2 = CpuCurl::<Trit>::default();
        subseed::<CpuCurl<Trit>>(&seed, 0, &mut subseed_space, &mut c1);
        c1.reset();
        key::<Trit, CpuCurl<Trit>>(&subseed_space, &mut security_space, &mut key_space, &mut c1);
        c1.reset();
        digest_key::<Trit, CpuCurl<Trit>>(&key_space, &mut key_digest_space, &mut c1, &mut c2);
        c1.reset();
        address::<Trit, CpuCurl<Trit>>(&key_digest_space, &mut address_space, &mut c1);

        address_space.len();
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

        let mut c1 = CpuCurl::<Trit>::default();
        let mut c2 = CpuCurl::<Trit>::default();
        let mut subseed_space = vec![0; seed.len()];
        let mut security_space = vec![0; security * KEY_LENGTH];
        let mut key_space = vec![0; KEY_LENGTH];
        let mut digest_space = vec![0; DIGEST_LENGTH];
        let mut address_space = vec![0; ADDRESS_LENGTH];
        let mut sig_address_space = vec![0; ADDRESS_LENGTH];
        let mut signature_space = vec![0; SIGNATURE_LENGTH];
        let index = 23498762134896712438679;

        subseed::<CpuCurl<Trit>>(&seed, index, &mut subseed_space, &mut c1);
        c1.reset();
        key::<Trit, CpuCurl<Trit>>(&subseed_space, &mut security_space, &mut key_space, &mut c1);
        c1.reset();
        digest_key::<Trit, CpuCurl<Trit>>(&key_space, &mut digest_space, &mut c1, &mut c2);
        c1.reset();
        address::<Trit, CpuCurl<Trit>>(&digest_space, &mut address_space, &mut c1);

        c1.reset();
        signature::<CpuCurl<Trit>>(
            message_hash.as_slice(),
            &key_space,
            &mut signature_space,
            &mut c1,
        );

        c1.reset();
        c2.reset();
        digest_bundle_signature::<CpuCurl<Trit>>(
            message_hash.as_slice(),
            &signature_space,
            &mut digest_space,
            &mut c1,
            &mut c2,
        );
        c1.reset();
        address::<Trit, CpuCurl<Trit>>(&digest_space, &mut sig_address_space, &mut c1);

        assert_eq!(address_space, sig_address_space);
    }
}
