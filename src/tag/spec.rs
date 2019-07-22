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
pub struct TemplateTagSpec {
    pub required_tags: Vec<Tag>,
    pub conflicting_tags: Vec<Tag>,
    pub required_roles: Vec<Role>,
}

#[derive(Debug)]
pub struct TagSpec {
    tag: Tag,
    pub required_tags: Vec<Tag>,
    pub conflicting_tags: Vec<Tag>,
    pub required_roles: Vec<Role>,
}

impl TagSpec {
    #[inline]
    pub fn tag(&self) -> Tag {
        Tag::clone(&self.tag)
    }

    #[inline]
    pub fn from_template(tag: &Tag, spec: TemplateTagSpec) -> Self {
        let tag = Tag::clone(tag);
        let TemplateTagSpec {
            required_tags,
            conflicting_tags,
            required_roles,
        } = spec;

        TagSpec {
            tag,
            required_tags,
            conflicting_tags,
            required_roles,
        }
    }

    pub fn check_tags(&self, tags: &[Tag]) -> Result<()> {
        for required in &self.required_tags {
            if !tags.contains(required) {
                let required_tags = self.required_tags.clone();
                return Err(Error::RequiresTags(self.tag(), required_tags));
            }
        }

        for conflicts in &self.conflicting_tags {
            if tags.contains(conflicts) {
                let conflicts = Tag::clone(conflicts);
                return Err(Error::IncompatibleTags(self.tag(), conflicts));
            }
        }

        Ok(())
    }

    pub fn check_tag_additions(
        &self,
        new_tags: &[Tag],
        tags: &[Tag],
        roles: &[Role],
    ) -> Result<()> {
        for role in &self.required_roles {
            if !roles.contains(role) {
                return Err(Error::MissingRole(Role::clone(role)));
            }
        }

        for required in &self.required_tags {
            if !(tags.contains(required) || new_tags.contains(required)) {
                let required_tags = self.required_tags.clone();
                return Err(Error::RequiresTags(self.tag(), required_tags));
            }
        }

        for conflicts in &self.conflicting_tags {
            if tags.contains(conflicts) || new_tags.contains(conflicts) {
                let conflicts = Tag::clone(conflicts);
                return Err(Error::IncompatibleTags(self.tag(), conflicts));
            }
        }

        Ok(())
    }
}
