use trytes::*;

pub trait Search<T> {
    fn search(
        &[Trit],
        length: usize,
        group: usize,
        check: fn(&[T]) -> Option<usize>,
    ) -> Option<Trinary>;
}
