use constants::{Trit, BCTrit};
use alloc::Vec;

use bct::*;

pub trait IntoTrits<T> {
    fn len_trits(&self) -> usize;
    fn trits(&self) -> Vec<T>;
}

pub trait FromTrits<T>
where
    Self: Sized,
{
    type Err;
    fn from_trits(trits: &[T]) -> Result<Self, Self::Err>;
}


// BCTrit
impl IntoTrits<BCTrit> for Vec<BCTrit> {
    fn len_trits(&self) -> usize {
        self.len()
    }

    fn trits(&self) -> Vec<BCTrit> {
        self.clone()
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

impl<'a> IntoTrits<BCTrit> for &'a [BCTrit] {
    fn len_trits(&self) -> usize {
        self.len()
    }

    fn trits(&self) -> Vec<BCTrit> {
        self.to_vec()
    }
}

impl FromTrits<BCTrit> for Vec<BCTrit> {
    type Err = ();
    fn from_trits(trits: &[BCTrit]) -> Result<Self, Self::Err> {
        Ok(trits.iter().cloned().collect())
    }
}

// Trit
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

impl<'a> IntoTrits<Trit> for &'a [Trit] {
    fn len_trits(&self) -> usize {
        self.len()
    }

    fn trits(&self) -> Vec<Trit> {
        self.to_vec()
    }
}


impl FromTrits<Trit> for Vec<Trit> {
    type Err = ();
    fn from_trits(trits: &[Trit]) -> Result<Self, Self::Err> {
        Ok(trits.iter().cloned().collect())
    }
}
