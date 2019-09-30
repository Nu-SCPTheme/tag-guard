/*
 * test/changes.rs
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
fn test_good_changes() {
    let engine = setup();

    macro_rules! check {
        ($tags:expr, $added_tags:expr, $removed_tags:expr, $roles:expr) => {
            let result = engine.check_tag_changes($tags, $added_tags, $removed_tags, $roles);
            assert_eq!(result, Ok(()));
        };
    }

    // Tag additions
    check!(
        &[Tag::new("scp"), Tag::new("euclid"), Tag::new("humanoid")],
        &[Tag::new("ontokinetic")],
        &[],
        &[]
    );

    check!(
        &[
            Tag::new("scp"),
            Tag::new("amorphous"),
            Tag::new("electronic")
        ],
        &[Tag::new("safe")],
        &[],
        &[]
    );

    check!(
        &[Tag::new("tale"), Tag::new("marshall-carter-and-dark")],
        &[Tag::new("serpents-hand")],
        &[],
        &[]
    );

    check!(
        &[Tag::new("tale"), Tag::new("_image")],
        &[Tag::new("creepypasta")],
        &[],
        &[]
    );

    // Tag removals
    check!(
        &[
            Tag::new("scp"),
            Tag::new("esoteric-class"),
            Tag::new("antimemetic"),
            Tag::new("electronic")
        ],
        &[],
        &[Tag::new("electronic")],
        &[]
    );

    check!(
        &[Tag::new("tale"), Tag::new("serpents-hand")],
        &[],
        &[Tag::new("serpents-hand")],
        &[]
    );

    // Additions and removals
    check!(
        &[Tag::new("scp"), Tag::new("keter"), Tag::new("humanoid")],
        &[Tag::new("euclid")],
        &[Tag::new("keter")],
        &[]
    );

    check!(
        &[Tag::new("scp"), Tag::new("keter"), Tag::new("humanoid")],
        &[Tag::new("amorphous")],
        &[Tag::new("humanoid")],
        &[]
    );

    check!(
        &[Tag::new("hub"), Tag::new("global-occult-coalition")],
        &[Tag::new("tale")],
        &[Tag::new("hub")],
        &[]
    );

    check!(
        &[Tag::new("hub"), Tag::new("global-occult-coalition")],
        &[Tag::new("serpents-hand")],
        &[Tag::new("global-occult-coalition")],
        &[]
    );

    check!(
        &[Tag::new("tale"), Tag::new("_image")],
        &[Tag::new("_cc")],
        &[Tag::new("_image")],
        &[]
    );
}

#[test]
fn test_bad_changes() {
    let engine = setup();

    macro_rules! check {
        ($tags:expr, $added_tags:expr, $removed_tags:expr, $roles:expr, $error:expr) => {
            let result = engine.check_tag_changes($tags, $added_tags, $removed_tags, $roles);
            assert_eq!(result, Err($error));
        };
    }

    // Incompatible tags
    check!(
        &[Tag::new("scp"), Tag::new("euclid"), Tag::new("humanoid")],
        &[Tag::new("tale")],
        &[],
        &[],
        Error::IncompatibleTags(Tag::new("scp"), Tag::new("primary"))
    );

    check!(
        &[Tag::new("tale"), Tag::new("_image")],
        &[Tag::new("_cc")],
        &[],
        &[],
        Error::IncompatibleTags(Tag::new("_image"), Tag::new("_cc"))
    );

    // Requires tags
    check!(
        &[Tag::new("scp"), Tag::new("electronic")],
        &[],
        &[Tag::new("scp")],
        &[],
        Error::RequiresTags(Tag::new("electronic"), vec![Tag::new("primary")])
    );

    // Missing roles
    check!(
        &[Tag::new("scp")],
        &[Tag::new("doomsday2018")],
        &[],
        &[Role::new("member")],
        Error::MissingRoles(vec![Role::new("locked")])
    );

    check!(
        &[],
        &[Tag::new("admin"), Tag::new("hub")],
        &[],
        &[Role::new("member")],
        Error::MissingRoles(vec![Role::new("admin")])
    );
}

#[test]
fn test_invalid_tags() {
    let engine = setup();

    macro_rules! check {
        ($tags:expr, $added_tags:expr, $removed_tags:expr, $roles:expr, $error:expr) => {
            let result = engine.check_tag_changes($tags, $added_tags, $removed_tags, $roles);
            assert_eq!(result, Err($error));
        };
    }

    // No tag registered
    check!(
        &[Tag::new("scp"), Tag::new("keter"), Tag::new("keter")],
        &[Tag::new("badass")],
        &[],
        &[Role::new("member")],
        Error::MissingTag(Tag::new(str!("badass")))
    );

    // No such role
    check!(
        &[Tag::new("tale")],
        &[Tag::new("_image")],
        &[],
        &[Role::new("invalid-role")],
        Error::MissingRole(Role::new("invalid-role"))
    );
}

#[test]
fn test_misc() {
    let engine = setup();

    macro_rules! check {
        ($tags:expr, $added_tags:expr, $removed_tags:expr, $roles:expr) => {
            let result = engine.check_tag_changes($tags, $added_tags, $removed_tags, $roles);
            match result {
                Err(Error::Other(_)) => (),
                _ => panic!("Expected Error::Other, got {:#?}", result),
            }
        };
    }

    check!(&[], &[Tag::new("tale")], &[Tag::new("tale")], &[]);
}
