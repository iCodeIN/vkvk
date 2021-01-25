#![allow(unused_imports)]

use magnesium::{skip_comments, skip_empty_text_elements, ElementIterator, TagAttribute, TagAttributeIterator, XmlElement, XmlElement::*};
use std::collections::HashMap;
use tinyvec::TinyVec;

mod burners;
use burners::*;

mod collectors;
use collectors::*;

mod data_types;
use data_types::*;

type AttrList<'s> = TinyVec<[TagAttribute<'s>; 4]>;

// TODO: provide c_int
// TODO: defines

fn main() {
  let filename = std::env::args().skip(1).next().unwrap();
  let file_string = std::fs::read_to_string(filename).expect("couldn't open vk.xml");
  let mut elem_iter = ElementIterator::new(&file_string).filter_map(skip_comments).filter_map(skip_empty_text_elements);
  assert_eq!(elem_iter.next(), Some(StartTag { name: "registry", attrs: "" }));
  collect_registry(&mut elem_iter);
}

#[allow(unused)]
pub fn collect_registry<'s>(elem_iter: &mut impl Iterator<Item = XmlElement<'s>>) {
  // TODO
  let mut definitions = Vec::new();
  loop {
    match elem_iter.next().unwrap() {
      EndTag { name: "registry" } => return,
      StartTag { name: "comment", attrs: "" } => burn_comment(elem_iter),
      StartTag { name: "platforms", attrs } => burn_platforms(elem_iter, attrs),
      StartTag { name: "tags", attrs } => burn_tags(elem_iter, attrs),
      StartTag { name: "types", attrs } => definitions.extend(collect_types(elem_iter, attrs)),
      StartTag { name: "enums", attrs } => definitions.extend(collect_enums(elem_iter, attrs)),
      StartTag { name: "commands", .. } => collect_commands(elem_iter),
      StartTag { name: "feature", .. } => collect_feature(elem_iter),
      StartTag { name: "extensions", .. } => collect_extensions(elem_iter),
      StartTag { name: "spirvextensions", attrs } => burn_spirvextensions(elem_iter, attrs),
      StartTag { name: "spirvcapabilities", attrs } => burn_spirvcapabilities(elem_iter, attrs),
      other => panic!("{:?}", other),
    }
  }
}

#[allow(unused)]
pub fn collect_commands<'s>(elem_iter: &mut impl Iterator<Item = XmlElement<'s>>) {
  // TODO
  loop {
    match elem_iter.next().unwrap() {
      EndTag { name: "commands" } => return,
      StartTag { name: "comment", .. } => burn_comment(elem_iter),
      StartTag { name: "command", .. } => loop {
        match elem_iter.next().unwrap() {
          EndTag { name: "command" } => break,
          _ => (),
        }
      },
      EmptyTag { name: "command", .. } => (),
      other => panic!("{:?}", other),
    }
  }
}

#[allow(unused)]
pub fn collect_feature<'s>(elem_iter: &mut impl Iterator<Item = XmlElement<'s>>) {
  // TODO
  loop {
    match elem_iter.next().unwrap() {
      EndTag { name: "feature" } => return,
      StartTag { name: "comment", .. } => burn_comment(elem_iter),
      StartTag { name: "require", .. } => loop {
        match elem_iter.next().unwrap() {
          EndTag { name: "require" } => break,
          _ => (),
        }
      },
      EmptyTag { name: "require", .. } => (),
      other => panic!("{:?}", other),
    }
  }
}

#[allow(unused)]
pub fn collect_extensions<'s>(elem_iter: &mut impl Iterator<Item = XmlElement<'s>>) {
  // TODO
  loop {
    match elem_iter.next().unwrap() {
      EndTag { name: "extensions" } => return,
      StartTag { name: "comment", .. } => burn_comment(elem_iter),
      StartTag { name: "extension", .. } => loop {
        match elem_iter.next().unwrap() {
          EndTag { name: "extension" } => break,
          _ => (),
        }
      },
      EmptyTag { name: "extension", .. } => (),
      other => panic!("{:?}", other),
    }
  }
}
