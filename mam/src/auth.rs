use alloc::*;
use curl::*;
use sign::iss;
use trytes::*;

pub fn sign<C, H>(
    message_in: &Trinary,
    next: &Trinary,
    key: &Trinary,
    hashes: &[Trinary],
) -> Trinary
where
    C: Curl<Trit>,
    H: HammingNonce,
{
    let message = vec![next.clone(), message_in.clone()].as_slice().trinary();
    let message_length: Trinary = num::int2trits(message.len_trytes() as isize)
        .into_iter()
        .collect();
    let message_length_length: Trinary = num::int2trits(message_length.len_trytes() as isize)
        .into_iter()
        .collect();
    let min_nonce_length = 27;
    let security = 3;
    let message_nonce: Trinary = H::search(&message.trits(), min_nonce_length, security).unwrap();
    let nonce_length: Trinary = num::int2trits(message_nonce.len_trytes() as isize)
        .into_iter()
        .collect();
    let nonce_length_length: Trinary = num::int2trits(nonce_length.len_trytes() as isize)
        .into_iter()
        .collect();
    let mut curl = C::default();
    curl.absorb(&message_length.trits());
    curl.absorb(&message.trits());
    curl.absorb(&message_nonce.trits());
    let signature =
        iss::signature::<C>(curl.squeeze(HASH_LENGTH).into_iter().collect(), key.clone());
    let siblings: Trinary = hashes.trinary();
    let siblings_length: Trinary = num::int2trits(hashes.len() as isize).into_iter().collect();
    let siblings_length_length: Trinary = num::int2trits(siblings_length.len_trits() as isize)
        .into_iter()
        .collect();
    let payload = vec![
        message_length_length,
        message_length,
        message.clone(),
        nonce_length_length,
        nonce_length,
        message_nonce,
        signature,
        siblings_length_length,
        siblings_length,
        siblings,
    ];
    payload.as_slice().trinary()
}

pub fn authenticate<C>(payload: &Trinary, root: &Trinary) -> () {
    let message_length_length: usize = {
        let trits: Vec<Trit> = payload.trits().into_iter().take(3).collect();
        num::trits2int(&trits) as usize
    };
    let message_length_trits = &payload.trits()[3..message_length_length];
    let message_length: usize = num::trits2int(message_length_trits) as usize;
    let message: Vec<Trit> = payload
        .trits()
        .into_iter()
        .skip(3 + message_length_length)
        .take(message_length)
        .collect();
    // TODO:
    // * Get Nonce
    // * compute message hash
    // * check that message hash has hamming weight of 0
    // * calculate signature length based upon message hash hamming weight length
    // * check signature againts message hash
    // * get hashes
    // * get merkle root with hashes and address of signature
    // * check merkle root against given merkle root
    // * return Result<( message, next_root )>
    // * error if signature doesn't work
}

#[cfg(test)]
mod tests {
    use super::*;
    use curl_cpu::*;
    use alloc::Vec;
    use alloc::*;
    use merkle;
    use sign::iss;
    use alloc::string::ToString;
    #[test]
    fn it_works() {
        let seed = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9\
                             ABCDEFGHIJKLMNOPQRSTUVWXYZ9\
                             ABCDEFGHIJKLMNOPQRSTUVWXYZ9";
        let message: Trinary = "IAMSOMEMESSAGE9HEARMEROARMYMESSAGETOTHEWORLDYOUHEATHEN"
            .chars()
            .collect();
        let start = 1;
        let count = 9;
        let index = 3;
        let (key, hashes) = {
            let my_keys = merkle::keys(seed.chars().collect(), start, count);
            let addresses: Vec<Vec<Trit>> = my_keys
                .iter()
                .map(|k| {
                    iss::address::<Trit, CpuCurl<Trit>>(&iss::digest_key::<Trit, CpuCurl<Trit>>(&k))
                })
                .collect();
            let my_key: Trinary = my_keys[index].clone().into_iter().collect();
            let my_hashes: Vec<Trinary> = merkle::siblings(&addresses, index)
                .into_iter()
                .map(|h| h.into_iter().collect())
                .collect();
            (my_key, my_hashes)
        };
        let next_root = {
            let next_keys = merkle::keys(seed.chars().collect(), start + count, count);
            let next_addrs: Vec<Vec<Trit>> = next_keys
                .iter()
                .map(|k| {
                    iss::address::<Trit, CpuCurl<Trit>>(&iss::digest_key::<Trit, CpuCurl<Trit>>(&k))
                })
                .collect();
            let next_key = next_keys[index].clone();
            merkle::root(&next_key, &merkle::siblings(&next_addrs, index), index)
                .into_iter()
                .collect()
        };
        let payload = sign::<CpuCurl<Trit>, CpuHam>(&message, &next_root, &key, &hashes);
    }
}
