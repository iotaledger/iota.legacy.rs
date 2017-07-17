use alloc::*;
use alloc::string::ToString;
use curl::*;
use sign::iss;
use merkle;
use trytes::*;

pub enum MamError {
    /// Message Hash did not have any hamming weight of zero
    InvalidHash,
    /// Signature did not match expected root
    InvalidSignature,
    /// Custom error
    CustomError(String),
}
pub fn sign<C, H>(
    message_in: &[Trit],
    next: &[Trit],
    key: &[Trit],
    hashes: &[Vec<Trit>],
    security: u8,
) -> Vec<Trit>
where
    C: Curl<Trit>,
    H: HammingNonce,
{
    let message_length: Vec<Trit> =
        pascal::encode((next.len() + message_in.len()) / TRITS_PER_TRYTE);
    let message: Vec<Trit> = {
        let mut m = Vec::with_capacity(next.len() + message_in.len());
        m.extend_from_slice(next);
        m.extend_from_slice(message_in);
        m
    };
    let message_nonce: Vec<Trit> = H::search(&message, TRITS_PER_TRYTE as u8, security).unwrap();
    let nonce_length: Vec<Trit> = pascal::encode(message_nonce.len() / TRITS_PER_TRYTE);
    let signature = {
        let mut curl = C::default();
        curl.absorb(&message_length);
        curl.absorb(&message);
        curl.absorb(&message_nonce);
        let hash = curl.squeeze(HASH_LENGTH);
        let security = iss::checksum_security(&hash);
        iss::signature::<C>(&hash, key)
    };
    let siblings: Vec<Trit> = hashes.iter().fold(
        Vec::with_capacity(hashes.len() * HASH_LENGTH),
        |mut acc, v| {
            acc.extend(v);
            acc
        },
    );
    //let siblings_length: Trinary = num::int2trits(hashes.len() as isize).into_iter().collect();
    let siblings_length: Vec<Trit> = pascal::encode(siblings.len() / TRITS_PER_TRYTE);
    let mut payload: Vec<Trit> = Vec::with_capacity(
        message_length.len() + message.len() + nonce_length.len() + message_nonce.len() +
            signature.len() + siblings_length.len() +
            siblings.len(),
    );
    payload.extend(message_length);
    payload.extend(message);
    payload.extend(nonce_length);
    payload.extend(message_nonce);
    payload.extend(signature);
    payload.extend(siblings_length);
    payload.extend(siblings);
    payload
}

pub fn authenticate<C>(
    payload: &[Trit],
    root: &[Trit],
    index: usize,
) -> Result<(Vec<Trit>, Vec<Trit>), MamError>
where
    C: Curl<Trit>,
{

    let mut length = 0;
    let mut payload_iter = payload.iter();
    let message_length_trits: Vec<Trit> = payload_iter
        .by_ref()
        .take({
            let (l, e) = pascal::decode(payload);
            length = l * TRITS_PER_TRYTE;
            e
        })
        .cloned()
        .collect();
    let message: Vec<Trit> = payload_iter.by_ref().take(length).cloned().collect();
    let nonce: Vec<Trit> = payload_iter
        .by_ref()
        .skip({
            let (l, e) = pascal::decode(&payload[(message_length_trits.len() + message.len())..]);
            length = l * TRITS_PER_TRYTE;
            e
        })
        .take(length)
        .cloned()
        .collect();
    let hash = {
        let mut curl = C::default();
        curl.absorb(&message_length_trits);
        curl.absorb(&message);
        curl.absorb(&nonce);
        curl.squeeze(HASH_LENGTH)
    };
    let security = iss::checksum_security(&hash);
    if security != 0 {
        let calculated_root: Vec<Trit> = {
            let signature: Vec<Trit> = payload_iter
                .by_ref()
                .take(security * iss::KEY_LENGTH / 3)
                .cloned()
                .collect();
            let siblings: Vec<Vec<Trit>> = {
                let end_trits: Vec<Trit> = payload_iter.by_ref().cloned().collect();
                let (l, e) = pascal::decode(&end_trits);
                end_trits[e..l]
                    .chunks(HASH_LENGTH)
                    .map(|c| c.to_vec())
                    .collect()
            };
            let address: Vec<Trit> =
                iss::address::<Trit, C>(&iss::digest_bundle_signature::<C>(&hash, &signature));
            merkle::root(&address, &siblings, index)
        };
        if calculated_root
            .iter()
            .zip(root.iter())
            .filter(|t| t.0 != t.1)
            .count() == 0
        {
            let next_root: Vec<Trit> = message[..HASH_LENGTH].to_vec();
            let message_out: Vec<Trit> = message[HASH_LENGTH..].to_vec();
            Ok((message_out, next_root))
        } else {
            Err(MamError::InvalidSignature)
        }
    } else {
        Err(MamError::InvalidHash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use curl_cpu::*;
    use alloc::Vec;
    use alloc::*;
    use merkle;
    use sign::iss;
    #[test]
    fn it_works() {
        let seed: Trinary = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9\
                             ABCDEFGHIJKLMNOPQRSTUVWXYZ9\
                             ABCDEFGHIJKLMNOPQRSTUVWXYZ9"
            .chars()
            .collect();
        let message: Trinary = "IAMSOMEMESSAGE9HEARMEROARMYMESSAGETOTHEWORLDYOUHEATHEN"
            .chars()
            .collect();
        let start = 1;
        let count = 9;
        let index = 3;
        let (key, hashes, root) = {
            let my_keys = merkle::keys(&seed.trits(), start, count);
            let addresses: Vec<Vec<Trit>> = my_keys
                .iter()
                .map(|k| {
                    iss::address::<Trit, CpuCurl<Trit>>(&iss::digest_key::<Trit, CpuCurl<Trit>>(&k))
                })
                .collect();
            let my_key: Vec<Trit> = my_keys[index].clone();
            let my_hashes: Vec<Vec<Trit>> = merkle::siblings(&addresses, index);
            let root = merkle::root(
                &addresses[index],
                &merkle::siblings(&addresses, index),
                index,
            );
            (my_key, my_hashes, root)
        };
        let next_root = {
            let next_keys = merkle::keys(&seed.trits(), start + count, count);
            let next_addrs: Vec<Vec<Trit>> = next_keys
                .iter()
                .map(|k| {
                    iss::address::<Trit, CpuCurl<Trit>>(&iss::digest_key::<Trit, CpuCurl<Trit>>(&k))
                })
                .collect();
            let next_addr = next_addrs[index].clone();
            merkle::root(&next_addr, &merkle::siblings(&next_addrs, index), index)
        };
        let payload = sign::<CpuCurl<Trit>, CpuHam>(&message.trits(), &next_root, &key, &hashes, 3);
        let payload_trinary: Trinary = payload.iter().cloned().collect();
        let payload_str = payload_trinary.to_string();
        let (message_calc, next_root_calc) = authenticate::<CpuCurl<Trit>>(&payload, &root, index)
            .ok()
            .unwrap();
        assert_eq!(message_calc, message.trits());
        assert_eq!(next_root_calc, next_root);
    }
}
/*
*/
