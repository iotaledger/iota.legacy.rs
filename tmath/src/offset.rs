use trytes::BCTrit;
use trytes::constants::RADIX;
use core::mem;

pub trait Offset {
    /// Given a `value`, this offsets bctrit words starting at `value`
    fn offset(&mut self, value: isize);
}

#[inline]
pub fn trits_in_word_offset() -> usize {
    let usize_size = mem::size_of::<usize>() * 8;
    // get the number of trits needed for a full set of offset binary coded trits
    let mut o = 1;
    while (RADIX.pow(o) as usize) < usize_size {
        o += 1;
    }
    o as usize
}

impl<'a> Offset for &'a mut [BCTrit] {
    fn offset(&mut self, value: isize) {
        let usize_size = mem::size_of::<usize>() * 8;
        let num_offset_trits = trits_in_word_offset();

        let mut shift: usize = {
            let max = (RADIX as isize).pow(num_offset_trits as u32);
            if value.is_negative() {
                (max + value % max) as usize
            } else {
                (value % max) as usize
            }
        };
        
        let mut base: usize = RADIX as usize;
        let mut in_shift = 0;
        for i in 0..num_offset_trits {
            let num_expanded: usize = RADIX.pow(i as u32 + 1) as usize;
            let out_shift = num_expanded / RADIX as usize;
            if shift != 0 {
                let m = shift % num_expanded;
                in_shift += m;
                shift -= m;
            }
            if in_shift < usize_size {
                self[i].1 = base >> (in_shift % num_expanded);
            }

            let mut j = num_expanded - in_shift % num_expanded;
            while j < usize_size {
                self[i].1 |= base.rotate_left(j as u32);
                j += num_expanded;
            }
            self[i].0 = self[i].1 >> out_shift;

            j -= out_shift;

            let k: isize = j as isize -
                if j >= usize_size {
                    num_expanded as isize
                } else {
                    0
                };
            self[i].0 |= if k.is_negative() {
                base.rotate_right(-k as u32)
            } else {
                base.rotate_left(k as u32)
            };
            base |= base.rotate_left((num_expanded * 2 / 3) as u32);
            base |= base.rotate_left(((num_expanded / 3) << 1) as u32);
            in_shift += out_shift;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use trytes::*;
    #[test]
    fn offset_works() {

        let usize_size = mem::size_of::<usize>() * 8;
        let mut out: [BCTrit; 10] = [(0, 0); 10];
        let mut trit: [Trit; 10] = [0; 10];
        let mut t = out;

        for i in 0..68 {
            (&mut out[..]).offset(i);
            for j in 0..usize_size {
                for (i, v) in TrinaryDemultiplexer::new(&out).get(j).enumerate() {
                    trit[i] = v;
                }
                let k = num::trits2int(&trit) as i64;
                let r = if k < 0 { (81 + k) } else { k };
                //assert_eq!(r, ((j as i64 + i as i64) % 81));
            }
            for t in out.iter_mut() {
                *t = (0, 0);
            }
        }
    }
}
