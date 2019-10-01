/*
 * load.rs
 *
 * tag-guard - Configurable tag enforcement library
 * Copyright (c) 2019 Ammon Smith
 *
 * tag-guard is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

//! A module for easily serializing/deserializing [`Engine`] configurations.
//!
//! Will automatically handle registration of new tags, groups, and roles as
//! they are encountered through the specification. See the repository's
//! `misc/config.toml` file to get an example of how these configurations should
//! be structured.
//!
//! [`Engine`]: ./struct.Engine.html

use crate::prelude::*;
use crate::Result;
use std::collections::HashSet;
use std::mem;

/// A serializeable struct that can be applied to an [`Engine`].
///
/// [`Engine`]: ./struct.Engine.html
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Configuration {
    /// A declaration of all [`Role`]s.
    ///
    /// [`Role`]: ./struct.Role.html
    pub roles: Vec<String>,

    /// All tags, and their respective configuration.
    ///
    /// See also [`TemplateTagSpec`].
    ///
    /// [`TemplateTagSpec`]: ./struct.TemplateTagSpec.html
    pub tags: Vec<TagConfig>,
}

impl Configuration {
    /// Parses all of the fields in the config and applies them to the [`Engine`].
    ///
    /// [`Engine`]: ./struct.Engine.html
    pub fn apply(self, engine: &mut Engine) {
        let Configuration { roles, tags } = self;

        Self::apply_roles(roles, engine);
        Self::apply_tags(&tags, engine);
        Self::update_tags(tags, engine).expect("Unable to update tag data");
    }

    fn apply_roles(roles: Vec<String>, engine: &mut Engine) {
        let extant_roles = engine
            .get_roles()
            .iter()
            .map(Role::clone)
            .collect::<HashSet<Role>>();

        // Remove old roles
        for extant_role in &extant_roles {
            if !roles.contains(extant_role.as_ref()) {
                engine.delete_role(&extant_role);
            }
        }

        // Add new roles
        for role in roles {
            if !extant_roles.contains(&role) {
                engine.add_role(role);
            }
        }
    }

    fn apply_tags(tags: &[TagConfig], engine: &mut Engine) {
        let extant_tags = engine
            .get_tags()
            .iter()
            .map(Tag::clone)
            .collect::<HashSet<Tag>>();

        // Remove old tags
        for extant_tag in &extant_tags {
            let contains = tags.iter().any(|tag| tag.name == extant_tag.as_ref());
            if !contains {
                engine.delete_tag(&extant_tag);
            }
        }

        // Add new tags
        for tag in tags {
            if !extant_tags.contains(&tag.name) {
                engine.add_tag(&tag.name, TemplateTagSpec::default());
            }
        }
    }

    fn update_tags(configs: Vec<TagConfig>, engine: &mut Engine) -> Result<()> {
        for config in configs {
            let TagConfig {
                name,
                groups,
                roles,
                requires,
                conflicts_with,
            } = config;

            let current_tag = engine.get_tag(name)?;

            // Update required_tags
            {
                let requires = requires.unwrap_or_else(Vec::new);
                let mut required_tags = Vec::new();

                for name in requires {
                    let tag = engine.get_tag(name)?;
                    required_tags.push(tag);
                }

                let spec = engine.get_spec_mut(&current_tag)?;
                mem::replace(&mut spec.required_tags, required_tags);
            }

            // Update conflicting_tags
            {
                let conflicts_with = conflicts_with.unwrap_or_else(Vec::new);
                let mut conflicting_tags = Vec::new();

                for name in conflicts_with {
                    let tag = engine.get_tag(name)?;
                    conflicting_tags.push(tag);
                }

                let spec = engine.get_spec_mut(&current_tag)?;
                mem::replace(&mut spec.conflicting_tags, conflicting_tags);
            }

            // Update groups
            {
                let groups = groups.unwrap_or_else(Vec::new);
                let mut new_groups = Vec::new();

                for name in groups {
                    let group = match engine.get_tag(name.as_str()) {
                        Ok(group) => group,
                        Err(_) => engine.add_group(name),
                    };

                    new_groups.push(group);
                }

                let spec = engine.get_spec_mut(&current_tag)?;
                mem::replace(&mut spec.groups, new_groups);
            }

            // Update roles
            {
                let roles = roles.unwrap_or_else(Vec::new);
                let mut needed_roles = Vec::new();

                for name in roles {
                    let role = engine.get_role(name)?;
                    needed_roles.push(role);
                }

                let spec = engine.get_spec_mut(&current_tag)?;
                mem::replace(&mut spec.needed_roles, needed_roles);
            }
        }

        Ok(())
    }
}

/// Serializeable sub-structure used as part of [`Configuration`].
///
/// [`Configuration`]: ./struct.Configuration.html
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TagConfig {
    /// The name of the [`Tag`].
    ///
    /// [`Tag`]: ./struct.Tag.html
    pub name: String,

    /// All groups of which this [`Tag`] is a member.
    ///
    /// [`Tag`]: ./struct.Tag.html
    pub groups: Option<Vec<String>>,

    /// Any [`Role`]s needed to apply or remove this [`Tag`].
    ///
    /// [`Role`]: ./struct.Role.html
    /// [`Tag`]: ./struct.Tag.html
    pub roles: Option<Vec<String>>,

    /// Which other [`Tag`]s or tag groups this tag requires.
    ///
    /// [`Tag`]: ./struct.Tag.html
    pub requires: Option<Vec<String>>,

    /// Which other [`Tag`]s or tag groups this tag conflicts with.
    ///
    /// [`Tag`]: ./struct.Tag.html
    pub conflicts_with: Option<Vec<String>>,
}
