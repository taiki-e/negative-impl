// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::panic::{RefUnwindSafe, UnwindSafe};

use negative_impl::negative_impl;
use static_assertions::assert_not_impl_all as assert_not_impl;

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
