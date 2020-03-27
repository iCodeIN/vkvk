use super::*;

#[derive(Debug, Clone, Default)]
pub struct TypesEntry<'s> {
  types: Vec<TypeEntry>,
  _marker: PhantomData<&'s str>,
}

impl<'s> TypesEntry<'s> {
  /// Consume all the vulkan type definitions. Critical.
  pub fn from_xml(iter: &mut XmlIterator<'s>, attrs: &'s str) -> Self {
    let mut types_entries = TypesEntry::default();
    loop {
      match iter.next() {
        Some(EndTag { name: "types" }) => {
          println!("TypeEntries> Gathered {}", types_entries.types.len());
          return types_entries
        },
        Some(EmptyTag { name: "type", attrs }) => {
          types_entries.types.push(TypeEntry::from_empty_tag(attrs));
        }
        Some(StartTag { name: "type", attrs }) => {
          types_entries.types.push(TypeEntry::from_xml_start(iter, attrs));
        }
        Some(StartTag { name: "comment", attrs }) => {
          do_comment(iter, attrs);
        }
        other => (),
      }
    }
  }
}
