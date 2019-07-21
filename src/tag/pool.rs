/*
 * tag/pool.rs
 *
 * tag-guard - Configurable tag enforcement library
 * Copyright (c) 2019 Ammon Smith
 *
 * tag-guard is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

use std::collections::{HashMap, HashSet};
use super::{Tag, TagSpec};

#[derive(Debug)]
pub struct TagPool {
    specs: HashMap<Tag, TagSpec>,
    tags: HashSet<Tag>,
}
