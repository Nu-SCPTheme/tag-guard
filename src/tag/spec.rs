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
use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct TemplateTagSpec {
    pub required_tags: Vec<Tag>,
    pub conflicting_tags: Vec<Tag>,
    pub needed_roles: Vec<Role>,
    pub groups: Vec<Tag>,
}

#[derive(Debug)]
pub struct TagSpec {
    tag: Tag,
    pub required_tags: Vec<Tag>,
    pub conflicting_tags: Vec<Tag>,
    pub needed_roles: Vec<Role>,
    pub groups: Vec<Tag>,
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
            groups,
        } = spec;

        TagSpec {
            tag,
            required_tags,
            conflicting_tags,
            needed_roles,
            groups,
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

    #[inline]
    pub fn check_tags(&self, engine: &Engine, tags: &[Tag]) -> Result<()> {
        self.check_tag_changes(engine, &[], &[], tags, &[])
    }

    pub fn check_tag_changes(
        &self,
        engine: &Engine,
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

        // Check if this tag was changed
        if added_tags.contains(&self.tag) || removed_tags.contains(&self.tag) {
            // If so, ensure user has permission to change this tag
            self.check_roles(roles)?;
        }

        // Local helper function
        let count_tags = |tag| -> Result<usize> {
            // Tag isn't present
            if removed_tags.contains(tag) {
                return Ok(0);
            }

            // Check current and new tags
            let result = engine.count_tag(tag, tags)? + engine.count_tag(tag, added_tags)?;
            Ok(result)
        };

        // Ensure all requirements are met
        for required in &self.required_tags {
            if count_tags(required)? == 0 {
                let required_tags = self.required_tags.clone();
                return Err(Error::RequiresTags(self.tag(), required_tags));
            }
        }

        // Ensure no conflicts are present
        for conflicts in &self.conflicting_tags {
            let self_matches = engine.check_tag(&self.tag, tags)? || engine.check_tag(&self.tag, added_tags)?;

            if count_tags(conflicts)? <= usize::from(self_matches) {
                let conflicts = Tag::clone(conflicts);
                return Err(Error::IncompatibleTags(self.tag(), conflicts));
            }
        }

        Ok(())
    }
}
