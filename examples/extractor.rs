#![allow(unused_mut)]
#![allow(unused_imports)]

use std::collections::HashMap;

use magnesium::{XmlElement::*, *};

fn hashmap_from_attrs(attrs: &str) -> HashMap<String, String> {
  TagAttributeIterator::new(attrs)
    .map(|ta| (ta.key.to_string(), ta.value.to_string()))
    .collect()
}

fn revert_xml_encoding(text: String) -> String {
  let mut out = String::with_capacity(text.as_bytes().len());
  let mut chars = text.chars();
  while let Some(c) = chars.next() {
    if c != '&' {
      out.push(c);
    } else {
      match chars.next().unwrap() {
        'l' => {
          assert_eq!(chars.next().unwrap(), 't');
          assert_eq!(chars.next().unwrap(), ';');
          out.push('<');
        }
        'g' => {
          assert_eq!(chars.next().unwrap(), 't');
          assert_eq!(chars.next().unwrap(), ';');
          out.push('>');
        }
        'a' => {
          assert_eq!(chars.next().unwrap(), 'm');
          assert_eq!(chars.next().unwrap(), 'p');
          assert_eq!(chars.next().unwrap(), ';');
          out.push('&');
        }
        other => panic!("{}", other),
      }
    }
  }
  out
}

fn eat_to_comment_close<'s>(iter: &mut impl Iterator<Item = XmlElement<'s>>) {
  loop {
    match iter.next().unwrap() {
      EndTag { name: "comment" } => return,
      _ => continue,
    }
  }
}

fn eat_to_platforms_close<'s>(iter: &mut impl Iterator<Item = XmlElement<'s>>) {
  loop {
    match iter.next().unwrap() {
      EndTag { name: "platforms" } => return,
      _ => continue,
    }
  }
}

fn eat_to_tags_close<'s>(iter: &mut impl Iterator<Item = XmlElement<'s>>) {
  loop {
    match iter.next().unwrap() {
      EndTag { name: "tags" } => return,
      _ => continue,
    }
  }
}

fn main() {
  let vk_xml =
    std::fs::read_to_string("target/vk.xml").expect("Couldn't read vk.xml");
  let mut iter = &mut ElementIterator::new(&vk_xml)
    .filter_map(skip_comments)
    .filter_map(skip_empty_text_elements);
  assert!(matches!(
    iter.next().unwrap(),
    StartTag { name: "registry", attrs: "" }
  ));
  let registry = VkRegistry::from_iter(iter);
  println!("Got A Registry!");
  println!("{:?}", registry);
}

#[derive(Debug, Default)]
pub struct VkRegistry {
  vk_types: VkTypes,
}
impl VkRegistry {
  pub fn from_iter<'s>(
    iter: &mut impl Iterator<Item = XmlElement<'s>>,
  ) -> Self {
    let mut registry = VkRegistry::default();
    loop {
      match iter.next().unwrap() {
        EndTag { name: "registry" } => return registry,
        StartTag { name: "comment", attrs: "" } => eat_to_comment_close(iter),
        StartTag { name: "platforms", attrs: _ } => {
          eat_to_platforms_close(iter)
        }
        StartTag { name: "tags", attrs: _ } => eat_to_tags_close(iter),
        StartTag { name: "types", attrs: _ } => {
          registry.vk_types = VkTypes::from_iter(iter)
        }
        StartTag { name: "enums", attrs: _ } => loop {
          // TODO
          match iter.next().unwrap() {
            EndTag { name: "enums" } => break,
            _ => continue,
          }
        },
        StartTag { name: "commands", attrs: _ } => loop {
          // TODO
          match iter.next().unwrap() {
            EndTag { name: "commands" } => break,
            _ => continue,
          }
        },
        StartTag { name: "feature", attrs: _ } => loop {
          // TODO
          match iter.next().unwrap() {
            EndTag { name: "feature" } => break,
            _ => continue,
          }
        },
        StartTag { name: "extensions", attrs: _ } => loop {
          // TODO
          match iter.next().unwrap() {
            EndTag { name: "extensions" } => break,
            _ => continue,
          }
        },
        unknown => panic!("{:?}", unknown),
      }
    }
  }
}

#[derive(Debug, Default)]
pub struct VkTypes {
  the_types: Vec<VkType>,
}
impl VkTypes {
  pub fn from_iter<'s>(
    iter: &mut impl Iterator<Item = XmlElement<'s>>,
  ) -> Self {
    let mut types = Default::default();
    loop {
      match iter.next().unwrap() {
        EndTag { name: "types" } => {
          println!("{:#?}", types);
          return types;
        }
        StartTag { name: "comment", attrs: "" } => eat_to_comment_close(iter),
        EmptyTag { name: "type", attrs } => {
          if let Some(vk_type) = VkType::from_empty_type_tag(attrs) {
            types.the_types.push(vk_type)
          }
        }
        StartTag { name: "type", attrs } => {
          if let Some(vk_type) = VkType::from_iter_and_attrs(iter, attrs) {
            types.the_types.push(vk_type)
          }
        }
        other => panic!("ILLEGAL: {:?}", other),
      }
    }
  }
}

