// Copyright (c) 2016, 2018 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! # Generate Build Time Information
//! `vergen`, when used in conjunction with cargo [build scripts], will
//! generate environment variables to use with the `env!` macro.  Below
//! is a list of the supported variables.
//!
//! Key                       | Sample Value
//! --------------------------|----------------------------------------
//! VERGEN_BUILD_TIMESTAMP    |2018-08-09T15:15:57.282334589+00:000
//! VERGEN_BUILD_DATE         |2018-08-09
//! VERGEN_SHA                |75b390dc6c05a6a4aa2791cc7b3934591803bc22
//! VERGEN_SHA_SHORT          |75b390d
//! VERGEN_COMMIT_DATE        |2018-08-08
//! VERGEN_TARGET_TRIPLE      |x86_64-unknown-linux-gnu
//! VERGEN_SEMVER             |v0.1.0
//! VERGEN_SEMVER_LIGHTWEIGHT |v0.1.0
//!
//! The variable generation can be toggled on or off at an individual level
//! via [ConstantsFlags](crate::constants::ConstantsFlags)
//!
//! ### Note on SEMVER
//! `VERGEN_SEMVER` can be generated via `git describe` or by
//! `env::var("CARGO_PKG_VERSION")`.
//!
//! By default, `SEMVER` uses `git describe` if possible, and falls back to `CARGO_PKG_VERSION`.
//!
//! If you wish to force `CARGO_PKG_VERSION`, toggle off `SEMVER` and toggle
//! on `SEMVER_FROM_CARGO_PKG`.
//!
//! # Re-build On Changed HEAD
//! `vergen` can also be configured to re-run `build.rs` when either `.git/HEAD` or
//! the file that `.git/HEAD` points at changes.
//!
//! This can behavior can be toggled on or of with the [REBUILD_ON_HEAD_CHANGE] flag.
//!
//! [REBUILD_ON_HEAD_CHANGE]: crate::constants::ConstantsFlags::REBUILD_ON_HEAD_CHANGE
//! [build scripts]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
//!
//! ## 'cargo:' Key Build Script Output
//! ```toml
//! [package]
//! #..
//! build = "build.rs"
//!
//! [dependencies]
//! #..
//!
//! [build-dependencies]
//! vergen = "2"
//! ```
//!
//! ### Example 'build.rs'
//! ```
//! extern crate vergen;
//!
//! use vergen::{ConstantsFlags, generate_cargo_keys};
//!
//! fn main() {
//!     // Setup the flags, toggling off the 'SEMVER_FROM_CARGO_PKG' flag
//!     let mut flags = ConstantsFlags::all();
//!     flags.toggle(ConstantsFlags::SEMVER_FROM_CARGO_PKG);
//!
//!     // Generate the 'cargo:' key output
//!     generate_cargo_keys(ConstantsFlags::all()).expect("Unable to generate the cargo keys!");
//! }
//! ```
//!
//! ### Use the constants in your code
//! ```
//! fn my_fn() {
//!     println!("Build Timestamp: {}", env!("VERGEN_BUILD_TIMESTAMP"));
//! }
//! ```
//!
//! ## **DEPRECATED** - `version.rs` File Build Script Output
//! Generate a `version.rs` file in `OUT_DIR` (defined by cargo) with up to 8 build time
//! constants.  This file can then be used with the `include!` macro to pull the
//! constants into your source for use.
//!
//! See the [generate_version_rs](crate::output::codegen::generate_version_rs) documentation
//! if you wish to use this method.
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]
#[macro_use]
extern crate bitflags;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;

mod constants;
mod output;

pub use crate::constants::ConstantsFlags;
pub use crate::output::codegen::generate_version_rs;
pub use crate::output::envvar::generate_cargo_keys;
