//! A functor that can embed pure expressions and sequence computation
use crate::functor::Functor;

/// A functor with application, providing operations to
/// - Embed pure expressions using [`Applicative::pure`]
/// - Sequence computations and combine their results using [`Applicative::lift_a2`]
pub trait Applicative: Functor {
    /// Lift a value into a functor
    ///
    /// In Haskell speak this translates to,
    /// pure :: (Functor f) => a -> f a
    fn pure(a: Self::Inner) -> Self::Wrapped<Self::Inner>;
    /// Run a computation inside a functor.
    ///
    /// In Haskell speak this translates to,
    /// (<*>) :: (Functor f) => f (a -> b) -> f a -> f b
    fn seq<F: Fn(Self::Inner) -> B, B>(f: F, fa: Self::Wrapped<Self::Inner>) -> Self::Wrapped<B>;
    /// Lift a binary function into actions
    ///
    /// lift_a2 :: (a -> b -> c) -> f a -> f b -> f c
    fn lift_a2<F: Fn(Self::Inner, B) -> C, B, C>(
        f: F,
        fa: Self::Wrapped<Self::Inner>,
        fb: Self::Wrapped<B>,
    ) -> Self::Wrapped<C>;
}

impl<A> Applicative for Option<A> {
    #[inline]
    fn pure(val: Self::Inner) -> Self::Wrapped<A> {
        Some(val)
    }

    #[inline]
    fn seq<F: Fn(Self::Inner) -> B, B>(f: F, fa: Self::Wrapped<Self::Inner>) -> Self::Wrapped<B> {
        match fa {
            Some(a) => Some(f(a)),
            _ => None,
        }
    }

    #[inline]
    fn lift_a2<F: Fn(Self::Inner, B) -> C, B, C>(
        f: F,
        fa: Self::Wrapped<Self::Inner>,
        fb: Self::Wrapped<B>,
    ) -> Self::Wrapped<C> {
        match (fa, fb) {
            (Some(a), Some(b)) => Some(f(a, b)),
            _ => None,
        }
    }
}

impl<T, E> Applicative for Result<T, E> {
    #[inline]
    fn pure(val: Self::Inner) -> Self::Wrapped<Self::Inner> {
        Ok(val)
    }

    #[inline]
    fn lift_a2<F: Fn(Self::Inner, B) -> C, B, C>(
        f: F,
        fa: Self::Wrapped<Self::Inner>,
        fb: Self::Wrapped<B>,
    ) -> Self::Wrapped<C> {
        match (fa, fb) {
            (Ok(a), Ok(b)) => Ok(f(a, b)),
            (Err(e), _) | (_, Err(e)) => Err(e),
        }
    }

    #[inline]
    fn seq<F: Fn(Self::Inner) -> B, B>(f: F, fa: Self::Wrapped<Self::Inner>) -> Self::Wrapped<B> {
        match fa {
            Ok(a) => Ok(f(a)),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Applicative;

    #[test]
    fn applicative_pure() {
        assert_eq!(<Option<u32> as Applicative>::pure(20), Some(20));
    }

    #[test]
    fn applicative_option_some_lift_a2() {
        let plus = |x: u32, y: u32| x + y;
        assert_eq!(
            <Option<u32> as Applicative>::lift_a2(plus, Some(1730), Some(8)),
            Some(1738)
        );
    }

    #[test]
    fn applicative_option_none_lift_a2() {
        let plus = |x: u32, y: u32| x + y;
        assert_eq!(
            <Option<u32> as Applicative>::lift_a2(plus, None, None),
            None
        );
    }

    #[test]
    fn applicative_option_some_none_lift_a2() {
        let plus = |x: u32, y: u32| x + y;
        assert_eq!(
            <Option<u32> as Applicative>::lift_a2(plus, None, Some(1738)),
            None
        );
    }
}
