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
    fn fmap<B, F>(f: F, fa: Self::Wrapped<Self::Inner>) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Inner) -> B;
}

impl<A> Functor for Option<A> {
    type Inner = A;
    type Wrapped<T> = Option<T>;

    fn fmap<B, F: FnOnce(Self::Inner) -> B>(
        f: F,
        fa: Self::Wrapped<Self::Inner>,
    ) -> Self::Wrapped<B> {
        match fa {
            None => None,
            Some(a) => Some(f(a)),
        }
    }
}

impl<T, E> Functor for Result<T, E> {
    type Inner = T;
    type Wrapped<U> = Result<U, E>;

    fn fmap<B, F: FnOnce(Self::Inner) -> B>(
        f: F,
        fa: Self::Wrapped<Self::Inner>,
    ) -> Self::Wrapped<B> {
        match fa {
            Err(e) => Err(e),
            Ok(a) => Ok(f(a)),
        }
    }
}

pub fn rmap<F, B>(f: impl FnOnce(F::Inner) -> B, fa: F::Wrapped<F::Inner>) -> F::Wrapped<B>
where
    F: Functor,
{
    <F as Functor>::fmap(f, fa)
}

#[cfg(test)]
mod test {
    use super::rmap;
    use crate::id;

    #[test]
    fn functor_option_identity() {
        assert_eq!(rmap::<Option<i32>, _>(id, Some(2)), Some(2));
    }

    #[test]
    fn functor_option_none() {
        assert_eq!(rmap::<Option<i32>, _>(|x| x * 2, None), None);
    }

    #[test]
    fn functor_option_some() {
        assert_eq!(rmap::<Option<i32>, i32>(|val| val * 2, Some(2)), Some(4));
    }
}
