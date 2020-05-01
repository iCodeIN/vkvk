#![allow(unused_mut)]
#![allow(unused_imports)]

use std::collections::HashMap;

use magnesium::{XmlElement::*, *};

pub fn hashmap_from_attrs(attrs: &str) -> HashMap<String, String> {
  TagAttributeIterator::new(attrs)
    .map(|ta| (ta.key.to_string(), ta.value.to_string()))
    .collect()
}

pub fn grab_text_and_end_tag<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
  end_tag: &str,
) -> &'s str {
  let text = match iter.next().unwrap() {
    Text(t) => t,
    unknown => panic!("grab_text_and_end_tag::not text> {:?}", unknown),
  };
  match iter.next().unwrap() {
    EndTag { name } if name == end_tag => (),
    unknown => panic!("grab_text_and_end_tag::end tag> {:?}", unknown),
  }
  text
}

#[derive(Debug, Default, Clone)]
pub struct Registry {
  pub platforms: Platforms,
  pub tags: Tags,
  pub types: Types,
}

pub fn parse_registry<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
) -> Registry {
  let mut registry = Registry::default();
  loop {
    match iter.next().unwrap() {
      EndTag { name: "registry" } => return registry,
      StartTag { name: "comment", attrs: "" } => parse_comment(iter),
      StartTag { name: "platforms", attrs } => {
        registry.platforms = parse_platforms(iter, attrs);
      }
      StartTag { name: "tags", attrs } => {
        registry.tags = parse_tags(iter, attrs);
      }
      StartTag { name: "types", attrs } => {
        registry.types = parse_types(iter, attrs);
      }
      unknown => panic!("parse_registry> {:?}", unknown),
    }
  }
}

pub fn parse_comment<'s>(iter: &mut impl Iterator<Item = XmlElement<'s>>) {
  loop {
    match iter.next().unwrap() {
      EndTag { name: "comment" } => return,
      Text(_) => (),
      unknown => panic!("parse_comment> {:?}", unknown),
    }
  }
}

#[derive(Debug, Default, Clone)]
pub struct Platforms {
  pub attrs: HashMap<String, String>,
  pub platforms: Vec<Platform>,
}

#[derive(Debug, Default, Clone)]
pub struct Platform {
  pub name: String,
  pub comment: String,
  pub protect: String,
}

pub fn parse_platforms<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
  attrs: &'s str,
) -> Platforms {
  let mut platforms = Platforms::default();
  platforms.attrs = hashmap_from_attrs(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "platforms" } => return platforms,
      EmptyTag { name: "platform", attrs } => {
        let a = hashmap_from_attrs(attrs);
        assert_eq!(a.len(), 3);
        platforms.platforms.push(Platform {
          name: a["name"].to_string(),
          comment: a["comment"].to_string(),
          protect: a["protect"].to_string(),
        });
      }
      unknown => panic!("parse_platforms> {:?}", unknown),
    }
  }
}

#[derive(Debug, Default, Clone)]
pub struct Tags {
  pub attrs: HashMap<String, String>,
  pub tags: Vec<Tag>,
}

#[derive(Debug, Default, Clone)]
pub struct Tag {
  pub name: String,
  pub contact: String,
  pub author: String,
}

pub fn parse_tags<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
  attrs: &'s str,
) -> Tags {
  let mut tags = Tags::default();
  tags.attrs = hashmap_from_attrs(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "tags" } => return tags,
      EmptyTag { name: "tag", attrs } => {
        let a = hashmap_from_attrs(attrs);
        assert_eq!(a.len(), 3);
        tags.tags.push(Tag {
          name: a["name"].to_string(),
          contact: a["contact"].to_string(),
          author: a["author"].to_string(),
        });
      }
      unknown => panic!("parse_tags> {:?}", unknown),
    }
  }
}

#[derive(Debug, Default, Clone)]
pub struct Types {
  pub attrs: HashMap<String, String>,
  pub types: Vec<Type>,
}

pub fn parse_types<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
  attrs: &'s str,
) -> Types {
  let mut types = Types::default();
  types.attrs = hashmap_from_attrs(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "types" } => return types,
      StartTag { name: "comment", attrs: "" } => parse_comment(iter),
      StartTag { name: "type", attrs } => {
        types.types.push(parse_type_start(iter, attrs));
      }
      EmptyTag { name: "type", attrs } => {
        types.types.push(parse_type_empty(attrs));
      }
      unknown => panic!("parse_types> {:?}", unknown),
    }
  }
}

#[derive(Debug, Clone)]
pub enum Type {
  // TODO
}

/// Parse from a start tag's attrs and an iter into a Type
pub fn parse_type_start<'s>(
  iter: &mut impl Iterator<Item = XmlElement<'s>>,
  attrs: &'s str,
) -> Type {
  todo!()
}

/// Parse from an empty tag's attrs into a Type
pub fn parse_type_empty<'s>(attrs: &'s str) -> Type {
  todo!()
}
