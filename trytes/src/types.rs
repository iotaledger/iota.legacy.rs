use constants::{Trit, BCTrit};
use alloc::Vec;

use bct::*;

pub trait IntoTrits<T> {
    fn len_trits(&self) -> usize;
    fn trits(&self) -> Vec<T>;
}

pub trait FromTrits<T> {
    fn from_trits(trits: &[T]) -> Self;
}


impl IntoTrits<BCTrit> for Vec<BCTrit> {
    fn len_trits(&self) -> usize {
        self.len()
    }

    fn trits(&self) -> Vec<BCTrit> {
        self.clone()
    }
}

impl IntoTrits<Trit> for Vec<Trit> {
    fn len_trits(&self) -> usize {
        self.len()
    }

    fn trits(&self) -> Vec<Trit> {
        self.clone()
    }
}

impl IntoTrits<BCTrit> for Vec<Trit> {
    fn len_trits(&self) -> usize {
        self.len()
    }

    fn trits(&self) -> Vec<BCTrit> {
        self.iter().map(|&t| trit_to_bct(t)).collect()
    }
}

impl IntoTrits<Trit> for Vec<BCTrit> {
    fn len_trits(&self) -> usize {
        self.len()
    }

    fn trits(&self) -> Vec<Trit> {
        self.iter().map(|&t| bct_to_trit(t)).collect()
    }
}


impl FromTrits<Trit> for Vec<Trit> {
    fn from_trits(trits: &[Trit]) -> Self {
        trits.iter().cloned().collect()
    }
}

impl FromTrits<BCTrit> for Vec<BCTrit> {
    fn from_trits(trits: &[BCTrit]) -> Self {
        trits.iter().cloned().collect()
    }
}
