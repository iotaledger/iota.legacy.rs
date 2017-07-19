use alloc::Vec;
use alloc::string::*;
use constants::*;
use types::*;
use util::*;
use num;

const MAX_ENCODED_VAL: Trit = 3;
const ENCODER_MASK: isize = 7;

enum PascalIter {
    Reading,
    Encoding,
}

pub fn decode(input: &IntoTrits<Trit>) -> (usize, usize) {
    let mut positive: Vec<Trit> = Vec::with_capacity(TRITS_PER_TRYTE);
    let negative: Vec<Trit> = input
        .trits()
        .chunks(TRITS_PER_TRYTE)
        .take_while(|tryte| {
            let val = num::trits2int(tryte);
            if val.is_positive() {
                positive.extend_from_slice(*tryte);
                false
            } else {
                true
            }
        })
        .flat_map(|t| t.to_vec())
        .collect();
    let encoders_start = negative.len() + positive.len();
    let num_encoder_pairs = {
        let num_negative_trytes = negative.len() / 3;
        num_negative_trytes / 3 + if num_negative_trytes % 3 == 0 { 0 } else { 1 }
    };
    let encoders: Vec<isize> = input.trits()[encoders_start..]
        .chunks(2)
        .take(num_encoder_pairs)
        .map(num::trits2int)
        .collect();
    let corrected_negatives: Vec<Trit> = negative
        .chunks(TRITS_PER_TRYTE.pow(2))
        .zip(encoders.iter())
        .flat_map(|(trytes, e)| {
            let mut i = 0;
            let mut encoder = *e + TRITS_PER_TRYTE as isize;
            let mut trytes_out: Vec<Trit> = Vec::with_capacity(trytes.len());
            for tryte in trytes.chunks(TRITS_PER_TRYTE) {
                trytes_out.extend(if ((encoder >> i) & 1isize) != 0isize {
                    tryte.iter().map(|trit| -trit).collect()
                } else {
                    tryte.to_vec()
                });
                i += 1;
            }
            trytes_out.into_iter()
        })
        .chain(positive.into_iter())
        .collect();
    (
        num::trits2int(&corrected_negatives) as usize,
        encoders_start + num::round_third(num_encoder_pairs * 2),
    )
}

pub fn encode(input: usize) -> Vec<Trit> {
    let length = num::round_third(num::min_trits(input as isize) as usize) as u8;
    let negative_length = (length as usize - TRITS_PER_TRYTE) / TRITS_PER_TRYTE;
    let triplet_count = negative_length / TRITS_PER_TRYTE +
        if negative_length % TRITS_PER_TRYTE == 0 {
            0
        } else {
            1
        };
    let encoder_trit_count = triplet_count * 2;
    let encoder_trits_size = num::round_third(encoder_trit_count);
    let mut encoding = 0;
    let mut trits: Vec<Trit> = {
        let mut myvec = num::int2trits(input as isize, length);
        while myvec.len() != myvec.capacity() {
            myvec.push(0);
        }
        let mut index = 0;
        myvec
            .chunks(TRITS_PER_TRYTE)
            .map(|c| {
                let val = num::trits2int(c);
                let out = if val.is_positive() && index < negative_length {

                    encoding |= 1 << index;
                    c.iter().map(|t| -t).collect()
                } else {
                    c.to_vec()
                };
                index += 1;
                out
            })
            .fold(
                Vec::with_capacity(length as usize + encoder_trits_size),
                |mut acc, v| {
                    acc.extend(v);
                    acc
                },
            )
    };
    trits.extend({
        let mut out: Vec<Trit> = (0..triplet_count)
            .into_iter()
            .map(|i| {
                let j = i * 3;
                let val = ((encoding >> j) & ENCODER_MASK) as isize - TRITS_PER_TRYTE as isize;
                let mut res = num::int2trits(val, 2);
                while res.len() != res.capacity() {
                    res.push(0);
                }
                res
            })
            .fold(Vec::with_capacity(encoder_trits_size), |mut acc, v| {
                acc.extend(v);
                acc
            });
        while out.len() != out.capacity() {
            out.push(0);
        }
        out
    });
    trits
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::*;
    use num;

    #[test]
    fn from_encoder_trytes() {
        let num_trytes: Vec<Trit> = "ABXDEFG".trits();
        let num_val = num::trits2int(&num_trytes) as usize;
        let length = encode(num_val);
        let expect_trytes: Vec<Trit> = "ZYXWVUGIA".trits();

        assert_eq!(expect_trytes, length);
        let (val, end) = decode(&length);
        assert_eq!(val, num_val);
        assert_eq!(end, expect_trytes.len());
    }
}
