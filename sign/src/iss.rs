// This is a straight clone off https://github.com/Come-from-Beyond/ISS/blob/master/src/cfb/iss/ISS.java for testing purposes.
// XXX DO NOT EXPECT THE METHODS OR CODE IN THIS MODULE TO PERSIST XXX

use trytes::*;
use tmath::*;
use trytes::constants::RADIX;
use curl::*;
use core::cmp::min;

const TRYTE_WIDTH: usize = 3;
const MAX_TRYTE_VALUE: i8 = 13;
const MIN_TRYTE_VALUE: i8 = -13;
pub const KEY_LENGTH: usize = ((HASH_LENGTH / 3) / RADIX as usize) * HASH_LENGTH;
pub const DIGEST_LENGTH: usize = HASH_LENGTH;
pub const ADDRESS_LENGTH: usize = HASH_LENGTH;
pub const SIGNATURE_LENGTH: usize = KEY_LENGTH;

pub fn subseed<C>(seed: &[Trit], index: isize, out: &mut [Trit], curl: &mut C)
where
    C: Curl<Trit>,
{
    assert!(out.len() >= HASH_LENGTH);
    let out_copy_len = min(out.len(), seed.len());
    out[..out_copy_len].clone_from_slice(seed);

    add_assign(out, index);
    //add_trits(seed, out);

    curl.absorb(&out[0..seed.len()]);
    curl.squeeze(&mut out[0..HASH_LENGTH]);
    curl.reset();
}

/// Take first 243 trits of `key_space` as subseed, and write key out to `key_space`
pub fn key<T, C>(key_space: &mut [T], security: usize, curl: &mut C)
where
    T: Copy,
    C: Curl<T>,
{
    let length = security * KEY_LENGTH;

    assert!(
        length % KEY_LENGTH == 0,
        "Security space size must be a multiple of KEY_LENGTH"
    );
    assert!(
        length == key_space.len(),
        "Key space size must be equal to security space size"
    );
    curl.absorb(&key_space[0..HASH_LENGTH]);
    curl.squeeze(&mut key_space[0..length]);

    for div_offset in 0..(length / HASH_LENGTH) {
        let offset = div_offset * HASH_LENGTH;
        curl.reset();
        curl.absorb(&key_space[offset..offset + HASH_LENGTH]);

        key_space[offset..offset + HASH_LENGTH].clone_from_slice(curl.rate());
    }
    curl.reset();
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

    key_fragment_curl.reset();
    digest_curl.reset();
}

