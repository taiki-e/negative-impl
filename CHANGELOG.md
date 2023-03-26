# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

## [0.1.3] - 2023-03-26

- Update `syn` dependency to 2. This increase the minimum supported Rust version from Rust 1.37 to Rust 1.56.

## [0.1.2] - 2022-02-05

- Support `UnwindSafe` and `RefUnwindSafe` in no-std at Rust 1.56+. ([#3](https://github.com/taiki-e/negative-impl/pull/3))

## [0.1.1] - 2021-11-10

- Suppress `clippy::non_send_fields_in_send_ty` lint in generated code. ([#2](https://github.com/taiki-e/negative-impl/pull/2))

## [0.1.0] - 2021-03-27

Initial release

[Unreleased]: https://github.com/taiki-e/negative-impl/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/taiki-e/negative-impl/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/taiki-e/derive_utils/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/taiki-e/derive_utils/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/taiki-e/negative-impl/releases/tag/v0.1.0
