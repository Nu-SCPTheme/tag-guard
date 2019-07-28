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

#[derive(Debug, Clone, Default)]
pub struct TemplateTagSpec {
    pub required_tags: Vec<Tag>,
    pub conflicting_tags: Vec<Tag>,
    pub needed_roles: Vec<Role>,
}

#[derive(Debug)]
pub struct TagSpec {
    tag: Tag,
    pub required_tags: Vec<Tag>,
    pub conflicting_tags: Vec<Tag>,
    pub needed_roles: Vec<Role>,
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
            needed_roles,
        } = spec;

        TagSpec {
            tag,
            required_tags,
            conflicting_tags,
            needed_roles,
        }
    }

    fn check_roles(&self, roles: &[Role]) -> Result<()> {
        for role in roles {
            if self.needed_roles.contains(role) {
                return Ok(());
            }
        }

        Err(Error::MissingRoles(self.needed_roles.clone()))
    }

    pub fn check_tags(&self, tags: &[Tag]) -> Result<()> {
        // Ensure all requirements are met
        for required in &self.required_tags {
            if !tags.contains(required) {
                let required_tags = self.required_tags.clone();
                return Err(Error::RequiresTags(self.tag(), required_tags));
            }
        }

        // Ensure no conflicts are present
        for conflicts in &self.conflicting_tags {
            if tags.contains(conflicts) {
                let conflicts = Tag::clone(conflicts);
                return Err(Error::IncompatibleTags(self.tag(), conflicts));
            }
        }

        Ok(())
    }

    pub fn check_tag_changes(
        &self,
        added_tags: &[Tag],
        removed_tags: &[Tag],
        tags: &[Tag],
        roles: &[Role],
    ) -> Result<()> {
        // Check for tags that are both added and removed
        for tag in added_tags {
            if removed_tags.contains(tag) {
                return Err(Error::Other(
                    "Tag present in both added_tags and removed_tags",
                ));
            }
        }

        // Ensure user has permission to change these tags
        self.check_roles(roles)?;

        // Local helper function
        let has_tag = |tag| {
            if !removed_tags.contains(tag) {
                tags.contains(tag) || added_tags.contains(tag)
            } else {
                false
            }
        };

        // Ensure all requirements are met
        for required in &self.required_tags {
            if !has_tag(required) {
                let required_tags = self.required_tags.clone();
                return Err(Error::RequiresTags(self.tag(), required_tags));
            }
        }

        // Ensure no conflicts are present
        for conflicts in &self.conflicting_tags {
            if has_tag(conflicts) {
                let conflicts = Tag::clone(conflicts);
                return Err(Error::IncompatibleTags(self.tag(), conflicts));
            }
        }

        Ok(())
    }
}
