//! Test for https://github.com/rust-lang/rust/pull/152003

#![feature(type_info)]

use std::any::TypeId;

trait Trait {}
impl Trait for [u8; 1 << 63] {}

fn main() {}

const _: () = const {
    TypeId::of::<[u8; 1 << 63]>().trait_info_of_trait_type_id(TypeId::of::<dyn Trait>());
    //~^ ERROR values of the type `[u8; 9223372036854775808]` are too big for the target architecture [E0080]
};
const _: () = const {
    TypeId::of::<[u8; 1 << 63]>().trait_info_of::<dyn Trait>();
    //~^ ERROR values of the type `[u8; 9223372036854775808]` are too big for the target architecture [E0080]
};
