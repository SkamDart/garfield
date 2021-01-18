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
    /// An associative operation.
    ///
    /// In Haskell speak this translates to,
    /// concat :: a -> a
    fn concat<T>(self, other: T) -> T;
}

/// Monoid extends the power of [`Semigroup`] by providing an identity element or "empty" value.
pub trait Monoid: Semigroup {
    /// The identity element associated with an associative operation.
    type Empty;
}

/// Functor
pub trait Functor {
    /// Alias for the type that we are generic over.
    type Inner;
    /// The type we are generic over and it's container.
    type Wrapped<T>: Functor;

    /// Lifts a function into the Functor context.
    ///
    /// In Haskell speak this translates to,
    /// fmap :: a -> b -> f a -> f b
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

/// A functor with application, providing operations to
/// - Embed pure expressions using [`pure`]
/// - Sequence computations and combine their results using [`lift_a2`]
pub trait Applicative: Functor {
    /// Lift a value into a functor
    ///
    /// In Haskell speak this translates to,
    /// pure :: (Functor f) => a -> f a
    fn pure(a: Self::Inner) -> Self::Wrapped<Self::Inner>;
    // Run a computation inside a functor.
    //
    // In Haskell speak this translates to,
    // (<*>) :: (Functor f) => f (a -> b) -> f a -> f b -> f b
    fn lift_a2<B, F, C>(f: F, a: Self::Wrapped<Self::Inner>, b: Self::Wrapped<B>) -> Self::Wrapped<C>
    where
        F: Fn(Self::Inner, B) -> C;
}

impl<A> Applicative for Option<A> {
    fn pure(val: Self::Inner) -> Self::Wrapped<A> {
        Some(val)
    }

    fn lift_a2<B, F, C>(f: F, fa: Self::Wrapped<Self::Inner>, fb: Self::Wrapped<B>) -> Self::Wrapped<C>
    where
         F: Fn(Self::Inner, B) -> C,
    {
        match (fa, fb) {
            (Some(a), Some(b)) => Some(f(a, b)),
            _ => None,
        }
    }
}

impl<T, E> Applicative for Result<T, E> {
    fn pure(val: Self::Inner) -> Self::Wrapped<Self::Inner> {
        Ok(val)
    }

    fn lift_a2<B, F, C>(f: F, fa: Self::Wrapped<Self::Inner>, fb: Self::Wrapped<B>) -> Self::Wrapped<C>
    where
        F: Fn(Self::Inner, B) -> C,
    {
        match (fa, fb) {
            (Ok(a), Ok(b)) => Ok(f(a, b)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
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

    #[test]
    fn applicative_pure() {
        assert_eq!(<Option<u32> as Applicative>::pure(20), Some(20));
    }

    #[test]
    fn applicative_option_some_lift_a2() {
        let plus = |x: u32, y: u32| x + y;
        assert_eq!(<Option<u32> as Applicative>::lift_a2(plus, Some(1730), Some(8)), Some(1738));
    }

    #[test]
    fn applicative_option_none_lift_a2() {
        let plus = |x: u32, y: u32| x + y;
        assert_eq!(<Option<u32> as Applicative>::lift_a2(plus, None, None), None);
    }

    #[test]
    fn applicative_option_some_none_lift_a2() {
        let plus = |x: u32, y: u32| x + y;
        assert_eq!(<Option<u32> as Applicative>::lift_a2(plus, None, Some(1738)), None);
    }
}
