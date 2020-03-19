#![allow(unused)]

use core::marker::PhantomData;

use tinyvec::ArrayVec;

pub mod xml;
use xml::{XmlElement::*, *};

pub struct Registry<'s> {
  _marker: core::marker::PhantomData<&'s str>,
}
impl<'s> Registry<'s> {
  /// Construct a registry from an [`XmlIterator`](XmlIterator).
  ///
  /// The iterator should have _just_ processed the `registry` opening tag,
  /// without having yet processed any other elements.
  pub fn from_xml(iter: &mut XmlIterator<'s>) -> Registry<'s> {
    'registry_loop: loop {
      match iter.next() {
        Some(EndTag { name: "registry", .. }) => break 'registry_loop,
        Some(StartTag { name: "comment", attrs }) => {
          Self::do_comment(iter, attrs)
        }
        Some(StartTag { name: "platforms", attrs }) => {
          Self::do_platforms(iter, attrs)
        }
        Some(StartTag { name: "tags", attrs }) => {
          Self::do_vendor_tags(iter, attrs)
        }
        Some(StartTag { name: "types", attrs }) => Self::do_types(iter, attrs),
        Some(StartTag { name: "enums", attrs }) => Self::do_enums(iter, attrs),
        Some(StartTag { name: "commands", attrs }) => {
          Self::do_commands(iter, attrs)
        }
        Some(StartTag { name: "feature", attrs }) => {
          Self::do_feature(iter, attrs)
        }
        Some(StartTag { name: "extensions", attrs }) => {
          Self::do_extensions(iter, attrs)
        }
        other => panic!("Registry::from_xml> unknown {:?}", other),
      }
    }
    Self { _marker: PhantomData }
  }

  /// Consume a comment block's text and end.
  pub fn do_comment(iter: &mut XmlIterator<'s>, attrs: &'s str) {
    loop {
      match iter.next() {
        Some(EndTag { name: "comment" }) => return,
        Some(Text(t)) => (),
        other => panic!("Registry::do_comment> unknown {:?}", other),
      }
    }
  }

  /// Consume the platform names. Not very important.
  pub fn do_platforms(iter: &mut XmlIterator<'s>, attrs: &'s str) {
    loop {
      match iter.next() {
        Some(EndTag { name: "platforms" }) => return,
        other => (),
      }
    }
  }

  /// Consume vulkan vendor/author tags for extensions and layers. Not very
  /// important.
  pub fn do_vendor_tags(iter: &mut XmlIterator<'s>, attrs: &'s str) {
    loop {
      match iter.next() {
        Some(EndTag { name: "tags" }) => return,
        other => (),
      }
    }
  }

  /// Consume all the vulkan type definitions. Critical.
  pub fn do_types(iter: &mut XmlIterator<'s>, attrs: &'s str) {
    // TODO: process all the types.
    loop {
      match iter.next() {
        Some(EndTag { name: "types" }) => return,
        other => (),
      }
    }
  }

  /// Consume vulkan enumerations / bitflags. Critical.
  pub fn do_enums(iter: &mut XmlIterator<'s>, attrs: &'s str) {
    // TODO: enum and bitflag stuff
    loop {
      match iter.next() {
        Some(EndTag { name: "enums" }) => return,
        other => (),
      }
    }
  }

  /// Consume vulkan function calls. Critical.
  pub fn do_commands(iter: &mut XmlIterator<'s>, attrs: &'s str) {
    // TODO: function calls.
    loop {
      match iter.next() {
        Some(EndTag { name: "commands" }) => return,
        other => (),
      }
    }
  }

  /// Consume vulkan API level descriptions. Critical.
  pub fn do_feature(iter: &mut XmlIterator<'s>, attrs: &'s str) {
    // TODO: api features.
    loop {
      match iter.next() {
        Some(EndTag { name: "feature" }) => return,
        other => (),
      }
    }
  }

  /// Consume vulkan extension info. Critical.
  pub fn do_extensions(iter: &mut XmlIterator<'s>, attrs: &'s str) {
    // TODO: extension info.
    loop {
      match iter.next() {
        Some(EndTag { name: "extensions" }) => return,
        other => (),
      }
    }
  }
}
