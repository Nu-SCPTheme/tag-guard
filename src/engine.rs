/*
 * engine.rs
 *
 * tag-guard - Configurable tag enforcement library
 * Copyright (c) 2019 Ammon Smith
 *
 * tag-guard is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

use crate::{Error, Result};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use super::prelude::*;

#[derive(Debug, Default)]
pub struct Engine {
    specs: HashMap<Tag, TagSpec>,
    tags: HashSet<Tag>,
}

impl Engine {
    pub fn add_tag(&mut self, _: ()) {
        unimplemented!()
    }

    pub fn get_spec(&self, tag: &Tag) -> Result<&TagSpec> {
        match self.specs.get(tag) {
            Some(spec) => Ok(spec),
            None => Err(Error::MissingTag(Tag::clone(tag))),
        }
    }

    pub fn get_tag<B: Borrow<str>>(&self, name: B) -> Result<Tag> {
        let name = name.borrow();

        match self.tags.get(name) {
            Some(tag) => Ok(Tag::clone(tag)),
            None => Err(Error::NoSuchTag(str!(name))),
        }
    }

    pub fn check_tags(&self, tags: &[Tag]) -> Result<()> {
        for tag in tags {
            let spec = self.get_spec(&tag)?;
            spec.check_tags(tag, tags)?;
        }

        Ok(())
    }
}
