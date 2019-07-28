/*
 * tag/role.rs
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

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Role(EZString);

impl Role {
    pub fn new<I: Into<String>>(name: I) -> Self {
        let name = name.into();
        assert_ne!(name, "", "Empty role names are not permitted");
        Role(EZString::from(name))
    }
}

impl AsRef<str> for Role {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<String> for Role {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl Borrow<str> for Role {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Borrow<String> for Role {
    fn borrow(&self) -> &String {
        &self.0
    }
}

impl Deref for Role {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl Debug for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Role({:?})", *self.0)
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self)
    }
}
