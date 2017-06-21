// This is a straight clone off https://github.com/Come-from-Beyond/ISS/blob/master/src/cfb/iss/ISS.java for testing purposes.
// XXX DO NOT EXPECT THE METHODS OR CODE IN THIS MODULE TO PERSIST XXX
use trytes::*;
use trytes::constants::RADIX;
use curl::*;

const TRYTE_WIDTH: usize = 3;
const MAX_TRYTE_VALUE: i8 = 1;
const MIN_TRYTE_VALUE: i8 = -1;
const KEY_LENGTH: usize = ((HASH_LENGTH / 3) / RADIX as usize) * HASH_LENGTH;
const DIGEST_LENGTH: usize = HASH_LENGTH;
const ADDRESS_LENGTH: usize = HASH_LENGTH;
const BUNDLE_LENGTH: usize = HASH_LENGTH;
const SIGNATURE_LENGTH: usize = KEY_LENGTH;

pub fn subseed<C>(seed: Trinary, mut index: usize) -> Trinary
where
    C: Curl + Default,
{
    let mut trits = seed.trits();
    let mut curl = C::default();

    while index > 0 {
        for i in 0..trits.len() {
            trits[i] += 1;

            if trits[i] > 1 {
                trits[i] = -1;
            } else {
                break;
            }
        }

        index -= 1;
    }

    curl.absorb(&trits);
    curl.squeeze(trits.len()).into_iter().collect()
}

pub fn key<C>(subseed: Trinary) -> Trinary
where
    C: Curl + Default,
{
    let mut c = C::default();
    let trits = subseed.trits();
    c.absorb(&trits);
    let mut key = c.squeeze(KEY_LENGTH);

    for divOffset in 0..(KEY_LENGTH/HASH_LENGTH){
        let offset = divOffset * HASH_LENGTH;
        c.reset();
        c.absorb(&key[offset..offset+HASH_LENGTH]);

        let squeezed = c.squeeze(HASH_LENGTH);
        key[offset..offset+squeezed.len()].clone_from_slice(squeezed.as_slice());
    }

    key.into_iter().collect()
}

pub fn digest_key<C>(key: Trinary) -> Trinary
where
    C: Curl + Default,
{
    let mut digestCurl = C::default();
    let mut keyFragmentCurl = C::default();
    let trits = key.trits();

    for i in 0..(KEY_LENGTH / HASH_LENGTH) {
        let mut buffer: Vec<Trit> = trits[i * HASH_LENGTH..(i + 1) * HASH_LENGTH]
            .iter()
            .cloned()
            .collect();

        for j in 0..(MAX_TRYTE_VALUE - MIN_TRYTE_VALUE) {
            keyFragmentCurl.reset();
            keyFragmentCurl.absorb(&buffer);
            buffer.clone_from_slice(&keyFragmentCurl.squeeze(HASH_LENGTH));
        }

        digestCurl.absorb(&buffer);
    }

    digestCurl.squeeze(DIGEST_LENGTH).into_iter().collect()
}

pub fn address<C>(digests: Trinary) -> Trinary
where
    C: Curl + Default,
{
    let mut c = C::default();
    let trits = digests.trits();
    c.absorb(&trits);
    c.squeeze(ADDRESS_LENGTH).into_iter().collect()
}

pub fn signature<C>(bundle: Trinary, key: Trinary) -> Trinary
where
    C: Curl + Default,
{

    let mut c = C::default();

    let mut signature = key.trits();
    let bundleTrits = bundle.trits();

    for i in 0..(SIGNATURE_LENGTH / HASH_LENGTH) {
        let hashingChainLength = MAX_TRYTE_VALUE -
            (bundleTrits[i * TRYTE_WIDTH] + bundleTrits[i * TRYTE_WIDTH + 1] * 3 +
                 bundleTrits[i * TRYTE_WIDTH + 2] * 9);
        for j in hashingChainLength..0 {
            c.reset();
            c.absorb(&signature[i * HASH_LENGTH..(i + 1) * HASH_LENGTH]);
            signature[i * HASH_LENGTH..(i + 1) * HASH_LENGTH]
                .clone_from_slice(c.squeeze(HASH_LENGTH).as_slice());
        }
    }

    signature.into_iter().collect()
}

pub fn digest_bundle_signature<C>(bundle: Trinary, signature: Trinary) -> Trinary
where
    C: Curl + Default,
{
    let mut digestCurl = C::default();
    let mut signatureFragmentCurl = C::default();

    let signatureTrits = signature.trits();
    let bundleTrits = bundle.trits();

    for i in 0..(SIGNATURE_LENGTH / HASH_LENGTH) {
        let hashingChainLength = MAX_TRYTE_VALUE -
            (bundleTrits[i * TRYTE_WIDTH] + bundleTrits[i * TRYTE_WIDTH + 1] * 3 +
                 bundleTrits[i * TRYTE_WIDTH + 2] * 9);

        let mut buffer: Vec<Trit> = signatureTrits[i * HASH_LENGTH..(i + 1) * HASH_LENGTH]
            .iter()
            .cloned()
            .collect();
        for j in hashingChainLength..0 {
            signatureFragmentCurl.reset();
            signatureFragmentCurl.absorb(&signatureTrits[i * HASH_LENGTH..(i + 1) * HASH_LENGTH]);
            buffer.clone_from_slice(signatureFragmentCurl.squeeze(HASH_LENGTH).as_slice());
        }

        digestCurl.absorb(&buffer);
    }

    digestCurl.squeeze(DIGEST_LENGTH).into_iter().collect()
}


#[cfg(test)]
mod test {
    use super::*;
    use curl::simple::*;

    #[test]
    fn test_nothing_crashes() {
        let seed: Trinary = "WJRVZJOSSMRCGCJYFN9SSETWFLRCPWSCOEPPT9KNHWUTTW9BTELBWDPMHDRN9NTFGWESKAKZCFHGBJJQZ"
            .chars()
            .collect();
        let subseed = subseed::<SimpleCurl>(seed, 0);
        let key = key::<SimpleCurl>(subseed);
        let key_digest = digest_key::<SimpleCurl>(key);
        let address = address::<SimpleCurl>(key_digest);
    }
}
