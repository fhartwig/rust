// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// FIXME: Remove when `item_like_imports` is stabilized.

#![feature(relaxed_adts)]

pub mod c {
    pub struct S {}
    pub struct TS();
    pub struct US;
    pub enum E {
        V {},
        TV(),
        UV,
    }

    pub struct Item;
}

pub mod proxy {
    pub use c::*;
    pub use c::E::*;
}

pub mod xm1 {
    pub use ::proxy::*;
    pub type S = ::c::Item;
}
pub mod xm2 {
    pub use ::proxy::*;
    pub const S: ::c::Item = ::c::Item;
}

pub mod xm3 {
    pub use ::proxy::*;
    pub type TS = ::c::Item;
}
pub mod xm4 {
    pub use ::proxy::*;
    pub const TS: ::c::Item = ::c::Item;
}

pub mod xm5 {
    pub use ::proxy::*;
    pub type US = ::c::Item;
}
pub mod xm6 {
    pub use ::proxy::*;
    pub const US: ::c::Item = ::c::Item;
}

pub mod xm7 {
    pub use ::proxy::*;
    pub type V = ::c::Item;
}
pub mod xm8 {
    pub use ::proxy::*;
    pub const V: ::c::Item = ::c::Item;
}

pub mod xm9 {
    pub use ::proxy::*;
    pub type TV = ::c::Item;
}
pub mod xmA {
    pub use ::proxy::*;
    pub const TV: ::c::Item = ::c::Item;
}

pub mod xmB {
    pub use ::proxy::*;
    pub type UV = ::c::Item;
}
pub mod xmC {
    pub use ::proxy::*;
    pub const UV: ::c::Item = ::c::Item;
}
