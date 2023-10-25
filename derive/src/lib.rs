//
// Original Copyright 2017 Idan Arye
// Modifications Copyright 2023 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

#![doc = include_str!("../README.md")]

use syn::{parse_macro_input, DeriveInput};

mod body_impl;
mod default_attr;
mod util;

/// Derive the `PartialDefault` trait.
///
/// The value used for a field can be overridden using the `#[partial_default(value =
/// "alternative()")]` syntax, where `alternative()` is a Rust expression that evaluates to the
/// correct type.
///
/// By default, the derived implementation will add a `T: PartialDefault` trait for every generic
/// parameter, like the built-in `derive(Default)`. You can override this by adding
/// `#[partial_default(bound = "T: MyTrait")]` to the type, which replaces any inferred bounds. Use
/// an empty string to impose no restrictions at all.
///
/// # Examples
///
/// ```
/// use partial_default::PartialDefault;
///
/// # fn main() {
/// #[derive(PartialDefault)]
/// #[partial_default(bound = "")]
/// # #[derive(PartialEq)]
/// # #[allow(dead_code)]
/// enum Foo<T> {
///     Bar,
///     #[partial_default]
///     Baz {
///         #[partial_default(value = "12")]
///         a: i32,
///         b: i32,
///         #[partial_default(value = "Some(Default::default())")]
///         c: Option<i32>,
///         #[partial_default(value = "vec![1, 2, 3]")]
///         d: Vec<u32>,
///         #[partial_default(value = r#""four".to_owned()"#)]
///         e: String,
///     },
///     Qux(T),
/// }
///
/// assert!(Foo::<&u8>::partial_default() == Foo::<&u8>::Baz {
///     a: 12,
///     b: 0,
///     c: Some(0),
///     d: vec![1, 2, 3],
///     e: "four".to_owned(),
/// });
/// # }
/// ```
#[proc_macro_derive(PartialDefault, attributes(partial_default))]
pub fn derive_partial_default(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match body_impl::impl_my_derive(&input) {
        Ok(output) => output.into(),
        Err(error) => error.to_compile_error().into(),
    }
}
