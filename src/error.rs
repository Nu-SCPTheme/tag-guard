/*
 * error.rs
 *
 * tag-guard - Configurable tag enforcement library
 * Copyright (c) 2019 Ammon Smith
 *
 * tag-guard is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

use std::error::Error as StdError;
use std::fmt;

#[must_use = "should handle errors"]
#[derive(Debug)]
pub enum Error {
    RequiresTags((), Vec<()>),
    IncompatibleTags((), ()),
}

impl StdError for Error {
    fn description(&self) -> &str {
        unimplemented!()
    }

    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", StdError::description(self))
    }
}
