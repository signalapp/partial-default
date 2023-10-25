//
// Copyright 2023 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

#![doc = include_str!("../README.md")]
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "derive")]
pub use partial_default_derive::PartialDefault;

/// A trait for giving a type a *non*-useful default value.
///
/// The standard [`Default`] trait documents its purpose as providing a "useful" default value.
/// However, some types (such as a Credential) don't have meaningful defaults, and yet there are
/// still uses for a known-initialized value:
///
/// - serde's hidden [`Deserializer::deserialize_in_place`][deserialize_in_place], which is
///   generally more efficient than the usual `deserialize`
/// - subtle's [`ConditionallySelectable::conditional_assign`][conditional_assign], for repeated
///   assignments with at least one success
/// - APIs that must produce results even when signalling an error out of band (like JNI functions)
///
/// `PartialDefault` satisfies this niche. A type that implements `PartialDefault` can provide a
/// value that is safe to drop or assign over, but promises nothing else about that value. Using it
/// in any other way may panic or produce unexpected results, though it should not be possible to
/// violate memory safety. That is, [`partial_default`][Self::partial_default] should always be a
/// "safe" function in the Rust sense.
///
/// The name "PartialDefault" is by analogy to [`PartialEq`]/[`Eq`] and [`PartialOrd`]/[`Ord`] in
/// the standard library: just as `PartialEq` provides weaker guarantees than `Eq` and `PartialOrd`
/// provides weaker guarantees than `Ord`, `PartialDefault` provides weaker guarantees than
/// `Default`. And just as every `Eq`-implementing type provides `PartialEq`, every
/// `Default`-implementing type provides `PartialDefault`.
///
/// # Derivable
///
/// Like [`Default`], `PartialDefault` supports `#[derive]` if all fields implement
/// `PartialDefault`. The value used for a field can be overridden using the
/// `#[partial_default(value = "alternative()")]` syntax, where `alternative()` is a Rust expression
/// that evaluates to the correct type.
///
/// By default, all generic parameters must implement `PartialDefault` to support deriving
/// `PartialDefault`. You can override this by adding `#[partial_default(bound = "T: MyTrait")]` to
/// the type, which replaces any bounds inferred by `PartialDefault`. Use an empty string to impose
/// no restrictions at all.
///
/// [deserialize_in_place]: https://docs.rs/serde/1.0.189/src/serde/de/mod.rs.html#546-568
/// [conditional_assign]: https://docs.rs/subtle/2.5.0/subtle/trait.ConditionallySelectable.html
pub trait PartialDefault: Sized {
    /// Returns a value that can be safely dropped or assigned over.
    fn partial_default() -> Self;
}

/// If a type does implement `Default`, its `PartialDefault` implementation will match.
impl<T: Default> PartialDefault for T {
    fn partial_default() -> Self {
        Self::default()
    }
}
