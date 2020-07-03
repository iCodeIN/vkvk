#![allow(unused_mut)]
#![allow(unused_imports)]

use std::collections::HashMap;

use magnesium::{XmlElement::*, *};

pub fn hashmap_from_attrs(attrs: &str) -> HashMap<String, String> {
  TagAttributeIterator::new(attrs)
    .map(|ta| (ta.key.to_string(), ta.value.to_string()))
    .collect()
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
  let file_data =
    std::fs::read_to_string("target/vk.xml").expect("Couldn't read vk.xml");
  let mut iter = &mut ElementIterator::new(&file_data)
    .filter_map(skip_comments)
    .filter_map(skip_empty_text_elements);
  let registry = loop {
    match iter.next().unwrap() {
      StartTag { name: "registry", attrs: "" } => {
        break VkRegistry::from_iter(iter)
      }
      unknown => panic!("{:?}", unknown),
    }
  };
  println!("Got A Registry!");
  println!("{:?}", registry);
}

#[derive(Debug)]
pub struct VkRegistry {
  //
}

impl VkRegistry {
  pub fn from_iter<'s>(
    iter: &mut impl Iterator<Item = XmlElement<'s>>,
  ) -> Self {
    let mut registry = VkRegistry {};
    loop {
      match iter.next().unwrap() {
        EndTag { name: "registry" } => return registry,
        StartTag { name: "comment", attrs: "" } => eat_to_comment_close(iter),
        StartTag { name: "platforms", attrs: _ } => {
          eat_to_platforms_close(iter)
        }
        StartTag { name: "tags", attrs: _ } => eat_to_tags_close(iter),
        StartTag { name: "types", attrs: _ } => drop(VkTypes::from_iter(iter)),
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
    //
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
