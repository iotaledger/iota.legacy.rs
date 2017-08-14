use curl::*;
use trytes::Trit;
use keccak::Keccak;

use constants::*;
use converter::*;


#[derive(Clone)]
pub struct CpuKerl {
    keccak: Keccak,
    state: [Trit; 0],
}

impl Default for CpuKerl {
    fn default() -> CpuKerl {
        CpuKerl {
            keccak: Keccak::new_keccak384(),
            state: [],
        }
    }
}

impl Sponge for CpuKerl {
    fn transform(&mut self) {
        self.keccak.keccakf();
    }

    fn reset(&mut self) {
        self.keccak = Keccak::new_keccak384();
    }
}


impl Curl<Trit> for CpuKerl
where
    Self: Sponge,
{
    fn absorb(&mut self, trits: &[Trit]) {
        assert_eq!(trits.len() % TRIT_LENGTH, 0);
        let mut bytes: [u8; BYTE_LENGTH] = [0; BYTE_LENGTH];

        for chunk in trits.chunks(TRIT_LENGTH) {
            trits_to_bytes(chunk, &mut bytes);
            self.keccak.update(&bytes);
        }
    }

    fn squeeze(&mut self, out: &mut [Trit]) {
        assert_eq!(out.len() % TRIT_LENGTH, 0);
        let mut bytes: [u8; BYTE_LENGTH] = [0; BYTE_LENGTH];

        for chunk in out.chunks_mut(TRIT_LENGTH) {
            self.keccak.pad();
            self.keccak.fill_block();
            self.keccak.squeeze(&mut bytes);
            self.reset();
            bytes_to_trits(&mut bytes.to_vec(), chunk);
            for b in bytes.iter_mut() {
                *b = *b ^ 0xFF;
            }
            self.keccak.update(&bytes);
        }
    }
    fn duplex(&mut self, _: &[Trit], _: &mut [Trit]) {
        unreachable!();
    }

    fn rate(&self) -> &[Trit] {
        &self.state
    }
    fn state(&self) -> &[Trit] {
        &self.state
    }
    fn state_mut(&mut self) -> &mut [Trit] {
        &mut self.state
    }
}


#[cfg(test)]
mod tests {
    use trytes::*;
    use alloc::Vec;

    use super::*;

    #[test]
    fn kerl_one_absorb() {
        let mut trits: Vec<Trit> = "EMIDYNHBWMBCXVDEFOFWINXTERALUKYYPPHKP9JJFGJEIUY9MUDVNFZHMMWZUYUSWAIOWEVTHNWMHANBH"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();


        let mut kerl = CpuKerl::default();
        kerl.absorb(&trits);
        kerl.squeeze(&mut trits);

        assert_eq!(
            trits_to_string(&trits).unwrap(),
            "EJEAOOZYSAWFPZQESYDHZCGYNSTWXUMVJOVDWUNZJXDGWCLUFGIMZRMGCAZGKNPLBRLGUNYWKLJTYEAQX"
        );
    }

    #[test]
    fn kerl_multi_squeeze_multi_absorb() {
        let trits: Vec<Trit> = "G9JYBOMPUXHYHKSNRNMMSSZCSHOFYOYNZRSZMAAYWDYEIMVVOGKPJBVBM9TD\
PULSFUNMTVXRKFIDOHUXXVYDLFSZYZTWQYTE9SPYYWYTXJYQ9IFGYOLZXWZBKWZN9QOOTBQMWMUBLEWUEEASRHRTNIQW\
JQNDWRYLCA"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();

        let mut kerl = CpuKerl::default();
        kerl.absorb(&trits);

        let mut out = vec![0 as Trit; 486];

        kerl.squeeze(&mut out);
        assert_eq!(
            trits_to_string(&out).unwrap(),
            "LUCKQVACOGBFYSPPVSSOXJEKNSQQRQKPZC9NXFSMQNRQCGGUL9OHVVKBDSKEQEBKXRNUJSRXYVHJTXBPD\
             WQGNSCDCBAIRHAQCOWZEBSNHIJIGPZQITIBJQ9LNTDIBTCQ9EUWKHFLGFUVGGUWJONK9GBCDUIMAYMMQX"
        );
    }

    #[test]
    fn kerl_multi_squeeze() {
        let trits: Vec<Trit> = "9MIDYNHBWMBCXVDEFOFWINXTERALUKYYPPHKP9JJFGJEIUY9MUDVNFZHMMWZUYUSWAIOWEVTHNWMHANBH"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();

        let mut kerl = CpuKerl::default();
        kerl.absorb(&trits);

        let mut out = vec![0 as Trit; 486];
        kerl.squeeze(&mut out);
        assert_eq!(
            trits_to_string(&out).unwrap(),
            "G9JYBOMPUXHYHKSNRNMMSSZCSHOFYOYNZRSZMAAYWDYEIMVVOGKPJBVBM9TDPULSFUNMTVXRKFIDOHUXX\
             VYDLFSZYZTWQYTE9SPYYWYTXJYQ9IFGYOLZXWZBKWZN9QOOTBQMWMUBLEWUEEASRHRTNIQWJQNDWRYLCA"
        );
    }


}
