use trytes::*;
use tmath::*;
use curl::*;
use alloc::Vec;

pub fn mask<C>(payload: &[Trit], keys: &[Vec<Trit>]) -> Vec<Trit>
where
    C: Curl<Trit>,
{
    let mut out: Vec<Trit> = Vec::with_capacity(payload.len());
    let mut curl = C::default();
    for key in keys {
        curl.absorb(&key);
    }
    for chunk in payload.chunks(HASH_LENGTH) {
        let key_chunk = curl.squeeze(chunk.len());
        out.extend(chunk.iter().zip(key_chunk.iter()).map(sum));
    }
    out
}

pub fn unmask<C>(payload: &[Trit], keys: &[Vec<Trit>]) -> Vec<Trit>
where
    C: Curl<Trit>,
{
    let mut out: Vec<Trit> = Vec::with_capacity(payload.len());
    let mut curl = C::default();
    for key in keys {
        curl.absorb(&key);
    }
    for chunk in payload.chunks(HASH_LENGTH) {
        let key_chunk: Vec<Trit> = curl.squeeze(chunk.len()).iter().map(|t| -t).collect();
        out.extend(chunk.iter().zip(key_chunk.iter()).map(sum));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use curl_cpu::*;
    use alloc::Vec;
    use alloc::*;
    use alloc::string::ToString;
    #[test]
    fn it_can_unmask() {
        let payload: Trinary = "AMESSAGEFORYOU9".chars().collect();
        let channel_key: Trinary = "MYBIGCHANNELKEY".chars().collect();
        let auth_id: Trinary = "MYMERKLEROOTHASH".chars().collect();
        let index: Trinary = "AEOWJID999999".chars().collect();
        let keys = vec![channel_key.trits(), auth_id.trits(), index.trits()];
        let cipher = mask::<CpuCurl<Trit>>(&payload.trits(), &keys);
        let plain: Trinary = unmask::<CpuCurl<Trit>>(&cipher.clone(), &keys)
            .into_iter()
            .collect();
        assert_eq!(plain.to_string(), payload.to_string());
    }
}