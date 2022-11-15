//! A mapping between categories
//!
//! A functor is a type that implements a function `rmap` that sastifies the following laws.
//!
//! Identity
//!
//!     rmap(identity, x) == identity(x)
//!
//! Composition
//!
//!     rmap(rmap(x, y), z) == rmap(x, rmap(y, z))
//!

pub trait Functor {
    /// Alias for the type that we are generic over.
    type Inner;
    /// The type we are generic over and it's container.
    type Wrapped<T>: Functor;

    /// Lift a function into the Functor context.
    fn rmap<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Inner) -> B;
}

impl<A> Functor for Option<A> {
    type Inner = A;
    type Wrapped<T> = Option<T>;

    fn rmap<B, F: FnOnce(Self::Inner) -> B>(self, f: F) -> Self::Wrapped<B> {
        match self {
            None => None,
            Some(a) => Some(f(a)),
        }
    }
}

impl<T, E> Functor for Result<T, E> {
    type Inner = T;
    type Wrapped<U> = Result<U, E>;

    fn rmap<B, F: FnOnce(Self::Inner) -> B>(self, f: F) -> Self::Wrapped<B> {
        match self {
            Err(e) => Err(e),
            Ok(a) => Ok(f(a)),
        }
    }
}

pub fn fmap<F: Functor, B>(f: impl FnOnce(F::Inner) -> B, fa: F) -> F::Wrapped<B>
where
{
    fa.rmap(f)
}

#[cfg(test)]
mod test {
    use super::fmap;
    use crate::id;

    #[test]
    fn functor_option_identity() {
        let i: Option<i32> = Some(2);
        let j: Option<i32> = Some(2);
        assert_eq!(fmap(id, i), j);
        assert_eq!(fmap(id, Some(2)), Some(2));
    }

    #[test]
    fn functor_option_none() {
        assert_eq!(fmap::<Option<i32>, _>(|x| x * 2, None), None);
    }

    #[test]
    fn functor_option_some() {
        assert_eq!(fmap(|val| val * 2, Some(2)), Some(4));
    }
}
