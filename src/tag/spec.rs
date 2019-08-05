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

/// Input specification of a tag's requirements.
///
/// Consumed by an [`Engine`] to produce a [`TagSpec`] for use.
///
/// [`Engine`]: ./struct.Engine.html
/// [`TagSpec`]: ./struct.TagSpec.html
#[derive(Debug, Clone, Default)]
pub struct TemplateTagSpec {
    /// Which [`Tag`]s or tag groups must also be present for before this one may be applied.
    ///
    /// [`Tag`]: ./struct.Tag.html
    pub required_tags: Vec<Tag>,

    /// Which [`Tag`]s or tag groups may not be present if this one is to be applied.
    ///
    /// Note that specifying a tag group that this tag is a member of is not contradictory,
    /// as the [`Engine`] will instead ensure this tag is the only one of its group present.
    ///
    /// [`Engine`]: ./struct.Engine.html
    /// [`Tag`]: ./struct.Tag.html
    pub conflicting_tags: Vec<Tag>,

    /// A list of [`Role`]s which may add or remove this tag.
    ///
    /// To "lock" a tag, you can set this to either moderator-only, or create a specific role that
    /// nobody has access to.
    ///
    /// [`Role`]: ./struct.Role.html
    pub needed_roles: Vec<Role>,

    /// A list of [`Tag`] groups this tag is a member of.
    ///
    /// If a tag group is checked for membership, then the presence of this tag will cause it to
    /// return `true`. Likewise, tag groups can be used for requirements or conflicts in other
    /// tag rules.
    ///
    /// [`Tag`]: ./struct.Tag.html
    pub groups: Vec<Tag>,
}

/// A [`TemplateTagSpec`] that has been associated with a particular [`Tag`].
///
/// Stored in an [`Engine`] to determine behavior with other tags.
///
/// [`Engine`]: ./struct.Engine.html
/// [`Tag`]: ./struct.Tag.html
/// [`TemplateTagSpec`]: ./struct.TemplateTagSpec.html
#[derive(Debug)]
pub struct TagSpec {
    tag: Tag,

    /// Which [`Tag`]s or tag groups must also be present for before this one may be applied.
    ///
    /// [`Tag`]: ./struct.Tag.html
    pub required_tags: Vec<Tag>,

    /// Which [`Tag`]s or tag groups may not be present if this one is to be applied.
    ///
    /// Note that specifying a tag group that this tag is a member of is not contradictory,
    /// as the [`Engine`] will instead ensure this tag is the only one of its group present.
    ///
    /// [`Engine`]: ./struct.Engine.html
    /// [`Tag`]: ./struct.Tag.html
    pub conflicting_tags: Vec<Tag>,

    /// A list of [`Role`]s which may add or remove this tag.
    ///
    /// To "lock" a tag, you can set this to either moderator-only, or create a specific role that
    /// nobody has access to.
    ///
    /// [`Role`]: ./struct.Role.html
    pub needed_roles: Vec<Role>,

    /// A list of [`Tag`] groups this tag is a member of.
    ///
    /// If a tag group is checked for membership, then the presence of this tag will cause it to
    /// return `true`. Likewise, tag groups can be used for requirements or conflicts in other
    /// tag rules.
    ///
    /// [`Tag`]: ./struct.Tag.html
    pub groups: Vec<Tag>,
}

impl TagSpec {
    /// Returns the [`Tag`] associated with this specification.
    ///
    /// [`Tag`]: ./struct.Tag.html
    #[inline]
    pub fn tag(&self) -> Tag {
        Tag::clone(&self.tag)
    }

    /// Creates a new instance using the given [`Tag`] and [`TemplateTagSpec`].
    ///
    /// [`Tag`]: ./struct.Tag.html
    /// [`TemplateTagSpec`]: ./struct.TemplateTagSpec.html
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

    /// Checks that the given [`Tag`]s comply with the policy described in the [`Engine`].
    ///
    /// [`Engine`]: ./struct.Engine.html
    /// [`Tag`]: ./struct.Tag.html
    #[inline]
    pub fn check_tags(&self, engine: &Engine, tags: &[Tag]) -> Result<()> {
        self.check_tag_changes(engine, tags, &[], &[], &[])
    }

    /// Checks that the given [`Tag`]s changes with the policy described in the [`Engine`].
    ///
    /// Will consider the action of adding `added_tags` and removing `removed_tags` from the
    /// list of tags passed in as `tags`. The user performing the action has the roles specified
    /// in `roles`, and their ability to add or remove the new tags will be checked against that.
    ///
    /// `added_tags` and `removed_tags` should not have any members in common.
    ///
    /// [`Engine`]: ./struct.Engine.html
    /// [`Tag`]: ./struct.Tag.html
    pub fn check_tag_changes(
        &self,
        engine: &Engine,
        tags: &[Tag],
        added_tags: &[Tag],
        removed_tags: &[Tag],
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
            // Sees if the current tag matches the conflict requirement,
            // to avoid getting a false-positive on ourselves.

            let limit = if engine.is_group(conflicts) {
                let self_matches =
                    engine.check_tag(&self.tag, tags)? || engine.check_tag(&self.tag, added_tags)?;

                usize::from(self_matches)
            } else {
                0
            };

            if count_tags(conflicts)? > limit {
                let conflicts = Tag::clone(conflicts);
                return Err(Error::IncompatibleTags(self.tag(), conflicts));
            }
        }

        Ok(())
    }
}
