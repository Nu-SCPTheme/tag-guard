/*
 * test/exists.rs
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
fn test_has_tags() {
    let engine = setup();

    macro_rules! check {
        ($tags:expr, $tag:expr, $result:expr) => (
            let tag = Tag::new($tag);
            let result = engine.check_tag(&tag, &$tags).unwrap();
            assert_eq!(
                result,
                $result,
                "Expected {:?} be {}in {:?}",
                tag,
                if $result { "" } else { "not " },
                $tags,
            );
        )
    }

    let tags = [
        Tag::new("scp"),
        Tag::new("euclid"),
        Tag::new("ontokinetic"),
        Tag::new("humanoid"),
    ];

    // Check tags exist
    check!(tags, "scp", true);
    check!(tags, "euclid", true);
    check!(tags, "ontokinetic", true);
    check!(tags, "humanoid", true);

    check!(tags, "tale", false);
    check!(tags, "keter", false);
    check!(tags, "admin", false);

    // Check groups exist
    check!(tags, "primary", true);
    check!(tags, "attribute", true);

    check!(tags, "licensing", false);
}

#[test]
fn test_count_tags() {
    let engine = setup();

    macro_rules! check {
        ($tags:expr, $tag:expr, $count:expr) => (
            let tag = Tag::new($tag);
            let count = engine.count_tag(&tag, &$tags).unwrap();
            assert_eq!(count, $count);
        )
    }

    let tags = [
        Tag::new("scp"),
        Tag::new("tale"),
        Tag::new("_image"),
        Tag::new("_cc"),
        Tag::new("amorphous"),
        Tag::new("electronic"),
        Tag::new("ontokinetic"),
        Tag::new("humanoid"),
    ];

    // Check tag counts
    check!(tags, "scp", 1);
    check!(tags, "tale", 1);
    check!(tags, "humanoid", 1);
    check!(tags, "co-authored", 0);
    check!(tags, "admin", 0);
    check!(tags, "cliche2019", 0);

    // Check group counts
    check!(tags, "primary", 2);
    check!(tags, "licensing", 2);
    check!(tags, "attribute", 4);
    check!(tags, "contests", 0);
}
