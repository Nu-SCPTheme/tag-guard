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

#[derive(Debug, Default)]
pub struct Engine {
    specs: HashMap<Tag, TagSpec>,
    tags: HashSet<Tag>,
    roles: HashSet<Role>,
}

impl Engine {
    pub fn add_tag<I: Into<String>>(&mut self, name: I, spec: TemplateTagSpec) -> Tag {
        let tag = Tag::new(name);
        let spec = TagSpec::from_template(&tag, spec);

        self.specs.insert(Tag::clone(&tag), spec);
        self.tags.insert(Tag::clone(&tag));
        tag
    }

    pub fn delete_tag(&mut self, tag: &Tag) {
        self.specs.remove(tag);
        self.tags.remove(tag);

        for (_, spec) in &mut self.specs {
            spec.required_tags.retain(|t| t != tag);
            spec.conflicting_tags.retain(|t| t != tag);
        }
    }

    pub fn add_group<I: Into<String>>(&mut self, name: I) -> Tag {
        let group = Tag::new(name);
        self.tags.insert(Tag::clone(&group));
        group
    }

    pub fn delete_group(&mut self, group: &Tag) {
        self.tags.remove(group);

        for (_, spec) in &mut self.specs {
            spec.groups.retain(|g| g != group);
        }
    }

    pub fn add_role<I: Into<String>>(&mut self, name: I) -> Role {
        let role = Role::new(name);
        self.roles.insert(Role::clone(&role));
        role
    }

    pub fn delete_role(&mut self, role: &Role) {
        self.roles.remove(role);

        for (_, spec) in &mut self.specs {
            spec.needed_roles.retain(|r| r != role);
        }
    }

    #[inline]
    pub fn get_tags(&self) -> &HashSet<Tag> {
        &self.tags
    }

    /// Gets a read-only set of all registered [`TagSpec`]s.
    /// [`TagSpec`]: ./tag/spec.html
    #[inline]
    pub fn get_specs(&self) -> &HashMap<Tag, TagSpec> {
        &self.specs
    }

    /// Gets a read-only set of all registered [`Role`]s.
    /// [`Role`]: ./tag/role.html
    #[inline]
    pub fn get_roles(&self) -> &HashSet<Role> {
        &self.roles
    }

    /// Gets the specification associated with a [`Tag`].
    /// [`Tag`]: ./tag/tag.html
    pub fn get_spec(&self, tag: &Tag) -> Result<&TagSpec> {
        match self.specs.get(tag) {
            Some(spec) => Ok(spec),
            None => Err(Error::MissingTag(Tag::clone(tag))),
        }
    }

    /// Gets the specification associated a [`Tag`] as `&mut`.
    /// [`Tag`]: ./tag/tag.html
    pub fn get_spec_mut(&mut self, tag: &Tag) -> Result<&mut TagSpec> {
        match self.specs.get_mut(tag) {
            Some(spec) => Ok(spec),
            None => Err(Error::MissingTag(Tag::clone(tag))),
        }
    }

    /// Determines if a [`Tag`] with the given name is registered.
    /// [`Tag`]: ./tag/tag.html
    pub fn has_tag<B: Borrow<str>>(&self, name: B) -> bool {
        let name = name.borrow();

        self.tags.get(name).is_some()
    }

    /// Gets the [`Tag`] with the given name.
    /// [`Tag`]: ./tag/tag.html
    pub fn get_tag<B: Borrow<str>>(&self, name: B) -> Result<Tag> {
        let name = name.borrow();

        match self.tags.get(name) {
            Some(tag) => Ok(Tag::clone(tag)),
            None => Err(Error::NoSuchTag(str!(name))),
        }
    }

    /// Determines if the given [`Tag`] is present as a group.
    /// [`Tag`]: ./tag/tag.html
    pub fn is_group(&self, tag: &Tag) -> bool {
        self.tags.contains(tag) && self.specs.get(tag).is_none()
    }

    /// Determines if a [`Role`] with the given name is registered.
    /// [`Role`]: ./tag/role.html
    pub fn has_role<B: Borrow<str>>(&self, name: B) -> bool {
        let name = name.borrow();

        self.roles.get(name).is_some()
    }

    /// Gets the [`Role`] with the given name.
    /// [`Role`]: ./tag/role.html
    pub fn get_role<B: Borrow<str>>(&self, name: B) -> Result<Role> {
        let name = name.borrow();

        match self.roles.get(name) {
            Some(role) => Ok(Role::clone(role)),
            None => Err(Error::NoSuchRole(str!(name))),
        }
    }

    /// Determines if the given tag/group is present in the list.
    pub fn check_tag(&self, some_tag: &Tag, tags: &[Tag]) -> Result<bool> {
        if self.is_group(some_tag) {
            for tag in tags {
                if self.get_spec(tag)?.groups.contains(some_tag) {
                    return Ok(true);
                }
            }

            Ok(false)
        } else {
            Ok(tags.contains(some_tag))
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
        added_tags: &[Tag],
        removed_tags: &[Tag],
        tags: &[Tag],
        roles: &[Role],
    ) -> Result<()> {
        for tag in tags {
            let spec = self.get_spec(&tag)?;
            spec.check_tag_changes(self, added_tags, removed_tags, tags, roles)?;
        }

        Ok(())
    }
}
