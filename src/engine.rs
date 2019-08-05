/*
 * engine.rs
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
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};

/// A representation of a complete configuration of tags, groups, and roles.
///
/// Contains methods to determine if a tagset is valid given the rules in this
/// `Engine` or getting other configured information.
/// Also provides means of modifying the `Engine`'s internal state.
///
/// The consumer is responsible for ensuring that referenced tags, groups, and
/// roles are registered before being used.
#[derive(Debug, Default)]
pub struct Engine {
    specs: HashMap<Tag, TagSpec>,
    tags: HashSet<Tag>,
    roles: HashSet<Role>,
}

impl Engine {
    /// Registers a tag in the `Engine`, with the given [`TemplateTagSpec`].
    ///
    /// [`TemplateTagSpec`]: ./struct.TemplateTagSpec.html
    pub fn add_tag<I: Into<String>>(&mut self, name: I, spec: TemplateTagSpec) -> Tag {
        let tag = Tag::new(name);
        let spec = TagSpec::from_template(&tag, spec);

        self.specs.insert(Tag::clone(&tag), spec);
        self.tags.insert(Tag::clone(&tag));
        tag
    }

    /// Unregisters a tag from the `Engine`. Does nothing if already deleted.
    pub fn delete_tag(&mut self, tag: &Tag) {
        self.specs.remove(tag);
        self.tags.remove(tag);

        for (_, spec) in &mut self.specs {
            spec.required_tags.retain(|t| t != tag);
            spec.conflicting_tags.retain(|t| t != tag);
        }
    }

    /// Registers a tag group in the `Engine`.
    pub fn add_group<I: Into<String>>(&mut self, name: I) -> Tag {
        let group = Tag::new(name);
        self.tags.insert(Tag::clone(&group));
        group
    }

    /// Unregisters a tag group from the `Engine`. Does nothing if already deleted.
    pub fn delete_group(&mut self, group: &Tag) {
        self.tags.remove(group);

        for (_, spec) in &mut self.specs {
            spec.groups.retain(|g| g != group);
        }
    }

    /// Registers a role in the `Engine`.
    pub fn add_role<I: Into<String>>(&mut self, name: I) -> Role {
        let role = Role::new(name);
        self.roles.insert(Role::clone(&role));
        role
    }

    /// Unregisters a role from the `Engine`. Does nothing if already deleted.
    pub fn delete_role(&mut self, role: &Role) {
        self.roles.remove(role);

        for (_, spec) in &mut self.specs {
            spec.needed_roles.retain(|r| r != role);
        }
    }

    /// Gets a [`HashSet`] of all tags and tag groups in the `Engine`.
    ///
    /// [`HashSet`]: https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html
    #[inline]
    pub fn get_tags(&self) -> &HashSet<Tag> {
        &self.tags
    }

    /// Gets a read-only set of all registered [`TagSpec`]s.
    /// This will not include specification data for tag groups, only proper tags.
    ///
    /// [`TagSpec`]: ./tag/spec.html
    #[inline]
    pub fn get_specs(&self) -> &HashMap<Tag, TagSpec> {
        &self.specs
    }

    /// Gets a read-only set of all registered [`Role`]s.
    ///
    /// [`Role`]: ./tag/role.html
    #[inline]
    pub fn get_roles(&self) -> &HashSet<Role> {
        &self.roles
    }

    /// Gets the specification associated with a [`Tag`].
    ///
    /// [`Tag`]: ./tag/tag.html
    pub fn get_spec(&self, tag: &Tag) -> Result<&TagSpec> {
        match self.specs.get(tag) {
            Some(spec) => Ok(spec),
            None => Err(Error::MissingTag(Tag::clone(tag))),
        }
    }

    /// Gets the specification associated a [`Tag`] as `&mut`.
    ///
    /// [`Tag`]: ./tag/tag.html
    pub fn get_spec_mut(&mut self, tag: &Tag) -> Result<&mut TagSpec> {
        match self.specs.get_mut(tag) {
            Some(spec) => Ok(spec),
            None => Err(Error::MissingTag(Tag::clone(tag))),
        }
    }

    /// Determines if a [`Tag`] with the given name is registered.
    ///
    /// [`Tag`]: ./tag/tag.html
    pub fn has_tag<B: Borrow<str>>(&self, name: B) -> bool {
        let name = name.borrow();

        self.tags.get(name).is_some()
    }

    /// Gets the [`Tag`] with the given name.
    ///
    /// [`Tag`]: ./tag/tag.html
    pub fn get_tag<B: Borrow<str>>(&self, name: B) -> Result<Tag> {
        let name = name.borrow();

        match self.tags.get(name) {
            Some(tag) => Ok(Tag::clone(tag)),
            None => Err(Error::NoSuchTag(str!(name))),
        }
    }

    /// Determines if the given [`Tag`] is present as a group.
    ///
    /// [`Tag`]: ./tag/tag.html
    pub fn is_group(&self, tag: &Tag) -> bool {
        self.tags.contains(tag) && self.specs.get(tag).is_none()
    }

    /// Determines if a [`Role`] with the given name is registered.
    ///
    /// [`Role`]: ./tag/role.html
    pub fn has_role<B: Borrow<str>>(&self, name: B) -> bool {
        let name = name.borrow();

        self.roles.get(name).is_some()
    }

    /// Gets the [`Role`] with the given name.
    ///
    /// [`Role`]: ./tag/role.html
    pub fn get_role<B: Borrow<str>>(&self, name: B) -> Result<Role> {
        let name = name.borrow();

        match self.roles.get(name) {
            Some(role) => Ok(Role::clone(role)),
            None => Err(Error::NoSuchRole(str!(name))),
        }
    }

    /// Count the number of tags in the list that are in the given group.
    /// For tags this will return 0 or 1.
    pub fn count_tag(&self, check: &Tag, tags: &[Tag]) -> Result<usize> {
        let mut count = 0;

        for tag in tags {
            if tag == check || self.get_spec(tag)?.groups.contains(check) {
                count += 1;
            }
        }

        Ok(count)
    }

    /// Determines if the given tag/group is present in the list.
    pub fn check_tag(&self, check: &Tag, tags: &[Tag]) -> Result<bool> {
        if self.is_group(check) {
            self.count_tag(check, tags).map(|count| count > 0)
        } else {
            Ok(tags.contains(check))
        }
    }

    /// Validates the given list of tags against the engine's tag policies.
    pub fn check_tags(&self, tags: &[Tag]) -> Result<()> {
        for tag in tags {
            let spec = self.get_spec(&tag)?;
            spec.check_tags(self, tags)?;
        }

        Ok(())
    }

    /// Validates the given list of tag changes against the engine's tag policies.
    pub fn check_tag_changes(
        &self,
        tags: &[Tag],
        added_tags: &[Tag],
        removed_tags: &[Tag],
        roles: &[Role],
    ) -> Result<()> {
        // Check for unregistered roles
        for role in roles {
            if !self.roles.contains(role) {
                let role = Role::clone(role);
                return Err(Error::MissingRole(role));
            }
        }

        // Check for tags that are both added and removed
        for tag in added_tags {
            if removed_tags.contains(tag) {
                return Err(Error::Other(
                    "Tag present in both added_tags and removed_tags",
                ));
            }
        }


        for tag in tags {
            let spec = self.get_spec(&tag)?;
            spec.check_tag_changes(self, tags, added_tags, removed_tags, roles)?;
        }

        Ok(())
    }
}
