/*
 * test/setup.rs
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

pub fn setup() -> Engine {
    let mut engine = Engine::default();

    engine.add_tag(
        "scp",
        TemplateTagSpec {
            conflicting_tags: vec![Tag::new("primary")],
            groups: vec![Tag::new("primary")],
            ..TemplateTagSpec::default()
        },
    );

    engine.add_tag(
        "tale",
        TemplateTagSpec {
            conflicting_tags: vec![Tag::new("primary")],
            groups: vec![Tag::new("primary")],
            ..TemplateTagSpec::default()
        },
    );

    engine.add_tag(
        "creepypasta",
        TemplateTagSpec {
            conflicting_tags: vec![],
            groups: vec![],
            required_tags: vec![Tag::new("tale")],
            ..TemplateTagSpec::default()
        },
    );

    engine.add_tag(
        "hub",
        TemplateTagSpec {
            conflicting_tags: vec![Tag::new("primary")],
            groups: vec![Tag::new("primary")],
            ..TemplateTagSpec::default()
        },
    );

    engine.add_tag(
        "safe",
        TemplateTagSpec {
            groups: vec![Tag::new("object-class")],
            required_tags: vec![Tag::new("scp")],
            ..TemplateTagSpec::default()
        },
    );

    engine.add_tag(
        "euclid",
        TemplateTagSpec {
            groups: vec![Tag::new("object-class")],
            required_tags: vec![Tag::new("scp")],
            ..TemplateTagSpec::default()
        },
    );

    engine.add_tag(
        "keter",
        TemplateTagSpec {
            groups: vec![Tag::new("object-class")],
            required_tags: vec![Tag::new("scp")],
            ..TemplateTagSpec::default()
        },
    );

    engine.add_tag(
        "thaumiel",
        TemplateTagSpec {
            groups: vec![Tag::new("object-class")],
            required_tags: vec![Tag::new("scp")],
            ..TemplateTagSpec::default()
        },
    );

    engine.add_tag(
        "esoteric-class",
        TemplateTagSpec {
            groups: vec![Tag::new("object-class")],
            required_tags: vec![Tag::new("scp")],
            ..TemplateTagSpec::default()
        },
    );

    engine.add_tag(
        "_image",
        TemplateTagSpec {
            conflicting_tags: vec![Tag::new("_cc")],
            groups: vec![Tag::new("licensing")],
            ..TemplateTagSpec::default()
        },
    );

    engine.add_tag(
        "_cc",
        TemplateTagSpec {
            conflicting_tags: vec![Tag::new("_image")],
            groups: vec![Tag::new("licensing")],
            needed_roles: vec![Role::new("licensing")],
            ..TemplateTagSpec::default()
        },
    );

    engine.add_tag(
        "amorphous",
        TemplateTagSpec {
            required_tags: vec![Tag::new("primary")],
            groups: vec![Tag::new("attribute")],
            ..TemplateTagSpec::default()
        },
    );

    engine.add_tag(
        "antimemetic",
        TemplateTagSpec {
            required_tags: vec![Tag::new("primary")],
            groups: vec![Tag::new("attribute")],
            ..TemplateTagSpec::default()
        },
    );

    engine.add_tag(
        "electronic",
        TemplateTagSpec {
            required_tags: vec![Tag::new("primary")],
            groups: vec![Tag::new("attribute")],
            ..TemplateTagSpec::default()
        },
    );

    engine.add_tag(
        "humanoid",
        TemplateTagSpec {
            required_tags: vec![Tag::new("primary")],
            groups: vec![Tag::new("attribute")],
            ..TemplateTagSpec::default()
        },
    );

    engine.add_tag(
        "ontokinetic",
        TemplateTagSpec {
            required_tags: vec![Tag::new("primary")],
            groups: vec![Tag::new("attribute")],
            ..TemplateTagSpec::default()
        },
    );

    engine.add_tag(
        "global-occult-coalition",
        TemplateTagSpec {
            groups: vec![Tag::new("goi")],
            ..TemplateTagSpec::default()
        }
    );

    engine.add_tag(
        "marshall-carter-and-dark",
        TemplateTagSpec {
            groups: vec![Tag::new("goi")],
            ..TemplateTagSpec::default()
        }
    );

    engine.add_tag(
        "serpents-hand",
        TemplateTagSpec {
            groups: vec![Tag::new("goi")],
            ..TemplateTagSpec::default()
        }
    );

    engine.add_tag("co-authored", TemplateTagSpec::default());

    engine.add_tag(
        "admin",
        TemplateTagSpec {
            required_tags: vec![Tag::new("primary")],
            needed_roles: vec![Role::new("admin")],
            ..TemplateTagSpec::default()
        },
    );

    engine.add_tag(
        "doomsday2018",
        TemplateTagSpec {
            conflicting_tags: vec![Tag::new("contests")],
            needed_roles: vec![Role::new("locked")],
            groups: vec![Tag::new("contests")],
            ..TemplateTagSpec::default()
        },
    );

    engine.add_tag(
        "cliche2019",
        TemplateTagSpec {
            conflicting_tags: vec![Tag::new("contests")],
            needed_roles: vec![Role::new("locked")],
            groups: vec![Tag::new("contests")],
            ..TemplateTagSpec::default()
        },
    );

    engine.add_group("attribute");
    engine.add_group("contests");
    engine.add_group("licensing");
    engine.add_group("primary");

    engine.add_role("admin");
    engine.add_role("moderator");
    engine.add_role("licensing");
    engine.add_role("member");
    engine.add_role("locked");

    engine
}
