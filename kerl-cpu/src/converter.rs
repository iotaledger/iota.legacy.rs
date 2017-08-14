use trytes::Trit;
use trytes::constants::RADIX;
use core;
use core::intrinsics;

use super::constants::*;

/// `3**242/2`
const HALF_3: [u32; 12] = [
    0xa5ce8964,
    0x9f007669,
    0x1484504f,
    0x3ade00d9,
    0x0c24486e,
    0x50979d57,
    0x79a4c702,
    0x48bbae36,
    0xa9f6808b,
    0xaa06a805,
    0xa87fabdf,
    0x5e69ebef,
];

pub fn trits_to_bytes(trits: &[Trit], bytes: &mut [u8]) {
    assert_eq!(trits.len(), TRIT_LENGTH);
    assert_eq!(bytes.len(), BYTE_LENGTH);

    // We _know_ that the sizes match.
    // So this is safe enough to do and saves us a few allocations.
    let base: &mut [u32] =
        unsafe { core::slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut u32, 12) };

    base.clone_from_slice(&[0; 12]);

    let mut size = 1;
    let mut all_minus_1 = true;

    for t in trits[0..TRIT_LENGTH - 1].iter() {
        if *t != -1 {
            all_minus_1 = false;
            break;
        }
    }

    if all_minus_1 {
        base.clone_from_slice(&HALF_3);
        bigint_not(base);
        bigint_add_small(base, 1_u32);
    } else {
        for t in trits[0..TRIT_LENGTH - 1].iter().rev() {
            // multiply by radix
            {
                let sz = size;
                let mut carry: u32 = 0;

                for j in 0..sz {
                    let v = (base[j] as u64) * (RADIX as u64) + (carry as u64);
                    let (newcarry, newbase) = ((v >> 32) as u32, v as u32);
                    carry = newcarry;
                    base[j] = newbase;
                }

                if carry > 0 {
                    base[sz] = carry;
                    size += 1;
                }
            }

            let trit = (t + 1) as u32;
            // addition
            {
                let sz = bigint_add_small(base, trit);
                if sz > size {
                    size = sz;
                }
            }
        }

        if !is_null(base) {
            if bigint_cmp(&HALF_3, base) <= 0 {
                // base >= HALF_3
                // just do base - HALF_3
                bigint_sub(base, &HALF_3);
            } else {
                // we don't have a wrapping sub.
                // so let's use some bit magic to achieve it
                let mut tmp = HALF_3.clone();
                bigint_sub(&mut tmp, base);
                bigint_not(&mut tmp);
                bigint_add_small(&mut tmp, 1_u32);
                base.clone_from_slice(&tmp);
            }
        }
    }

    bytes.reverse();
}

/// This will consume the input bytes slice and write to trits.
pub fn bytes_to_trits(bytes: &mut [u8], trits: &mut [Trit]) {
    assert_eq!(bytes.len(), BYTE_LENGTH);
    assert_eq!(trits.len(), TRIT_LENGTH);

    trits[TRIT_LENGTH - 1] = 0;

    bytes.reverse();
    // We _know_ that the sizes match.
    // So this is safe enough to do and saves us a few allocations.
    let base: &mut [u32] =
        unsafe { core::slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut u32, 12) };

    if is_null(base) {
        trits.clone_from_slice(&[0; TRIT_LENGTH]);
        return;
    }

    let mut flip_trits = false;

    if base[INT_LENGTH - 1] >> 31 == 0 {
        // positive number
        // we need to add HALF_3 to move it into positvie unsigned space
        bigint_add(base, &HALF_3);
    } else {
        // negative number
        bigint_not(base);
        if bigint_cmp(base, &HALF_3) > 0 {
            bigint_sub(base, &HALF_3);
            flip_trits = true;
        } else {
            bigint_add_small(base, 1 as u32);
            let mut tmp = HALF_3.clone();
            bigint_sub(&mut tmp, base);
            base.clone_from_slice(&tmp);
        }
    }

    let mut rem;
    for i in 0..TRIT_LENGTH - 1 {
        rem = 0;
        for j in (0..INT_LENGTH).rev() {
            let lhs = ((rem as u64) << 32) | (base[j] as u64);
            let rhs = RADIX as u64;
            let q = (lhs / rhs) as u32;
            let r = (lhs % rhs) as u32;

            base[j] = q;
            rem = r;
        }
        trits[i] = rem as i8 - 1;
    }

    if flip_trits {
        for v in trits.iter_mut() {
            *v = -*v;
        }
    }
}

#[inline]
fn bigint_not(base: &mut [u32]) {
    for i in base.iter_mut() {
        *i = !*i;
    }
}

#[inline]
fn bigint_add_small(base: &mut [u32], other: u32) -> usize {
    let (mut carry, v) = full_add(base[0], other, false);
    base[0] = v;

    let mut i = 1;
    while carry {
        let (c, v) = full_add(base[i], 0, carry);
        base[i] = v;
        carry = c;
        i += 1;
    }

    i
}

#[inline]
fn bigint_add(base: &mut [u32], rh: &[u32]) {
    let mut carry = false;

    for (a, b) in base.iter_mut().zip(rh.iter()) {
        let (c, v) = full_add(*a, *b, carry);
        *a = v;
        carry = c;
    }
}

