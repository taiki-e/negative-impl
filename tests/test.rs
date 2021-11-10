#![warn(rust_2018_idioms, single_use_lifetimes)]

use std::panic::{RefUnwindSafe, UnwindSafe};

use negative_impl::negative_impl;
use static_assertions::{assert_impl_all as assert_impl, assert_not_impl_all as assert_not_impl};

pub mod basic {
    use super::*;

    pub struct Foo<T>(T);

    #[negative_impl]
    impl<T> !Send for Foo<T> {}
    assert_not_impl!(Foo<()>: Send);

    #[negative_impl]
    impl<T> !Sync for Foo<T> {}
    assert_not_impl!(Foo<()>: Sync);

    #[negative_impl]
    impl<T> !Unpin for Foo<T> {}
    assert_not_impl!(Foo<()>: Unpin);

    #[negative_impl]
    impl<T> !UnwindSafe for Foo<T> {}
    assert_not_impl!(Foo<()>: UnwindSafe);

    #[negative_impl]
    impl<T> !RefUnwindSafe for Foo<T> {}
    assert_not_impl!(Foo<()>: RefUnwindSafe);
}

// https://github.com/taiki-e/negative-impl#conditional-negative-impls
pub mod conditional {
    use super::*;

    pub struct A<T>(T);
    pub struct B<T>(T);
    pub struct C;

    #[negative_impl]
    impl<T> !Send for A<Option<T>> {}
    #[negative_impl]
    impl<T> !Send for B<Vec<T>> where T: Copy {}

    unsafe impl<T: Send> Send for A<Result<T, T>> {}
    unsafe impl<T: Send> Send for B<Result<T, T>> {}

    assert_not_impl!(A<Option<()>>: Send);
    assert_impl!(A<Result<(), ()>>: Send);
    assert_impl!(B<Result<(), ()>>: Send);

    assert_impl!(A<()>: Send);
    assert_impl!(B<core::cell::Cell<()>>: Send);
}
