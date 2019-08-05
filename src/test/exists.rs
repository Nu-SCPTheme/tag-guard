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
            let result = engine.check_tag(&tag, &$tags, &[]).unwrap();
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

    let tags = [Tag::new("scp"), Tag::new("euclid"), Tag::new("ontokinetic"), Tag::new("humanoid")];

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
