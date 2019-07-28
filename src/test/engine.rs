/*
 * test/engine.rs
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

#[test]
fn add_remove_tags() {
    let mut engine = Engine::default();

    assert_eq!(engine.has_tag("test"), false);
    let tag = engine.add_tag("test", TemplateTagSpec::default());
    assert_eq!(engine.has_tag("test"), true);
    assert_eq!(engine.get_tag("test").unwrap(), tag);
}
