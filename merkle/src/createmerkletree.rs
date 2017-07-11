use trytes::*;
use tmath::*;
use curl::*;
use curl_cpu::*;
use sign::iss;
use core::mem;
use alloc::*;

fn offset_outer(acc: usize, i: usize, offset: usize, length: usize, size: usize) -> BCTrit {
    if i < size {
        offset_outer(
            acc | (offset_inner(if (offset + i) % 3 == 1 { 0 } else { 1 }, length, 0) << length),
            i + length,
            offset,
            length,
            size,
        )
    } else {
        (
            acc,
            (acc << length) | offset_inner(if offset % 3 == 1 { 0 } else { 1 }, length, 0),
        )
    }
}
fn offset_inner(value: usize, length: usize, index: usize) -> usize {
    if index >= length {
        return value;
    }
    value | offset_inner(value, length, index + 1)
}
fn get_bct_offset(length: usize, size: usize) -> BCTrit {
    let low: usize = (0..(size / length)).into_iter().fold(0, |acc, x| {
        (acc << length) |
            (0..length).into_iter().fold(
                if x % 3 == 1 { 0 } else { 1 },
                |acu, y| acu | (acu << y),
            )
    });
    let hi: usize = (low << length) | (0..length).into_iter().fold(1, |a, x| a | (a << x));
    (low, hi)
}

pub fn new_merkle_tree(seed: Trinary, start: usize, count: usize, index: usize) {
    /// Generate Some number of keys
    //let keys = (|seed, start, count| {})(seed, start, count);
    let usize_size = mem::size_of::<usize>() * 8;
    let keys: Vec<Vec<BCTrit>> = {
        let vec_size = count / usize_size + if count % usize_size != 0 { 1 } else { 0 };
        let mut trits: Vec<Trit> = seed.trits();
        // Increment to the start of the merkle tree
        for _ in 0..start {
            trits.as_mut_slice().incr();
        }
        let base_num_trits: usize = {
            let mut num = 0;
            while {
                num += 1;
                usize_size > 3usize.pow(num)
            }
            {}
            num as usize
        };
        let start = num::trits2int(&trits[..base_num_trits]);
        let offsets: Vec<BCTrit> = {
            let offset_start = if start < 0 {
                start + 3isize.pow(base_num_trits as u32 - 1) * 2
            } else {
                start
            };
            (0..base_num_trits)
                .into_iter()
                .map(|i| offset_outer(0, 0, start as usize, i, usize_size))
                .collect()
        };
        // fill the subseed vector
        (0..vec_size).into_iter().map(|i| {
                let mut bc_trits: Vec<BCTrit> = trits.iter().map(|&t| trit_to_bct(t)).collect();
                // bc_trits.offset();
                // We now have our starting point
                //
                // Figure out if we need to increment again after some point
                // Do a "smart offset" of the bc_trits
                {
                    // figure out how many trits you need ( this should probably be some global
                    // constant computed at compile time rather than inline )
                    let trit_range = {
                        //0b1101101101101101101101101101101101101101101101101101101101101101 or
                        0
                    };
                    //smart_offset(bc_trits);
                    // If the value of the first n trits, given by the word size is greater than some
                    // value, determined by some method, then
                    // increment to that point, and then increment once past it,
                    // and offset from there
                }
                // Now we have a subseed starting at the increment point, offset by the number of bits
                // in `usize`
                if i < vec_size - 1 {
                    for _ in 0..usize_size {
                        trits.as_mut_slice().incr();
                    }
                }
                bc_trits
                // increment to the next point in the subseed array
            }).map(|s| iss::key::<BCTrit, CpuCurl<BCTrit>>(&s)).collect()
    };
}
