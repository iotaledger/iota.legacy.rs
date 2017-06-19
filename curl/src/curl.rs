use trytes::Trinary;

pub trait Curl {
   fn absorb(&mut self, trinary: Trinary);
   fn squeeze(&mut self, length: usize) -> Trinary;
}
