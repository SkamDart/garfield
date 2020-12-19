//! # Garfield
//!
//! Your guide to Category Theory concepts in Rust using [Generic Associated Types]().
#![feature(generic_associated_types)]
#![allow(incomplete_features)]

/// Identity Morphism
///
/// For any input, this will give you the same element back.
pub fn id<T>(t: T) -> T {
    t
}

/// Semigroup
///
/// The class of semigroups (types with an associative binary operation).
///     Instances should satisfy the following:
///
/// Associativity
///     x <> (y <> z) = (x <> y) <> z
pub trait Semigroup {
    fn concat<T>(&self, other: T) -> T;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn identity() {
        assert_eq!(2 + 2, 4);
    }
}
