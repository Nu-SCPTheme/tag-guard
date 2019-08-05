/*
 * tag/tag.rs
 *
 * tag-guard - Configurable tag enforcement library
 * Copyright (c) 2019 Ammon Smith
 *
 * tag-guard is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

use easy_strings::EZString;
use std::borrow::Borrow;
use std::fmt::{self, Debug, Display};
use std::ops::Deref;

/// An owned reference to a tag.
///
/// Essentially an immutable wrapper over a [`String`], which allows cheap cloning
/// to avoid reallocating buffers. Used to represent a particular, case-sensitive tag.
///
/// See also [`Role`].
///
/// [`String`]: https://doc.rust-lang.org/stable/std/string/struct.String.html
/// [`Role`]: ./struct.Role.html
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Tag(EZString);

impl Tag {
    pub fn new<I: Into<String>>(name: I) -> Self {
        let name = name.into();
        assert_ne!(name, "", "Empty tag names are not permitted");
        Tag(EZString::from(name))
    }
}

impl AsRef<str> for Tag {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<String> for Tag {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl Borrow<str> for Tag {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Borrow<String> for Tag {
    fn borrow(&self) -> &String {
        &self.0
    }
}

impl Deref for Tag {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tag({:?})", *self.0)
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self)
    }
}
