/*
 * tag/spec.rs
 *
 * tag-guard - Configurable tag enforcement library
 * Copyright (c) 2019 Ammon Smith
 *
 * tag-guard is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

use crate::{Error, Result};
use super::{Role, Tag};

#[derive(Debug, Clone)]
pub struct TagSpec {
    pub required_tags: Vec<Tag>,
    pub conflicting_tags: Vec<Tag>,
    pub required_roles: Vec<Role>,
}

impl TagSpec {
    pub fn check_tags(&self, tag: &Tag, tags: &[Tag]) -> Result<()> {
        for required in &self.required_tags {
            if !tags.contains(required) {
                let tag = Tag::clone(tag);
                let required = self.required_tags.clone();

                return Err(Error::RequiresTags(tag, required));
            }
        }

        for conflicts in &self.conflicting_tags {
            if tags.contains(conflicts) {
                let tag = Tag::clone(tag);
                let conflicts = Tag::clone(conflicts);

                return Err(Error::IncompatibleTags(tag, conflicts));
            }
        }

        Ok(())
    }
}
