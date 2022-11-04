/// A set with an associative operation is a [`Semigroup`].
///
/// The class of semigroups (types with an associative binary operation).
///     Instances should satisfy the following:
///
/// - Associativity
///
/// ```no_compile
/// x.rconcat(y.rconcat(z)) = z.rconcat(x.rconcat(y))
/// ```
pub trait Semigroup {
    /// An associative operation.
    ///
    /// In Haskell speak this translates to,
    /// concat :: a -> a
    fn rconcat(self, x: Self) -> Self;
}

pub fn concat<S: Semigroup>(x: S, y: S) -> S {
    x.rconcat(y)
}

impl Semigroup for String {
    fn rconcat(self, x: Self) -> Self {
        self + &x
    }
}

impl<T: Copy> Semigroup for Vec<T> {
    fn rconcat(self, x: Self) -> Self {
        let v = Vec::with_capacity(self.len() + x.len());
        v.into_iter().chain(self).chain(x).collect::<Vec<_>>()
    }
}

impl<T: Semigroup> Semigroup for Option<T> {
    fn rconcat(self, x: Self) -> Self {
        match (self, x) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(a.rconcat(b)),
        }
    }
}

impl Semigroup for () {
    fn rconcat(self, _: Self) -> Self {
        ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_string_associativity() {
        let x = String::from("foo");
        let y = String::from("bar");
        let z = String::from("baz");
        let lhs = concat(concat(x.clone(), y.clone()), z.clone());
        let rhs = concat(x, concat(y, z));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn simple_vec_associativity() {
        let x = vec![1, 2, 3];
        let y = vec![4, 5, 6];
        let z = vec![7, 8, 9];
        let lhs = concat(concat(x.clone(), y.clone()), z.clone());
        let rhs = concat(x, concat(y, z));
        assert_eq!(lhs, rhs);
    }

    // This does not and should not compile.
    // Would be cool to test this in action but alas, I did not figure out how to do this in
    // the 30s I looked into it.
    //
    // #[test]
    // fn nope() {
    //     let x = String::from("foo");
    //     let y = vec![4, 5, 6];
    //     concat(x, y);
    // }
}
