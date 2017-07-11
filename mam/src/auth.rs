use alloc::*;
use curl::*;
use sign::iss;
use tmath::*;
use trytes::*;

pub fn sign<C, H>(
    message_in: &Trinary,
    next: &Trinary,
    key: &Trinary,
    hashes: Vec<Trinary>,
) -> Trinary
where
    C: Curl<Trit>,
    H: HammingNonce,
{
    let ms = vec![next.clone(), message_in.clone()];
    let message = ms.as_slice().trinary();
    let message_length_trits: Trinary = num::int2trits(message.len_trits() as isize)
        .into_iter()
        .collect();
    let message_length_trits_length_trits: Trinary = num::int2trits(
        message_length_trits.len_trits() as isize,
    ).into_iter()
        .collect();
    let min_nonce_length = 27;
    let security = 3;
    let message_nonce: Trinary = H::search(&message.trits(), min_nonce_length, security).unwrap();
    let mut curl = C::default();
    curl.absorb(&message_length_trits.trits());
    curl.absorb(&message.trits());
    curl.absorb(&message_nonce.trits());
    let signature =
        iss::signature::<C>(curl.squeeze(HASH_LENGTH).into_iter().collect(), key.clone());
    let siblings: Trinary = hashes.as_slice().trinary();
    let siblings_length: Trinary = num::int2trits(hashes.len() as isize).into_iter().collect();
    let siblings_length_length: Trinary = num::int2trits(siblings_length.len_trits() as isize)
        .into_iter()
        .collect();
    let payload = vec![
        message_length_trits_length_trits,
        message_length_trits,
        message.clone(),
        message_nonce,
        signature,
        siblings_length_length,
        siblings_length,
        siblings,
    ];
    payload.as_slice().trinary()
}

#[cfg(test)]
mod tests {
    use super::*;
    use curl_cpu::*;
    use alloc::Vec;
    use alloc::*;
    use alloc::string::ToString;
    #[test]
    fn it_works() {}
}
