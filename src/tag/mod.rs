/*
 * tag/mod.rs
 *
 * tag-guard - Configurable tag enforcement library
 * Copyright (c) 2019 Ammon Smith
 *
 * tag-guard is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

mod object;
mod role;
mod spec;

pub use self::object::Tag;
pub use self::role::Role;
pub use self::spec::{TagSpec, TemplateTagSpec};
