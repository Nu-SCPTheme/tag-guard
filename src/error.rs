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
use std::fmt::{self, Display};
use super::{Role, Tag};

#[must_use = "should handle errors"]
#[derive(Debug)]
pub enum Error {
    /// The tag cannot be applied unless the others are also present.
    RequiresTags(Tag, Vec<Tag>),

    /// The two tags cannot be applied together, as they conflict.
    IncompatibleTags(Tag, Tag),

    /// The given tag is not registered in the [`Engine`].
    /// [`Engine`]: ./engine.html
    MissingTag(Tag),

    /// The given tag name could not be found.
    NoSuchTag(String),

    /// Unable to perform this operation due to lacking necessary access role.
    MissingRoles(Vec<Role>),

    /// For uncommon error cases.
    Other(&'static str),
}

impl StdError for Error {
    fn description(&self) -> &str {
        use self::Error::*;

        match *self {
            RequiresTags(_, _) => "Tag missing requirements",
            IncompatibleTags(_, _) => "Tags conflict",
            MissingTag(_) => "Tag not found in Engine",
            NoSuchTag(_) => "No tag with that name",
            MissingRoles(_) => "Cannot apply tags without roles",
            Other(msg) => msg,
        }
    }

    fn source(&self) -> Option<&(StdError + 'static)> {
        None
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        write!(f, "{}: ", StdError::description(self))?;

        match *self {
            RequiresTags(ref tag, ref needed) => {
                write!(f, "{} needs ", tag)?;
                write_items(f, needed)?;
                Ok(())
            }
            MissingRoles(ref roles) => {
                write!(f, "at least one of ")?;
                write_items(f, roles)?;
                Ok(())
            }
            IncompatibleTags(ref first, ref second) => write!(f, "{} and {}", first, second),
            MissingTag(ref tag) => write!(f, "{}", tag),
            NoSuchTag(ref name) => write!(f, "{}", name),
            Other(_) => Ok(()),
        }
    }
}

fn write_items<D: Display>(f: &mut fmt::Formatter, items: &[D]) -> fmt::Result {
    for (i, item) in items.iter().enumerate() {
        let comma = if i < items.len() - 1 { ", " } else { "" };
        write!(f, "{}{}", item, comma)?;
    }

    Ok(())
}