#[inline]
fn bigint_cmp(lh: &[u32], rh: &[u32]) -> i8 {
    for (a, b) in lh.iter().rev().zip(rh.iter().rev()) {
        if a < b {
            return -1;
        } else if a > b {
            return 1;
        }
    }
    return 0;
}

#[inline]
fn bigint_sub(base: &mut [u32], rh: &[u32]) {
    let mut noborrow = true;
    for (a, b) in base.iter_mut().zip(rh) {
        let (c, v) = full_add(*a, !*b, noborrow);
        *a = v;
        noborrow = c;
    }
    assert!(noborrow);
}
#[inline]
fn is_null(base: &[u32]) -> bool {
    for b in base.iter() {
        if *b != 0 {
            return false;
        }
    }
    return true;
}

#[inline(always)]
fn full_add(lh: u32, rh: u32, carry: bool) -> (bool, u32) {
    let (v, carry1) = unsafe { intrinsics::add_with_overflow(lh, rh) };
    let (v, carry2) = unsafe { intrinsics::add_with_overflow(v, if carry { 1 } else { 0 }) };
    (carry1 || carry2, v)
}


#[cfg(test)]
mod tests {
    use super::*;
    use trytes::*;
    use std::*;
    use alloc::Vec;


    fn tbt(trytes: &str) {
        let mut trits_in: Vec<Trit> = trytes.chars().flat_map(char_to_trits).cloned().collect();
        trits_in[TRIT_LENGTH - 1] = 0;
        let mut trits_out = trits_in.clone();

        let mut bytes = [0 as u8; BYTE_LENGTH];

        trits_to_bytes(&trits_in, &mut bytes);
        bytes_to_trits(&mut bytes, &mut trits_out);


        assert_eq!(trits_in, trits_out);
    }

    fn bt(bytes_in: &[u8], expected: &str) {
        let mut trits = [0 as Trit; TRIT_LENGTH];
        let mut bytes = [0 as u8; BYTE_LENGTH];
        bytes.clone_from_slice(&bytes_in);
        bytes_to_trits(&mut bytes, &mut trits);

        let trytes_out = trits_to_string(&trits);
        assert_eq!(expected, trytes_out.unwrap());
    }

    #[test]
    fn trits_all_bytes() {
        for i in 0..256 {
            let mut bytes = [i as u8; BYTE_LENGTH];
            let mut trits = [0 as Trit; TRIT_LENGTH];
            let mut norm_trits = [0 as Trit; TRIT_LENGTH];

            bytes_to_trits(&mut bytes, &mut trits);
            trits_to_bytes(&trits, &mut bytes);
            bytes_to_trits(&mut bytes, &mut norm_trits);

            assert_eq!(trits.to_vec(), norm_trits.to_vec());
        }
    }

    #[test]
    fn trits_bytes_trits() {
        let trytes = "SCYLJDWIM9LIXCSLETSHLQOOFDKYOVFZLAHQYCCNMYHRTNIKBZRIRACFYPOWYNSOWDNXFZUG9OEOZPOTD";
        tbt(trytes);

        let trytes = "999999999999999999999999999999999999999999999999999999999999999999999999999999999";
        tbt(trytes);

        let trytes = "ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ";
        tbt(trytes);

        let trytes = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        tbt(trytes);

        let trytes = "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM";
        tbt(trytes);

        let trytes = "NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN";
        tbt(trytes);
    }


    #[test]
    fn bytes_trits() {
        let bytes = [0 as u8; BYTE_LENGTH];
        bt(
            &bytes,
            "999999999999999999999999999999999999999999999999999999999999999999999999999999999",
        );

        let bytes = [32 as u8; BYTE_LENGTH];
        bt(
            &bytes,
            "FKMPGCDVPEYWPGTBVRFDVVRURXFHVDPGHBTEWHEBDCKOL9AVTISEFCWMDHTUBWBOFPSQERRDQ9MFGFINB",
        );

        let bytes = [127 as u8; BYTE_LENGTH];
        bt(
            &bytes,
            "LWWOYBGUIARWDZWMLWORYDNDMTEXKSWLW9HPHYPZW9GABECSCPBFOTVTBRUUNVPBVXYNGAVMKONVGABBX",
        );

        let bytes = [128 as u8; BYTE_LENGTH];
        bt(
            &bytes,
            "NDDLBYTFRZIDWADNODLIBWMWNGVCPHDOD9SKSBKAD9TZYVXHXKYULGEGYIFFMEKYECBMTZENPLMETZYYC",
        );

        let bytes = [220 as u8; BYTE_LENGTH];
        bt(
            &bytes,
            "KEBSORVSJYXMUTVPUAYYBY9LXXSTMHTDQXFNSAFUPMKSRJWNUPXKSQH9ABNIRHWYUWVNNKYRAXGGGCUIY",
        );

        let bytes = [255 as u8; BYTE_LENGTH];
        bt(
            &bytes,
            "Z99999999999999999999999999999999999999999999999999999999999999999999999999999999",
        );
    }

}
