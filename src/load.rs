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

use crate::prelude::*;
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Configuration {
    roles: Vec<String>,
    tags: Vec<TagConfig>,
}

impl Configuration {
    pub fn apply(self, engine: &mut Engine) {
        let Configuration { roles, tags } = self;

        Self::apply_roles(roles, engine);
    }

    fn apply_roles(roles: Vec<String>, engine: &mut Engine) {
        let extant_roles = engine
            .get_roles()
            .iter()
            .map(Role::clone)
            .collect::<HashSet<Role>>();

        // Remove roles
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
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct TagConfig {
    name: String,
    groups: Option<Vec<String>>,
    roles: Option<Vec<String>>,
    requires: Option<Vec<String>>,
    conflicts_with: Option<Vec<String>>,
}
