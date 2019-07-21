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

use super::prelude::*;

#[derive(Debug)]
pub struct Engine {
    pool: TagPool,
    required_tags: Vec<()>, // TODO use Tag type
    conflicting_tags: Vec<()>,
    required_roles: Vec<()>, // TODO use Role type
}

impl Engine {
    pub fn new(_: ()) -> Self {
        unimplemented!()
    }
}
