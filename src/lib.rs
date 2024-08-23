// SPDX-License-Identifier: Apache-2.0 OR MIT

/*!
Negative trait implementations on stable Rust.

This crate emulates the [unstable `negative_impls` feature](https://doc.rust-lang.org/nightly/unstable-book/language-features/negative-impls.html)
by [generating a trait implementation with a condition that will never be true](https://github.com/taiki-e/negative-impl/issues/6#issuecomment-1669714453).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
negative-impl = "0.1"
```

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

```rust,compile_fail,E0119
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
*/

#![doc(test(
    no_crate_inject,
    attr(
        deny(warnings, rust_2018_idioms, single_use_lifetimes),
        allow(dead_code, unused_variables)
    )
))]
#![forbid(unsafe_code)]

#[macro_use]
mod error;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{
    parse_quote, token, Error, Generics, ItemImpl, Lifetime, LifetimeParam, Path, Result, Token,
    Type,
};

#[proc_macro_attribute]
pub fn negative_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    attribute(&args.into(), syn::parse_macro_input!(input))
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn attribute(args: &TokenStream2, mut impl_: ItemImpl) -> Result<TokenStream2> {
    parse_as_empty(args)?;

    let (not_token, trait_path, for_token) = match impl_.trait_.take() {
        Some((Some(not_token), path, for_token)) => (not_token, path, for_token),
        Some((_, path, _)) => bail!(path, "may only be used on negative trait impls"),
        None => bail!(impl_, "may only be used on negative trait impls"),
    };
    // https://github.com/rust-lang/rust/issues/80481
    impl_.attrs.push(parse_quote!(#[doc(hidden)]));

    if impl_.unsafety.is_some() {
        bail!(quote!(#not_token #trait_path), "negative impls cannot be unsafe");
    }
    if let Some(item) = impl_.items.first() {
        bail!(item, "negative impls cannot have any items");
    }

    let TraitInfo { trivial_bounds, unsafety, maybe_unsized, full_path } =
        TraitInfo::new(&trait_path)?;

    let wrapper_lifetime = Lifetime::new("'__wrapper", Span::call_site());
    let wrapper_ident = format_ident!("__Wrapper");

    let trivial_bounds = parse_quote!(
        #wrapper_ident<#wrapper_lifetime, #trivial_bounds>: #full_path
    );
    impl_.generics.make_where_clause().predicates.push(trivial_bounds);

    insert_lifetime(&mut impl_.generics, wrapper_lifetime);

    let unsafety = if unsafety { Some(<Token![unsafe]>::default()) } else { None };

    let sized = if maybe_unsized { Some(quote!(: ?Sized)) } else { None };
    let wrapper = quote! {
        pub struct #wrapper_ident<'a, T #sized>(::core::marker::PhantomData<&'a ()>, T);
        #unsafety impl<T #sized> #full_path for #wrapper_ident<'_, T>
            where T: #full_path {}
    };

    impl_.trait_ = Some((None, full_path, for_token));
    impl_.unsafety = unsafety;
    Ok(quote! {
        const _: () = {
            #wrapper
            // This is false positive as we generate a trait implementation with a condition that will never be true.
            #[allow(clippy::non_send_fields_in_send_ty)]
            #impl_
        };
    })
}

struct TraitInfo {
    trivial_bounds: Type,
    unsafety: bool,
    maybe_unsized: bool,
    full_path: Path,
}

impl TraitInfo {
    fn new(path: &Path) -> Result<Self> {
        match &*path.segments.last().unwrap().ident.to_string() {
            "Send" => Ok(Self {
                // https://github.com/rust-lang/rust/blob/1.37.0/src/libcore/marker.rs#L41
                // https://github.com/rust-lang/rust/blob/1.70.0/library/core/src/marker.rs#L43
                trivial_bounds: parse_quote!(*const ()),
                unsafety: true,
                maybe_unsized: true,
                full_path: parse_quote!(::core::marker::Send),
            }),
            "Sync" => Ok(Self {
                // https://github.com/rust-lang/rust/blob/1.37.0/src/libcore/marker.rs#L380
                // https://github.com/rust-lang/rust/blob/1.70.0/library/core/src/marker.rs#L547
                trivial_bounds: parse_quote!(*const ()),
                unsafety: true,
                maybe_unsized: true,
                full_path: parse_quote!(::core::marker::Sync),
            }),
            "Unpin" => Ok(Self {
                // https://github.com/rust-lang/rust/blob/1.37.0/src/libcore/marker.rs#L650
                // https://github.com/rust-lang/rust/blob/1.70.0/library/core/src/marker.rs#L840
                trivial_bounds: parse_quote!(::core::marker::PhantomPinned),
                unsafety: false,
                maybe_unsized: true,
                full_path: parse_quote!(::core::marker::Unpin),
            }),
            "UnwindSafe" => Ok(Self {
                // https://github.com/rust-lang/rust/blob/1.37.0/src/libstd/panic.rs#L203
                // https://github.com/rust-lang/rust/blob/1.70.0/library/core/src/panic/unwind_safe.rs#L181
                trivial_bounds: parse_quote!(&'static mut ()),
                unsafety: false,
                maybe_unsized: true,
                full_path: parse_quote!(::core::panic::UnwindSafe),
            }),
            "RefUnwindSafe" => Ok(Self {
                // https://github.com/rust-lang/rust/blob/1.37.0/src/libstd/panic.rs#L234
                // https://github.com/rust-lang/rust/blob/1.70.0/library/core/src/panic/unwind_safe.rs#L200
                trivial_bounds: parse_quote!(::core::cell::UnsafeCell<()>),
                unsafety: false,
                maybe_unsized: true,
                full_path: parse_quote!(::core::panic::RefUnwindSafe),
            }),
            _ => bail!(path, "non auto traits are not supported"),
        }
    }
}

/// Inserts a `lifetime` at position `0` of `generics.params`.
fn insert_lifetime(generics: &mut Generics, lifetime: Lifetime) {
    generics.lt_token.get_or_insert_with(token::Lt::default);
    generics.gt_token.get_or_insert_with(token::Gt::default);
    generics.params.insert(0, LifetimeParam::new(lifetime).into());
}

/// Checks if `tokens` is an empty `TokenStream`.
///
/// This is almost equivalent to `syn::parse2::<Nothing>()`, but produces
/// a better error message and does not require ownership of `tokens`.
fn parse_as_empty(tokens: &TokenStream2) -> Result<()> {
    if tokens.is_empty() {
        Ok(())
    } else {
        bail!(tokens, "unexpected token: `{}`", tokens)
    }
}
