//! # Garfield
//!
//! # Background
//! Cat-egory theory concepts in Rust. This library is intended to playfully introduce
//! people to the power and expressiveness of functional programming abstractions
//! without having to break into the walled garden of languages such as Haskell,
//! Idris, and Agda. The essence of these languages, and more generally, functional programming,
//! is function compositon. As a result, you will notice that _almost_ all of the traits and functions
//! provided by this library are free functions. In Rust, this means that we will not use `impl`
//! blocks and assign associated methods and functions to types. Note, I borrowed the notion of
//! associative functions and methods from the [functions](https://doc.rust-lang.org/rust-by-example/fn/methods.html)
//! section in the rust-by-example book.
//!
//! For example, we prefer this
//!
//! ```rust
//! struct Point {
//!     x: f64,
//!     y: f64,
//! }
//!
//! fn add(lhs: Point, rhs: Point) -> Point {
//!     Point {
//!         x: lhs.x + rhs.x,
//!         y: lhs.y + rhs.y,
//!     }
//! }
//! ```
//!
//! over the functionally equivalent
//!
//! ```rust
//! struct Point {
//!     x: f64,
//!     y: f64,
//! }
//!
//! impl Point {
//!     fn add(self, other: Self) -> Self {
//!         Self {
//!             x: self.x + other.x,
//!             y: self.y + other.y,
//!         }
//!     }
//! }
//! ```
//!
//! The former is visually closer to the syntax of functional programming languages,
//! ergo, I have found it much easier to translate to/from the Standard ML derivative languages.
//!
//! ## But why?
//! The concepts implemented in this library give us axioms that borrow from the mathematics of
//! computation which allows us to quickly write correct and reuseable software.
//!
//! Big thank you to those who worked towards [GAT Stabilization](https://blog.rust-lang.org/2022/10/28/gats-stabilization.html)
//! in Rust 1.65 that provided us with our first taste of in higher kinded types in Rust.
#![warn(clippy::all, clippy::pedantic)]

/// A monoid on applicative functors
pub mod alternative;
pub mod applicative;
pub mod functor;
pub mod monad;
pub mod semigroup;
pub use {
    alternative::Alternative,
    applicative::Applicative,
    functor::Functor,
    monad::Monad,
    semigroup::Semigroup,
};

/// Identity Morphism
///
/// For any input, this will give you the same element back.
#[inline]
pub const fn id<T>(t: T) -> T {
    t
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
}
