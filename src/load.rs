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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Configuration {
    roles: Vec<String>,
    tags: Vec<TagConfig>,
}

impl Into<Engine> for Configuration {
    fn into(self) -> Engine {
        let mut engine = Engine::default();

        unimplemented!()
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