#[derive(Debug)]
pub enum VkType {
  TypeAlias(VkTypeAlias),
  Enumerant(VkEnumerant),
  Define(String),
}
impl VkType {
  fn from_empty_type_tag(attrs: &str) -> Option<Self> {
    let attrs = hashmap_from_attrs(attrs);
    //
    if attrs.get("category").is_some()
      && attrs.get("name").is_some()
      && attrs.keys().count() == 2
    {
      let category = attrs.get("category").unwrap();
      match category.as_str() {
        "include" => return None,
        "enum" => {
          return Some(VkType::Enumerant(VkEnumerant::from_name(
            attrs.get("name").unwrap().clone(),
          )))
        }
        _ => panic!("illegal category: {:?}", category),
      }
    } else if attrs.get("requires").is_some()
      && attrs.get("name").is_some()
      && attrs.keys().count() == 2
    {
      let requires = attrs.get("requires").unwrap();
      if requires.contains(".h") {
        return None;
      } else if requires.as_str() == "vk_platform" {
        let name: &String = attrs.get("name").unwrap();
        return Some(VkType::TypeAlias(VkTypeAlias {
          new_name: name.clone(),
          old_name: String::from(match name.as_str() {
            "void" => "c_void",
            "char" => "c_char",
            "float" => "c_float",
            "double" => "c_double",
            "uint8_t" => "u8",
            "uint16_t" => "u16",
            "uint32_t" => "u32",
            "uint64_t" => "u64",
            "int32_t" => "i32",
            "int64_t" => "i64",
            "size_t" => "usize",
            _ => panic!("illegal name: {:?}", name),
          }),
        }));
      }
      panic!("requires/name: {:?}", attrs);
    } else if attrs.get("name").map(String::as_str) == Some("int") {
      return Some(VkType::TypeAlias(VkTypeAlias {
        new_name: String::from("int"),
        old_name: String::from("c_int"),
      }));
    } else if attrs.get("category").is_some()
      && attrs.get("name").is_some()
      && attrs.get("alias").is_some()
      && attrs.keys().count() == 3
    {
      return Some(VkType::TypeAlias(VkTypeAlias {
        old_name: attrs.get("alias").unwrap().clone(),
        new_name: attrs.get("name").unwrap().clone(),
      }));
    } else {
      panic!("UNKNOWN EMPTY TAG PATTERN: {:?}", attrs);
    }
  }

  fn from_iter_and_attrs<'s>(
    iter: &mut impl Iterator<Item = XmlElement<'s>>,
    attrs: &str,
  ) -> Option<Self> {
    let attrs = hashmap_from_attrs(attrs);
    if attrs.get("category").map(String::as_ref) == Some("include") {
      assert!(matches!(iter.next().unwrap(), Text(_)));
      assert!(matches!(iter.next().unwrap(), EndTag { name: "type" }));
      return None;
    } else if attrs.get("category").map(String::as_ref) == Some("define") {
      // TODO: extract this to its own method.
      let mut text = String::new();
      loop {
        match iter.next().unwrap() {
          EndTag { name: "type" } => {
            text = revert_xml_encoding(text);
            return Some(VkType::Define(text));
          }
          StartTag { name: "type", attrs } => {
            assert!(attrs.is_empty());
            text.push(' ');
            loop {
              match iter.next().unwrap() {
                EndTag { name: "type" } => break,
                Text(t) => {
                  text.push_str(t);
                }
                _ => (),
              }
            }
          }
          Text(t) => {
            if text.is_empty() {
              text.push_str(t.trim_start());
            } else {
              text.push_str(t)
            }
          }
          StartTag { name: "name", attrs: "" } => (),
          EndTag { name: "name" } => (),
          unknown => panic!("{:?}", unknown),
        }
      }
    } else if attrs.get("category").map(String::as_ref) == Some("basetype") {
      // TODO: process base types
      panic!("{:?}", attrs);
    } else {
      loop {
        match iter.next().unwrap() {
          EndTag { name: "type" } => {
            return None;
          }
          // TODO: handle start/end tag type entries.
          other => panic!("UNKNOWN FROM ITER ({:?}): {:?}", attrs, other),
        }
      }
    }
  }
}

#[derive(Debug)]
pub struct VkTypeAlias {
  new_name: String,
  old_name: String,
}

#[derive(Debug)]
pub struct VkEnumerant {
  name: String,
  possibilities: Vec<String>,
}
impl VkEnumerant {
  pub fn from_name(name: String) -> Self {
    Self { name, possibilities: vec![] }
  }
}
