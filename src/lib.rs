//! # Garfield
//!
//! Your guide to Category Theory concepts in Rust using [Generic Associated Types]().
#![feature(generic_associated_types)]
#![allow(incomplete_features)]
 #![warn(
     clippy::all,
     clippy::pedantic,
 )]

pub mod applicative;
pub mod functor;
pub mod monoid;
pub mod semigroup;

/// Identity Morphism
///
/// For any input, this will give you the same element back.
#[inline]
pub const fn id<T>(t: T) -> T {
    t
}

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq)]
    enum Size {
        Big,
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    struct Chungus {
        pub size: Size,
    }

    #[test]
    fn identity() {
        assert_eq!(id(1), 1);
        assert_eq!(id(vec![1]), vec![1]);
        let chungus = Chungus { size: Size::Big };
        assert_eq!(id(chungus), chungus);
    }
}
