use globals::*;
use mappings::*;

/*
pub trait BytesHaveTrits {
    fn trits_of_length(&self, length: usize) -> Vec<Trit>;
    fn trits(&self) -> Vec<Trit>;
}
*/


pub fn bytes_to_trits_with_length(bytes: &[i8], length: usize) -> Vec<Trit> {
    let mut offset: usize = 0;
    //let mut out: Vec<Trit> = vec![0; bytes.len() * NUMBER_OF_TRITS_IN_A_BYTE];
    let mut out: Vec<Trit> = vec![0; length];
    let mut end: usize;
    let mut btpos: usize;
    for i in 0..bytes.len() {
        if offset >= length {
            break;
        }
        end = offset +
              if length - offset < NUMBER_OF_TRITS_IN_A_BYTE {
                  (length - offset)
              } else {
                  NUMBER_OF_TRITS_IN_A_BYTE
              };
        btpos = if bytes[i] < 0 {
            ((bytes[i] as isize) + HASH_LENGTH as isize) as usize
        } else {
            bytes[i] as usize
        };
        out[offset..end].clone_from_slice(&BYTE_TO_TRITS_MAPPINGS[btpos as usize]);
        offset += i + NUMBER_OF_TRITS_IN_A_BYTE;
    }
    out
}
pub fn bytes_to_trits(bytes: &[i8]) -> Vec<Trit> {
    bytes_to_trits_with_length(bytes, 0);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trits_of_length_works() {
        //let in_bytes: [Trit; 5] = [57, -56, 108, -99, 1];
        let in_bytes: [Trit; 6] = [20, 25, -14, -4, 83, 1];

        let exp: [Trit; 27] = [0, 1, -1, 1, -1, 0, 1, 0, -1, 1, 0, -1, 1, 1, 1, 0, 0, 0, -1, -1,
                               1, 0, 0, 1, -1, 0, 1];
        let res = in_bytes.trits_of_length(31);
        println!("{:?}", res);
        println!("{}", res.len());
        assert_eq!(res.as_slice(), exp);
    }
}
/*
 *
 memcpy(trits + offset,
 BYTE_TO_TRITS_MAPPINGS[
 bytes[i] < 0 ? (bytes[i] +  HASH_LENGTH) 
 : bytes[i]
 ],
 sizeof(trit_t) * (length - offset < NUMBER_OF_TRITS_IN_A_BYTE ? 
 (length - offset) :
 NUMBER_OF_TRITS_IN_A_BYTE)
 );
 */
