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
use crate::prelude::*;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct Engine {
    specs: HashMap<Tag, TagSpec>,
    tags: HashSet<Tag>,
    groups: HashMap<Tag, HashSet<Tag>>, // TODO add groups
}

impl Engine {
    pub fn add_tag<S: AsRef<str>>(&mut self, name: S, spec: TemplateTagSpec) -> Tag {
        let tag = Tag::new(name);
        let spec = TagSpec::from_template(&tag, spec);

        self.specs.insert(Tag::clone(&tag), spec);
        self.tags.insert(Tag::clone(&tag));
        tag
    }

    pub fn delete_tag(&mut self, tag: &Tag) {
        self.specs.remove(tag);
        self.tags.remove(tag);
    }

    #[inline]
    pub fn get_all(&self) -> &HashMap<Tag, TagSpec> {
        &self.specs
    }

    pub fn get_spec(&self, tag: &Tag) -> Result<&TagSpec> {
        match self.specs.get(tag) {
            Some(spec) => Ok(spec),
            None => Err(Error::MissingTag(Tag::clone(tag))),
        }
    }

    pub fn get_spec_mut(&mut self, tag: &Tag) -> Result<&mut TagSpec> {
        match self.specs.get_mut(tag) {
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
            spec.check_tags(tags)?;
        }

        Ok(())
    }

    pub fn check_tag_changes(
        &self,
        added_tags: &[Tag],
        removed_tags: &[Tag],
        tags: &[Tag],
        roles: &[Role],
    ) -> Result<()> {
        for tag in tags {
            let spec = self.get_spec(&tag)?;
            spec.check_tag_changes(added_tags, removed_tags, tags, roles)?;
        }

        Ok(())
    }
}
