/*
 * test/mod.rs
 *
 * tag-guard - Configurable tag enforcement library
 * Copyright (c) 2019 Ammon Smith
 *
 * tag-guard is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

mod changes;
mod check;
mod engine;
mod exists;
mod setup;

mod prelude {
    pub use crate::prelude::*;
    pub use super::setup::setup;
}
