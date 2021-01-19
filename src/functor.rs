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

#[cfg(test)]
mod test {
    use super::Functor;
    use crate::id;

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
