/*
 * test/check.rs
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
fn test_good_tags() {
    let engine = setup();

    macro_rules! check {
        ($check_tags:expr) => (
            engine.check_tags(&$check_tags).unwrap()
        )
    }

    check!([
        Tag::new("scp"),
        Tag::new("ontokinetic"),
        Tag::new("humanoid"),
        Tag::new("doomsday2018")
    ]);
    check!([Tag::new("scp"), Tag::new("amourphous"), Tag::new("_image")]);
    check!([Tag::new("tale"), Tag::new("_cc")]);
    check!([Tag::new("hub"), Tag::new("doomsday2018")])
}

#[test]
fn test_no_tags() {
    let engine = setup();

    macro_rules! check {
        ($check_tags:expr, $err_tag:expr) => (
            match engine.check_tags(&$check_tags).unwrap_err() {
                Error::MissingTag(tag) => assert_eq!(tag, Tag::new($err_tag)),
                error => panic!("Error wasn't MissingTag: {:?}", error),
            }
        )
    }

    check!(
        [Tag::new("scp"), Tag::new("amorphous"), Tag::new("sliver")],
        "sliver"
    );

    check!(
        [Tag::new("tale"), Tag::new("_iamge")],
        "_iamge"
    )
}

#[test]
fn test_conflicts() {
    let engine = setup();

    macro_rules! check {
        ($check_tags:expr, $err_tags:expr) => (
            match engine.check_tags(&$check_tags).unwrap_err() {
                Error::RequiresTags(_, tags) => assert_eq!(tags, $err_tags),
                error => panic!("Error wasn't RequiresTags: {:?}", error),
            }
        )
    }

    check!(
        [Tag::new("ontokinetic"), Tag::new("humanoid")],
        [Tag::new("primary")]
    );

    check!([Tag::new("scp"), Tag::new("tale")], [Tag::new("primary")]);

    check!([Tag::new("_image"), Tag::new("_cc")], [Tag::new("_cc")]);
}
