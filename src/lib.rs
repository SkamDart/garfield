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

/// Functor
pub trait Functor {
    type Inner;
    type Wrapped<T>: Functor;

    fn fmap<B, F: FnOnce(Self::Inner) -> B>(self, f: F) -> Self::Wrapped<B>;
}

impl<A> Functor for Option<A> {
    type Inner = A;
    type Wrapped<T> = Option<T>;

    fn fmap<B, F: FnOnce(Self::Inner) -> B>(self, f: F) -> Self::Wrapped<B> {
        match self {
            None => None,
            Some(a) => Some(f(a)),
        }
    }
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

    #[test]
    fn functor_option_identity() {
        assert_eq!(Some(2).fmap(id), Some(2));
    }

    #[test]
    fn functor_option_none() {
        // need to help out type inference here.
        let none: Option<u32> = None;
        assert_eq!(none.fmap(|x| x * 2), None);
    }

    #[test]
    fn functor_option_some() {
        let some = Some(2);
        assert_eq!(some.fmap(|val| val * 2), Some(4));
    }
}
