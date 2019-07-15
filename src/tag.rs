/*
 * tag.rs
 *
 * tag-guard - Configurable tag enforcement library
 * Copyright (c) 2019 Ammon Smith
 *
 * tag-guard is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

use std::collections::HashMap;
use std::ptr;

#[derive(Debug)]
pub struct TagPool {
    next_id: u64,
    names: HashMap<String, u64>, // TODO change to double-sided map
}

#[derive(Debug, Copy, Clone)]
pub struct Tag<'a> {
    pool: &'a TagPool,
    id: u64,
}

impl<'a> PartialEq for Tag<'a> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(&self.pool, &other.pool) && self.id == other.id
    }
}

impl<'a> Eq for Tag<'a> {}
