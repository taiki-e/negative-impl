# negative-impl

[![crates.io](https://img.shields.io/crates/v/negative-impl?style=flat-square&logo=rust)](https://crates.io/crates/negative-impl)
[![docs.rs](https://img.shields.io/badge/docs.rs-negative--impl-blue?style=flat-square)](https://docs.rs/negative-impl)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)](#license)
[![rustc](https://img.shields.io/badge/rustc-1.37+-blue?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![build status](https://img.shields.io/github/workflow/status/taiki-e/negative-impl/CI/main?style=flat-square&logo=github)](https://github.com/taiki-e/negative-impl/actions)

Negative trait implementations on stable Rust.

This crate emulates the [unstable `negative_impls` feature](https://doc.rust-lang.org/beta/unstable-book/language-features/negative-impls.html)
by generating a trait implementation with a condition that will never be true.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
negative-impl = "0.1"
```

*Compiler support: requires rustc 1.37+*

## Examples

```rust
use negative_impl::negative_impl;

pub struct Type {}

#[negative_impl]
impl !Send for Type {}
#[negative_impl]
impl !Sync for Type {}
```

## Supported traits

Currently this crate only supports [auto traits](https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits).

- [`Send`](https://doc.rust-lang.org/std/marker/trait.Send.html)
- [`Sync`](https://doc.rust-lang.org/std/marker/trait.Sync.html)
- [`Unpin`](https://doc.rust-lang.org/std/marker/trait.Unpin.html)
- [`UnwindSafe`](https://doc.rust-lang.org/std/panic/trait.UnwindSafe.html)
- [`RefUnwindSafe`](https://doc.rust-lang.org/std/panic/trait.RefUnwindSafe.html)

## Limitations

### Conflicting implementations

The following code cannot compile due to `impl<T: Send> Trait for T` and
`impl Trait for Type` conflict.

```rust
use negative_impl::negative_impl;

pub struct Type {}

#[negative_impl]
impl !Send for Type {}

trait Trait {}

impl<T: Send> Trait for T {}
impl Trait for Type {}
```

```text
error[E0119]: conflicting implementations of trait `Trait` for type `Type`:
  --> src/lib.rs:60:1
   |
14 | impl<T: Send> Trait for T {}
   | ------------------------- first implementation here
15 | impl Trait for Type {}
   | ^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Type`
```

The above code can be compiled using the unstable `negative_impls` feature.

```rust
#![feature(negative_impls)]

pub struct Type {}

impl !Send for Type {}

trait Trait {}

impl<T: Send> Trait for T {}
impl Trait for Type {}
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
