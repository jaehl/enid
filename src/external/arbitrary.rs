#![cfg(feature = "arbitrary")]

use crate::enid::{Enid, Enid40, Enid80};
use arbitrary::{size_hint, Arbitrary, Result, Unstructured};

impl<'a> Arbitrary<'a> for Enid40 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        u.arbitrary().map(Self::from_bytes)
    }

    #[inline]
    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        (5, Some(5))
    }
}

impl<'a> Arbitrary<'a> for Enid80 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        u.arbitrary().map(Self::from_bytes)
    }

    #[inline]
    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        (10, Some(10))
    }
}

impl<'a> Arbitrary<'a> for Enid {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        if u.arbitrary()? {
            u.arbitrary().map(Self::Enid40)
        } else {
            u.arbitrary().map(Self::Enid80)
        }
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        size_hint::and(
            bool::size_hint(depth),
            size_hint::or(Enid40::size_hint(depth), Enid80::size_hint(depth)),
        )
    }
}
