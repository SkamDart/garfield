//! Definitely not a burrito
use crate::applicative::Applicative;

pub trait Monad: Applicative {
    fn bind<B, F: Fn(Self::Inner) -> Self::Wrapped<B>>(
        ma: Self::Wrapped<Self::Inner>,
        f: F,
    ) -> Self::Wrapped<B>;

    fn r#return(a: Self::Inner) -> Self::Wrapped<Self::Inner> {
        Self::pure(a)
    }
}

impl<A> Monad for Option<A> {
    fn bind<B, F: Fn(Self::Inner) -> Self::Wrapped<B>>(
        ma: Self::Wrapped<Self::Inner>,
        f: F,
    ) -> Self::Wrapped<B> {
        match ma {
            Some(a) => f(a),
            None => None,
        }
    }
}
