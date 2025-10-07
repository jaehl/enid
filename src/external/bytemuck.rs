#![cfg(feature = "bytemuck")]

use crate::enid::{Enid40, Enid80};
use bytemuck::{Pod, TransparentWrapper, Zeroable};

unsafe impl Zeroable for Enid40 {}

unsafe impl Zeroable for Enid80 {}

unsafe impl Pod for Enid40 {}

unsafe impl Pod for Enid80 {}

unsafe impl TransparentWrapper<[u8; 5]> for Enid40 {}

unsafe impl TransparentWrapper<[u8; 10]> for Enid80 {}
