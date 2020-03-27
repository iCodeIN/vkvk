
use super::*;

#[derive(Debug, Clone)]
pub enum TypeEntry {
  FileToInclude {
    name: String,
    text: String,
  },
  TypeFromInclude {
    name: String,
    required_file: String,
  },
  EmptyTypeEntry {
    name: String,
    alias: String,
    category: String,
  },
  DefaultEntry,
}
impl Default for TypeEntry {
  fn default() -> Self {
    TypeEntry::DefaultEntry
  }
}

impl TypeEntry {
  /// Consume one type definition.
  pub fn from_xml_start<'s>(iter: &mut XmlIterator<'s>, attrs: &'s str) -> Self {
    let tag_attrs: ArrayVec<[_;16]> = AttributeIterator::new(attrs).collect();
    //println!("from_xml_start,tag_attrs:{:?}", tag_attrs);
    let category = tag_attrs.iter().find(|attribute|attribute.key=="category").unwrap();
    let category_value = category.value;
    loop {
      match iter.next() {
        Some(EndTag { name: "type" }) => return TypeEntry::default(),
        Some(Text(t)) => (),
        Some(StartTag { name: "type", attrs }) => {
          // "inner" type tag pair
          assert_eq!(attrs, "");
          let t = match iter.next() {
            Some(Text(t)) => t,
            other => panic!("TypeEntry> unknown {:?}", other),
          };
          match iter.next() {
            Some(EndTag { name: "type" }) => (),
            other => panic!("TypeEntry> unknown {:?}", other),
          }
        }
        Some(StartTag { name: "name", attrs }) => {
          assert_eq!(attrs, "");
          let t = match iter.next() {
            Some(Text(t)) => (),
            other => panic!("TypeEntry> unknown {:?}", other),
          };
          match iter.next() {
            Some(EndTag { name: "name" }) => (),
            other => panic!("TypeEntry> unknown {:?}", other),
          }
        }
        _ => (),
      }
    }
  }

  pub fn from_empty_tag<'s>(attrs: &'s str) -> Self {
    let tag_attrs: ArrayVec<[_;16]> = AttributeIterator::new(attrs).collect();
    let name = tag_attrs.iter().find(|attribute|attribute.key=="name").unwrap().value.to_string();
    let category = tag_attrs.iter().find(|attribute|attribute.key=="category");
    let category_value = category.map(|a|a.value).unwrap_or("");
    match category_value {
      "include" => {
        TypeEntry::FileToInclude { name, text: String::new() }
      }
      "" => {
        let requires_attribute = tag_attrs.iter().find(|attribute|attribute.key=="requires");
        let required_file = requires_attribute.map(|a|a.value).unwrap_or("").to_string();
        TypeEntry::TypeFromInclude { name, required_file }
      }
      "enum" |  "bitmask" | "struct" | "handle" => {
        let alias_attribute = tag_attrs.iter().find(|attribute|attribute.key=="alias");
        let alias = alias_attribute.map(|a|a.value).unwrap_or("").to_string();
        let category = category_value.to_string();
        println!("name:{name}, alias:{alias}, category:{category}", name = name, alias = alias, category = category);
        TypeEntry::EmptyTypeEntry { name, alias, category }
      }
      _ => {
        println!("from_empty_tag,unknown category:\n{:?}", tag_attrs);
        Self::default()
      }
    }
  }
}
