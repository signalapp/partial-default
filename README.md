[`PartialDefault`] is a trait for giving a type a *non*-useful default value.

The standard [`Default`][] trait documents its purpose as providing a "useful" default value. However, some types (such as a Credential) don't have meaningful defaults, and yet there are still uses for a known-initialized value:

- serde's hidden [`Deserializer::deserialize_in_place`][deserialize_in_place], which is generally more efficient than the usual `deserialize`
- subtle's [`ConditionallySelectable::conditional_assign`][conditional_assign], for repeated assignments with at least one success
- APIs that must produce results even when signalling an error out of band (like JNI functions)

`PartialDefault` satisfies this niche. A type that implements `PartialDefault` can provide a value that is safe to drop or assign over, but promises nothing else about that value. It provides a derive macro (opt-in, with the `derive` feature) and is `no_std` compatible.

[`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html
[deserialize_in_place]: https://docs.rs/serde/1.0.189/src/serde/de/mod.rs.html#546-568
[conditional_assign]: https://docs.rs/subtle/2.5.0/subtle/trait.ConditionallySelectable.html

# License and Contributions

`PartialDefault` was made to support [libsignal][], but is available for general use under the **[AGPLv3][]**. Still, this is meant to be a low-maintenance crate; do not expect active support or progress on feature requests.

Signal does accept external contributions to this project; however, signing a [CLA (Contributor License Agreement)][cla] is required for all contributions.

Copyright 2023 Signal Messenger, LLC.

The `partial-default-derive` crate contains code adapted from the [`rust-smart-default`][] crate, Copyright (c) 2017 Idan Arye, under the [MIT license][].

[libsignal]: https://github.com/signalapp/libsignal
[AGPLv3]: https://www.gnu.org/licenses/agpl-3.0.html
[cla]: https://signal.org/cla/
[`rust-smart-default`]: https://github.com/idanarye/rust-smart-default
[MIT license]: https://github.com/idanarye/rust-smart-default/blob/084c5cd5ddc3ddb98cc005b48141ec34607ecf7a/LICENSE
