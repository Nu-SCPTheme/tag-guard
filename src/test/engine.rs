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

use super::prelude::*;

#[test]
fn add_remove_tags() {
    let mut engine = Engine::default();

    assert_eq!(engine.has_tag("test"), false);
    let tag = engine.add_tag("test", TemplateTagSpec::default());
    assert_eq!(engine.has_tag("test"), true);
    assert_eq!(engine.get_tag("test").unwrap(), tag);

    engine.delete_tag(&tag);
    assert_eq!(engine.has_tag("test"), false);
}

#[test]
fn add_remove_specs() {
    let mut engine = Engine::default();

    let mut apple = TemplateTagSpec::default();
    apple.required_tags.push(Tag::new("banana"));

    let mut banana = TemplateTagSpec::default();
    banana.needed_roles.push(Role::new("admin"));

    {
        assert_eq!(engine.has_tag("apple"), false);
        let tag = engine.add_tag("apple", apple);
        assert_eq!(engine.has_tag("apple"), true);

        let spec = engine.get_spec(&tag).unwrap();
        assert_eq!(spec.required_tags.len(), 1);
    }

    {
        assert_eq!(engine.has_tag("banana"), false);
        let tag = engine.add_tag("banana", banana);
        assert_eq!(engine.has_tag("apple"), true);
        assert_eq!(engine.has_tag("banana"), true);

        let spec = engine.get_spec(&tag).unwrap();
        assert_eq!(spec.needed_roles.len(), 1);
    }
}

#[test]
fn add_remove_groups() {
    let mut engine = Engine::default();

    assert_eq!(engine.has_tag("fruit"), false);
    let tag = engine.add_group("fruit");
    assert_eq!(engine.has_tag("fruit"), true);
    assert_eq!(engine.get_tag("fruit").unwrap(), tag);
    assert_eq!(engine.get_spec(&tag).is_err(), true);

    engine.delete_group(&tag);
    assert_eq!(engine.has_tag("fruit"), false);
}

#[test]
fn add_remove_roles() {
    let mut engine = Engine::default();

    assert_eq!(engine.has_role("admin"), false);
    let role = engine.add_role("admin");
    assert_eq!(engine.has_role("admin"), true);
    assert_eq!(engine.has_tag("admin"), false);

    engine.delete_role(&role);
    assert_eq!(engine.has_role("fruit"), false);
}
