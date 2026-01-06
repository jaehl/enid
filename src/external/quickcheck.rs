#![cfg(feature = "quickcheck")]

use crate::enid::{Enid, Enid40, Enid80};
use core::mem::{self, MaybeUninit};
use quickcheck::{Arbitrary, Gen};

impl Arbitrary for Enid40 {
    fn arbitrary(g: &mut Gen) -> Self {
        let mut bytes = [const { MaybeUninit::uninit() }; 5];

        for byte in &mut bytes {
            byte.write(u8::arbitrary(g));
        }

        // SAFETY: All elements have been initialised.
        let bytes = unsafe { mem::transmute::<[MaybeUninit<u8>; 5], [u8; 5]>(bytes) };

        Self::from_bytes(bytes)
    }
}

impl Arbitrary for Enid80 {
    fn arbitrary(g: &mut Gen) -> Self {
        let mut bytes = [const { MaybeUninit::uninit() }; 10];

        for byte in &mut bytes {
            byte.write(u8::arbitrary(g));
        }

        // SAFETY: All elements have been initialised.
        let bytes = unsafe { mem::transmute::<[MaybeUninit<u8>; 10], [u8; 10]>(bytes) };

        Self::from_bytes(bytes)
    }
}

impl Arbitrary for Enid {
    fn arbitrary(g: &mut Gen) -> Self {
        if bool::arbitrary(g) {
            Self::Enid40(Enid40::arbitrary(g))
        } else {
            Self::Enid80(Enid80::arbitrary(g))
        }
    }
}
