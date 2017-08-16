use curl::*;
use tmath::*;
use trytes::*;

#[inline]
pub fn search_prepare_trits<C: Curl<Trit>, CB: Curl<BCTrit>>(c: &C, cb: &mut CB, offset: usize) {
    for (b, &t) in cb.state_mut().iter_mut().zip(c.state().iter()) {
        *b = trit_to_bct(t);
    }
    (&mut cb.state_mut()[offset..]).offset(0);
}
