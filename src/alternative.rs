use crate::applicative::Applicative;

/// A monoid on applicative functors.
pub trait Alternative: Applicative {
    /// Identity of the mirror.
    fn empty() -> Self::Wrapped<Self::Inner>;

    /// An associative binary operation.
    fn mirror(
        l: Self::Wrapped<Self::Inner>,
        r: Self::Wrapped<Self::Inner>,
    ) -> Self::Wrapped<Self::Inner>;
}

impl<A> Alternative for Option<A> {
    fn empty() -> Self::Wrapped<Self::Inner> {
        None
    }

    fn mirror(
        l: Self::Wrapped<Self::Inner>,
        r: Self::Wrapped<Self::Inner>,
    ) -> Self::Wrapped<Self::Inner> {
        match (l, r) {
            (None, r) => r,
            (l, _) => l,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn alt_opt() {
        let l = Some(4u8);
        let r = None;
        assert_eq!(<Option<u8> as Alternative>::empty(), None);
        assert_eq!(<Option<u8> as Alternative>::mirror(l, r), l);
    }
}
