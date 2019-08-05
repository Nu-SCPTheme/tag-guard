/*
 * lib.rs
 *
 * tag-guard - Configurable tag enforcement library
 * Copyright (c) 2019 Ammon Smith
 *
 * tag-guard is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

#![deny(missing_debug_implementations)]

//! A library to enforce user-specified tag relationships.
//!
//! Ensures that all tagsets observe all specified rules,
//! such as tag A conflicting with tags B or C, or requiring
//! the presence of tag D.
//!
//! The library also provides functionality to verify that tagsets
//! match certain rules or restrictions.
//!
//! The actual meaning of the tags, or what objects they are applied
//! to is up to the consumer of the library.

extern crate easy_strings;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate str_macro;

mod engine;
mod error;
mod tag;

#[cfg(test)]
mod test;

pub mod load;

pub use self::engine::Engine;
pub use self::error::Error;
pub use self::tag::{Role, Tag, TagSpec, TemplateTagSpec};

/// An alias for the [`Result`] type found in the standard library.
///
/// [`Result`]: https://doc.rust-lang.org/stable/std/result/enum.Result.html
pub type StdResult<T, E> = std::result::Result<T, E>;

/// A [`Result`] type with the [`Error`] used by this crate.
///
/// [`Error`]: ./enum.Error.html
/// [`Result`]: https://doc.rust-lang.org/stable/std/result/enum.Result.html
pub type Result<T> = StdResult<T, Error>;

pub mod prelude {
    //! A "prelude" module, intended to be star-imported: `use tag_guard::prelude::*;`

    pub use super::{Engine, Error, Role, Tag, TagSpec, TemplateTagSpec};
}
