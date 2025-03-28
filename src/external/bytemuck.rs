#![cfg(feature = "bytemuck")]

use crate::enid::{Enid40, Enid80};
use bytemuck::{Pod, TransparentWrapper, Zeroable};

#[cfg_attr(docsrs, doc(cfg(feature = "bytemuck")))]
unsafe impl Zeroable for Enid40 {}

#[cfg_attr(docsrs, doc(cfg(feature = "bytemuck")))]
unsafe impl Zeroable for Enid80 {}

#[cfg_attr(docsrs, doc(cfg(feature = "bytemuck")))]
unsafe impl Pod for Enid40 {}

#[cfg_attr(docsrs, doc(cfg(feature = "bytemuck")))]
unsafe impl Pod for Enid80 {}

#[cfg_attr(docsrs, doc(cfg(feature = "bytemuck")))]
unsafe impl TransparentWrapper<[u8; 5]> for Enid40 {}

#[cfg_attr(docsrs, doc(cfg(feature = "bytemuck")))]
unsafe impl TransparentWrapper<[u8; 10]> for Enid80 {}