/// since `digests` is normally ephemeral, address is written out to `digests`
pub fn address<T, C>(digests: &mut [T], curl: &mut C)
where
    T: Copy,
    C: Curl<T>,
{

    curl.absorb(digests);
    curl.squeeze(&mut digests[..ADDRESS_LENGTH]);
    curl.reset();
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

/// Takes a `bundle` and input key `key_signature`, and writes the signature out to `key_signature`
pub fn signature<C>(bundle: &[Trit], key_signature: &mut [Trit], curl: &mut C)
where
    C: Curl<Trit>,
{
    assert_eq!(HASH_LENGTH, bundle.len());

    let length = KEY_LENGTH * checksum_security(bundle);
    assert_eq!(length, key_signature.len());

    for i in 0..(length / HASH_LENGTH) {
        for _ in 0..
            MAX_TRYTE_VALUE -
                (bundle[i * TRYTE_WIDTH] + bundle[i * TRYTE_WIDTH + 1] * 3 +
                     bundle[i * TRYTE_WIDTH + 2] * 9)
        {
            curl.reset();
            curl.absorb(&key_signature[i * HASH_LENGTH..(i + 1) * HASH_LENGTH]);
            key_signature[i * HASH_LENGTH..(i + 1) * HASH_LENGTH].clone_from_slice(curl.rate());
        }
    }

    curl.reset();
}

/// Given a `subseed` and a `security` level, generate the key digest, and write it to `out`
pub fn subseed_to_digest<T, C>(
    subseed: &[T],
    security: usize,
    out: &mut [T],
    c1: &mut C,
    c2: &mut C,
    c3: &mut C,
) where
    T: Copy + Clone + Sized,
    C: Curl<T>,
{
    let length = security * KEY_LENGTH / HASH_LENGTH;
    c1.absorb(subseed);
    for _ in 0..(length) {
        c1.squeeze(out);
        for _ in 0..(MAX_TRYTE_VALUE - MIN_TRYTE_VALUE + 1) {
            c2.reset();
            c2.absorb(out);
            out.clone_from_slice(c2.rate());
        }

        c3.absorb(out);

    }
    c3.squeeze(out);
}

/// Given a `hash` to sign, a `subkey` (or subseed), and a `security` (size of signature in units
/// of `SIGNATURE_LENGTH`), write output to `signature`
pub fn subseed_to_signature<C>(
    hash: &[Trit],
    subkey: &[Trit],
    signature: &mut [Trit],
    security: usize,
    curl1: &mut C,
    curl2: &mut C,
) where
    C: Curl<Trit>,
{
    let length = security as usize * KEY_LENGTH;

    curl1.reset();
    curl1.absorb(&subkey[..HASH_LENGTH]);

    for i in 0..(length / HASH_LENGTH) {
        let offset = i * HASH_LENGTH;
        curl2.reset();
        curl1.squeeze(&mut signature[offset..offset + HASH_LENGTH]);
        curl2.absorb(&signature[offset..offset + HASH_LENGTH]);

        signature[offset..offset + HASH_LENGTH].clone_from_slice(curl2.rate());
        for _ in 0..
            MAX_TRYTE_VALUE -
                (hash[i * TRYTE_WIDTH] + hash[i * TRYTE_WIDTH + 1] * 3 +
                     hash[i * TRYTE_WIDTH + 2] * 9)
        {
            curl2.reset();
            curl2.absorb(&signature[offset..offset + HASH_LENGTH]);
            signature[offset..offset + HASH_LENGTH].clone_from_slice(curl2.rate());
        }
    }

    curl1.reset();
    curl2.reset();
}

/// Takes an input `signature`, and writes its digest out to the first 243 trits
pub fn digest_bundle_signature<C>(bundle: &[Trit], signature: &mut [Trit], curl: &mut C)
where
    C: Curl<Trit>,
{
    assert_eq!(DIGEST_LENGTH, bundle.len());

    let length = SIGNATURE_LENGTH * checksum_security(bundle);
    assert_eq!(length, signature.len());

    let mut buffer: [Trit; HASH_LENGTH] = [0; HASH_LENGTH];
    let mut pos = 0;
    for i in 0..(length / HASH_LENGTH) {
        buffer.clone_from_slice(&signature[i * HASH_LENGTH..(i + 1) * HASH_LENGTH]);
        for _ in 0..
            (bundle[i * TRYTE_WIDTH] + bundle[i * TRYTE_WIDTH + 1] * 3 +
                 bundle[i * TRYTE_WIDTH + 2] * 9) - MIN_TRYTE_VALUE
        {
            curl.reset();
            curl.absorb(&signature[pos..pos + HASH_LENGTH]);
            signature[pos..pos + HASH_LENGTH].clone_from_slice(curl.rate());
        }
        pos += HASH_LENGTH;
    }

    curl.reset();
    curl.absorb(&signature[..length]);
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
        let mut key_space = vec![0; KEY_LENGTH];
        let mut key_digest_space = vec![0; DIGEST_LENGTH];

        let mut c1 = CpuCurl::<Trit>::default();
        let mut c2 = CpuCurl::<Trit>::default();
        subseed(&seed, 0, &mut key_space, &mut c1);
        c1.reset();
        key(&mut key_space, security, &mut c1);
        c1.reset();
        digest_key(&mut key_space, &mut key_digest_space, &mut c1, &mut c2);
        c1.reset();
        address(&mut key_digest_space, &mut c1);

        let mut c3 = CpuCurl::<BCTrit>::default();
        let mut bsubseed: Vec<BCTrit> = key_space.into_iter().map(trit_to_bct).collect();
        key::<BCTrit, CpuCurl<BCTrit>>(&mut bsubseed, security, &mut c3);
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
        let mut key_space = vec![0; KEY_LENGTH];
        let mut digest_space = vec![0; DIGEST_LENGTH];
        let mut address_space = vec![0; ADDRESS_LENGTH];
        let mut sig_address_space = vec![0; ADDRESS_LENGTH];
        let mut signature_space = vec![0; SIGNATURE_LENGTH];
        let index = 234987621;

        subseed::<CpuCurl<Trit>>(&seed, index, &mut key_space, &mut c1);
        c1.reset();
        key(&mut key_space, security, &mut c1);
        c1.reset();
        digest_key::<Trit, CpuCurl<Trit>>(&key_space, &mut digest_space, &mut c1, &mut c2);
        c1.reset();
        address::<Trit, CpuCurl<Trit>>(&mut digest_space, &mut c1);
        address_space.clone_from_slice(&digest_space);

        c1.reset();
        signature::<CpuCurl<Trit>>(message_hash.as_slice(), &mut key_space, &mut c1);
        signature_space.clone_from_slice(&key_space);

        c1.reset();
        c2.reset();
        digest_bundle_signature::<CpuCurl<Trit>>(
            message_hash.as_slice(),
            &mut signature_space,
            &mut c1,
        );
        c1.squeeze(&mut digest_space);
        c1.reset();
        address::<Trit, CpuCurl<Trit>>(&mut digest_space, &mut c1);
        sig_address_space.clone_from_slice(&digest_space);

        assert_eq!(address_space, sig_address_space);
    }

    #[test]
    fn test_subseed_to_signature_matches_address() {
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
        let mut c3 = CpuCurl::<Trit>::default();
        let mut key_space = vec![0; KEY_LENGTH];
        let mut digest_space = vec![0; DIGEST_LENGTH];
        let mut address_space = vec![0; ADDRESS_LENGTH];
        let mut sig_address_space = vec![0; ADDRESS_LENGTH];
        let mut signature_space = vec![0; SIGNATURE_LENGTH];
        let mut direct_address = vec![0; ADDRESS_LENGTH];
        let index = 234987621;

        subseed::<CpuCurl<Trit>>(&seed, index, &mut key_space, &mut c1);
        c1.reset();
        key(&mut key_space, security, &mut c1);
        c1.reset();
        digest_key::<Trit, CpuCurl<Trit>>(&key_space, &mut digest_space, &mut c1, &mut c2);
        c1.reset();
        c2.reset();
        address::<Trit, CpuCurl<Trit>>(&mut digest_space, &mut c1);
        c1.reset();
        address_space.clone_from_slice(&digest_space);

        subseed(&seed, index, &mut digest_space, &mut c1);
        c1.reset();
        subseed_to_digest(
            &digest_space,
            security as usize,
            &mut direct_address,
            &mut c1,
            &mut c2,
            &mut c3,
        );
        c1.reset();
        c2.reset();
        address(&mut direct_address, &mut c1);
        c1.reset();
        subseed_to_signature(
            message_hash.as_slice(),
            digest_space.as_mut_slice(),
            &mut signature_space,
            security as usize,
            &mut c1,
            &mut c2,
        );

        c1.reset();
        c2.reset();
        digest_bundle_signature::<CpuCurl<Trit>>(
            message_hash.as_slice(),
            &mut signature_space,
            &mut c1,
        );
        c1.squeeze(&mut digest_space);
        c1.reset();
        address::<Trit, CpuCurl<Trit>>(&mut digest_space, &mut c1);
        sig_address_space.clone_from_slice(&digest_space);

        assert_eq!(address_space, sig_address_space);
        assert_eq!(address_space, direct_address);
    }
}
