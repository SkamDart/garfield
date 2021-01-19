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
