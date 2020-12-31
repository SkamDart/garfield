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

/// A set with an associative operation is a [`Semigroup`].
///
/// The class of semigroups (types with an associative binary operation).
///     Instances should satisfy the following:
///
/// - Associativity
///
/// ```no_compile
/// x.concat(y.concat(z)) = z.concat(x.concat(y))
/// ```
pub trait Semigroup {
    fn concat<T>(self, other: T) -> T;
}

/// Monoid extends the power of [`Semigroup`] by providing an identity element or "empty" value.
pub trait Monoid: Semigroup {
    type Empty;
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

impl<T, E> Functor for Result<T, E> {
    type Inner = T;
    type Wrapped<U> = Result<U, E>;

    fn fmap<B, F: FnOnce(Self::Inner) -> B>(self, f: F) -> Self::Wrapped<B> {
        match self {
            Err(e) => Err(e),
            Ok(a) => Ok(f(a)),
        }
    }
}

pub trait Applicative: Functor {
    fn pure(a: Self::Inner) -> Self::Wrapped<Self::Inner>;
    // fn apply<B, Self::Wrapped<F: FnOnce()>>(self, fa: Self::Wrapped<Self::Inner>) -> Self::Wrapped<B>;
}

impl<A> Applicative for Option<A> {
    fn pure(val: Self::Inner) -> Self::Wrapped<A> {
        Some(val)
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

    #[test]
    fn applicative_pure() {
        assert_eq!(<Option<u32> as Applicative>::pure(20), Some(20));
    }
}
