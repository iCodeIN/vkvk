use super::*;

/// Grabs up all the `<types>` tag info.
pub fn collect_types<'s>(elem_iter: &mut impl Iterator<Item = XmlElement<'s>>, attrs: &'s str) -> Vec<VulkanTypeDefinition> {
  let attrs: AttrList<'s> = TagAttributeIterator::new(attrs).collect();
  assert_eq!(attrs.len(), 1);
  assert_eq!(attrs[0].key, "comment");

  let mut definitions = Vec::new();
  loop {
    match elem_iter.next().unwrap() {
      EndTag { name: "types" } => return definitions,
      StartTag { name: "comment", .. } => burn_comment(elem_iter),
      StartTag { name: "type", attrs } => {
        if let Some(def) = collect_type_start(elem_iter, attrs) {
          definitions.push(def);
        }
      }
      EmptyTag { name: "type", attrs } => {
        if let Some(def) = collect_type_empty(attrs) {
          definitions.push(def);
        }
      }
      other => panic!("{:?}", other),
    }
  }
}

#[allow(unused)]
pub fn collect_type_start<'s>(elem_iter: &mut impl Iterator<Item = XmlElement<'s>>, attrs: &'s str) -> Option<VulkanTypeDefinition> {
  // TODO
  let mut attrs: AttrList<'s> = TagAttributeIterator::new(attrs).collect();
  attrs.sort();

  // skip includes
  if attrs.iter().find(|a| a == &&TagAttribute { key: "category", value: "include" }).is_some() {
    loop {
      match elem_iter.next().unwrap() {
        EndTag { name: "type" } => return None,
        Text(t) => (),
        other => panic!("{:?}", other),
      }
    }
  }

  // skip defines
  if attrs.iter().find(|a| a == &&TagAttribute { key: "category", value: "define" }).is_some() {
    let mut source = String::new();
    loop {
      match elem_iter.next().unwrap() {
        EndTag { name: "type" } => return None,
        StartTag { name: "name", attrs: "" } | EndTag { name: "name" } => continue,
        StartTag { name: "type", attrs: "" } => loop {
          match elem_iter.next().unwrap() {
            EndTag { name: "type" } => break,
            Text(_) => (),
            other => panic!("{:?}", other),
          }
        },
        Text(_) => (),
        other => panic!("{:?}", other),
      }
    }
  }

  // skip basetypes
  if attrs.iter().find(|a| a == &&TagAttribute { key: "category", value: "basetype" }).is_some() {
    loop {
      match elem_iter.next().unwrap() {
        EndTag { name: "type" } => return None,
        StartTag { name: "name", attrs: "" } | EndTag { name: "name" } => continue,
        StartTag { name: "type", attrs: "" } => loop {
          match elem_iter.next().unwrap() {
            EndTag { name: "type" } => break,
            Text(_) => (),
            other => panic!("{:?}", other),
          }
        },
        Text(_) => (),
        other => panic!("{:?}", other),
      }
    }
  }

  // bitmask entries. Ignore them here, we get the info from the `<enums>` later.
  if attrs.iter().find(|a| a == &&TagAttribute { key: "category", value: "bitmask" }).is_some() {
    loop {
      match elem_iter.next().unwrap() {
        EndTag { name: "type" } => return None,
        StartTag { name: "name", attrs: "" } | EndTag { name: "name" } => continue,
        StartTag { name: "type", attrs: "" } => loop {
          match elem_iter.next().unwrap() {
            EndTag { name: "type" } => break,
            Text(_) => (),
            other => panic!("{:?}", other),
          }
        },
        Text(_) => (),
        other => panic!("{:?}", other),
      }
    }
  }

  // handle entries are a newtype over either a pointer (dispatchable) or a u64
  // (non-dispatchable)
  if attrs.iter().find(|a| a == &&TagAttribute { key: "category", value: "handle" }).is_some() {
    let objtypeenum = attrs.iter().find(|a| matches!(a, TagAttribute { key: "objtypeenum", value: _ })).unwrap().value.to_string();
    let parent = attrs.iter().find(|a| matches!(a, TagAttribute { key: "parent", value: _ })).map(|t| t.value.to_string());

    let mut the_type = "";
    let mut the_name = "";
    loop {
      match elem_iter.next().unwrap() {
        EndTag { name: "type" } => break,
        StartTag { name: "type", attrs: "" } => loop {
          match elem_iter.next().unwrap() {
            EndTag { name: "type" } => break,
            Text(t) => the_type = t,
            other => panic!("{:?}", other),
          }
        },
        StartTag { name: "name", attrs: "" } => loop {
          match elem_iter.next().unwrap() {
            EndTag { name: "name" } => break,
            Text(t) => the_name = t,
            other => panic!("{:?}", other),
          }
        },
        Text(_) => (),
        other => panic!("{:?}", other),
      }
    }

    match the_type {
      "VK_DEFINE_HANDLE" => return Some(VulkanTypeDefinition::Handle(Handle { name: the_name.to_string(), objtypeenum, parent })),
      "VK_DEFINE_NON_DISPATCHABLE_HANDLE" => {
        return Some(VulkanTypeDefinition::NonDispatchableHandle(NonDispatchableHandle { name: the_name.to_string(), objtypeenum, parent }))
      }
      other => panic!("{:?}", other),
    }
  }

  // function pointers are simple to understand but long to parse.
  if attrs.iter().find(|a| a == &&TagAttribute { key: "category", value: "funcpointer" }).is_some() {
    match elem_iter.next().unwrap() {
      Text(_vk_api_ptr) => (),
      other => panic!("{:?}", other),
    }
    match elem_iter.next().unwrap() {
      StartTag { name: "name", attrs: "" } => (),
      other => panic!("{:?}", other),
    }
    let name = match elem_iter.next().unwrap() {
      Text(name) => name.to_string(),
      other => panic!("{:?}", other),
    };
    match elem_iter.next().unwrap() {
      EndTag { name: "name" } => (),
      other => panic!("{:?}", other),
    }
    let mut next_is_const = match elem_iter.next().unwrap() {
      Text(parens) => parens.contains("const"),
      other => panic!("{:?}", other),
    };
    let mut args: Vec<(String, String)> = Vec::new();
    loop {
      match elem_iter.next().unwrap() {
        EndTag { name: "type" } => break,
        StartTag { name: "type", attrs: "" } => (),
        other => panic!("{:?}", other),
      }
      let arg_ty = match elem_iter.next().unwrap() {
        Text(arg_ty) => arg_ty,
        other => panic!("{:?}", other),
      };
      match elem_iter.next().unwrap() {
        EndTag { name: "type" } => (),
        other => panic!("{:?}", other),
      }
      let arg_name = match elem_iter.next().unwrap() {
        Text(arg_name) => arg_name.trim(),
        other => panic!("{:?}", other),
      };
      //
      let is_ptr = arg_name.starts_with('*');
      let arg_name: String = {
        let mut arg_name = if is_ptr { &arg_name[1..].trim() } else { arg_name.trim() };
        let p = arg_name.as_bytes().iter().copied().position(|b| !(b as char).is_alphabetic()).unwrap();
        arg_name = &arg_name[..p];
        //
        arg_name.to_string()
      };
      let arg_ty = if is_ptr {
        format!(
          "*{} {}",
          if next_is_const { "const" } else { "mut" },
          match arg_ty {
            "char" => "u8",
            "void" => "c_void",
            other => other,
          }
        )
      } else {
        arg_ty.to_string()
      };
      next_is_const = arg_name.contains("const");
      args.push((arg_name, arg_ty));
    }
    return Some(VulkanTypeDefinition::FnPtrAlias(FnPtrAlias { name, args }));
  }

  // Structs are a whole thing
  if attrs.iter().find(|a| a == &&TagAttribute { key: "category", value: "struct" }).is_some() {
    let name = attrs.iter().find(|a| matches!(a, TagAttribute { key: "name", value: _ })).unwrap().value.to_string();
    let structextends = attrs.iter().find(|a| matches!(a, TagAttribute { key: "structextends", value: _ })).map(|t| t.value.to_string());
    let mut fields = Vec::new();
    'field_gather: loop {
      let mut field = StructField::default();
      match elem_iter.next().unwrap() {
        EndTag { name: "type" } => break 'field_gather,
        StartTag { name: "comment", attrs: "" } => burn_comment(elem_iter),
        StartTag { name: "member", attrs } => {
          field.attrs = TagAttributeIterator::new(attrs).map(|t| (t.key.to_string(), t.value.to_string())).collect();
          // grab up the type
          {
            match elem_iter.next().unwrap() {
              StartTag { name: "type", attrs: "" } => (),
              Text(t) => {
                field.is_const = t.contains("const");
                assert!(matches!(elem_iter.next().unwrap(), StartTag { name: "type", attrs: "" }));
              }
              other => panic!("{:?}", other),
            }
            field.type_ = match elem_iter.next().unwrap() {
              Text("char") => "u8".to_string(),
              Text("void") => "c_void".to_string(),
              Text(t) => t.to_string(),
              other => panic!("{:?}", other),
            };
            assert!(matches!(elem_iter.next().unwrap(), EndTag { name: "type" }));
          }
          // grab up the name
          {
            match elem_iter.next().unwrap() {
              StartTag { name: "name", attrs: "" } => (),
              Text(t) => {
                field.is_ptr = t.contains("*");
                assert!(matches!(elem_iter.next().unwrap(), StartTag { name: "name", attrs: "" }));
              }
              other => panic!("{:?}", other),
            }
            field.name = match elem_iter.next().unwrap() {
              Text(t) => t.to_string(),
              other => panic!("{:?}", other),
            };
            assert!(matches!(elem_iter.next().unwrap(), EndTag { name: "name" }));
          }
          // grab up any extras (array and comment), if any
          loop {
            match elem_iter.next().unwrap() {
              EndTag { name: "member" } => {
                fields.push(field);
                continue 'field_gather;
              }
              Text("[") => {
                assert!(matches!(elem_iter.next().unwrap(), StartTag { name: "enum", attrs: "" }));
                field.array_count = match elem_iter.next().unwrap() {
                  Text(t) => Some(t.to_string()),
                  other => panic!("{:?}", other),
                };
                assert!(matches!(elem_iter.next().unwrap(), EndTag { name: "enum" }));
                assert!(matches!(elem_iter.next().unwrap(), Text("]")));
              }
              Text(t) => {
                field.array_count = Some(t[1..t.len() - 1].to_string());
              }
              StartTag { name: "comment", attrs: "" } => {
                field.comment = match elem_iter.next().unwrap() {
                  Text(t) => Some(t.to_string()),
                  other => panic!("{:?}", other),
                };
                assert!(matches!(elem_iter.next().unwrap(), EndTag { name: "comment" }));
              }
              other => panic!("{:?}", other),
            }
          }
        }
        other => panic!("{:?}", other),
      }
    }
    return Some(VulkanTypeDefinition::Struct(Struct { name, structextends, fields }));
  }

  // Unions are a whole thing too
  if attrs.iter().find(|a| a == &&TagAttribute { key: "category", value: "union" }).is_some() {
    let name = attrs.iter().find(|a| matches!(a, TagAttribute { key: "name", value: _ })).unwrap().value.to_string();
    let mut fields = Vec::new();
    'field_gather: loop {
      let mut field = StructField::default();
      match elem_iter.next().unwrap() {
        EndTag { name: "type" } => break 'field_gather,
        StartTag { name: "comment", attrs: "" } => burn_comment(elem_iter),
        StartTag { name: "member", attrs } => {
          field.attrs = TagAttributeIterator::new(attrs).map(|t| (t.key.to_string(), t.value.to_string())).collect();
          // grab up the type
          {
            match elem_iter.next().unwrap() {
              StartTag { name: "type", attrs: "" } => (),
              Text(t) => {
                field.is_const = t.contains("const");
                assert!(matches!(elem_iter.next().unwrap(), StartTag { name: "type", attrs: "" }));
              }
              other => panic!("{:?}", other),
            }
            field.type_ = match elem_iter.next().unwrap() {
              Text("char") => "u8".to_string(),
              Text("void") => "c_void".to_string(),
              Text(t) => t.to_string(),
              other => panic!("{:?}", other),
            };
            assert!(matches!(elem_iter.next().unwrap(), EndTag { name: "type" }));
          }
          // grab up the name
          {
            match elem_iter.next().unwrap() {
              StartTag { name: "name", attrs: "" } => (),
              Text(t) => {
                field.is_ptr = t.contains("*");
                assert!(matches!(elem_iter.next().unwrap(), StartTag { name: "name", attrs: "" }));
              }
              other => panic!("{:?}", other),
            }
            field.name = match elem_iter.next().unwrap() {
              Text(t) => t.to_string(),
              other => panic!("{:?}", other),
            };
            assert!(matches!(elem_iter.next().unwrap(), EndTag { name: "name" }));
          }
          // grab up any extras (array and comment), if any
          loop {
            match elem_iter.next().unwrap() {
              EndTag { name: "member" } => {
                fields.push(field);
                continue 'field_gather;
              }
              Text("[") => {
                assert!(matches!(elem_iter.next().unwrap(), StartTag { name: "enum", attrs: "" }));
                field.array_count = match elem_iter.next().unwrap() {
                  Text(t) => Some(t.to_string()),
                  other => panic!("{:?}", other),
                };
                assert!(matches!(elem_iter.next().unwrap(), EndTag { name: "enum" }));
                assert!(matches!(elem_iter.next().unwrap(), Text("]")));
              }
              Text(t) => {
                field.array_count = Some(t[1..t.len() - 1].to_string());
              }
              StartTag { name: "comment", attrs: "" } => {
                field.comment = match elem_iter.next().unwrap() {
                  Text(t) => Some(t.to_string()),
                  other => panic!("{:?}", other),
                };
                assert!(matches!(elem_iter.next().unwrap(), EndTag { name: "comment" }));
              }
              other => panic!("{:?}", other),
            }
          }
        }
        other => panic!("{:?}", other),
      }
    }
    return Some(VulkanTypeDefinition::Union(Union { name, fields }));
  }

  // unknown
  println!("start tag type: {:#?}", attrs);
  loop {
    match elem_iter.next().unwrap() {
      EndTag { name: "type" } => return None,
      StartTag { name: "comment", attrs: "" } => burn_comment(elem_iter),
      other => panic!("{:?}", other),
    }
  }
}

/// Collects empty tags that contain type info.
///
/// This where we get most all of our type aliases.
///
/// Empty tags can have "enum" names, but that name is also elsewhere so we
/// don't generate it here.
pub fn collect_type_empty<'s>(attrs: &'s str) -> Option<VulkanTypeDefinition> {
  let attrs: AttrList<'s> = TagAttributeIterator::new(attrs).collect();

  if attrs.iter().find(|a| a == &&TagAttribute { key: "category", value: "include" }).is_some() {
    // skip any includes.
    return None;
  }

  if attrs.iter().find(|a| matches!(a, TagAttribute { key: "requires", value: _ })).is_some() {
    // skip any "requires" stuff, that's basically an include.
    return None;
  }

  if attrs.iter().find(|a| matches!(a, TagAttribute { key: "name", value: "int" })).is_some() {
    // This is just `c_int`, we provide that ourselves.
    return None;
  }

  let name_attr = attrs.iter().find(|a| matches!(a, TagAttribute { key: "name", value: _ }));
  let alias_attr = attrs.iter().find(|a| matches!(a, TagAttribute { key: "alias", value: _ }));

  if name_attr.is_some() && alias_attr.is_some() {
    match (name_attr, alias_attr) {
      (Some(n), Some(a)) => {
        return Some(VulkanTypeDefinition::TypeAlias(TypeAlias { old: n.value.to_string(), new: a.value.to_string() }));
      }
      _ => panic!(),
    }
  }

  if attrs.iter().find(|a| a == &&TagAttribute { key: "category", value: "enum" }).is_some() {
    // Don't store the name, because in the "enums" part of the XML later we'll get
    // both the name as well as all the values.
    return None;
  }

  panic!("{:?}", attrs);
}

/// Collects a single `<enums>` instance.
///
/// Each `<enums>` instance holds either an Enumeration or a Bitmask.
pub fn collect_enums<'s>(elem_iter: &mut impl Iterator<Item = XmlElement<'s>>, attrs: &'s str) -> Option<VulkanTypeDefinition> {
  let attrs: AttrList<'s> = TagAttributeIterator::new(attrs).collect();
  //
  let name = attrs.iter().find(|a| matches!(a, TagAttribute { key: "name", value: _ })).unwrap().value;
  if name == "API Constants" {
    // skip over the API constants
    loop {
      match elem_iter.next().unwrap() {
        EndTag { name: "enums" } => return None, /* TODO */
        EmptyTag { name: "enum", attrs: _ } => {
          // Stability: should we check that all API constants are as we expect?
        }
        other => panic!("{:?}", other),
      }
    }
  }

  let type_ = attrs.iter().find(|a| matches!(a, TagAttribute { key: "type", value: _ })).unwrap().value;

  match type_ {
    "enum" => {
      let mut e = Enumeration { name: name.to_string(), entries: Vec::new() };
      loop {
        match elem_iter.next().unwrap() {
          EndTag { name: "enums" } => {
            return Some(VulkanTypeDefinition::Enumeration(e));
          }
          StartTag { name: "comment", .. } => burn_comment(elem_iter),
          EmptyTag { name: "enum", attrs } => {
            let entry_attrs: HashMap<String, String> = TagAttributeIterator::new(attrs).map(|t| (t.key.to_string(), t.value.to_string())).collect();
            e.entries.push(entry_attrs);
          }
          EmptyTag { name: "unused", .. } => {
            // these are comment-like tags, we ignore them.
            assert!(e.name == "VkResult" || e.name == "VkVendorId");
          }
          other => panic!("{:?}", other),
        }
      }
    }
    "bitmask" => {
      let mut b = Bitmask { name: name.to_string(), entries: Vec::new() };
      loop {
        match elem_iter.next().unwrap() {
          EndTag { name: "enums" } => {
            return Some(VulkanTypeDefinition::Bitmask(b));
          }
          StartTag { name: "comment", .. } => burn_comment(elem_iter),
          EmptyTag { name: "enum", attrs } => {
            let entry_attrs: HashMap<String, String> = TagAttributeIterator::new(attrs).map(|t| (t.key.to_string(), t.value.to_string())).collect();
            b.entries.push(entry_attrs);
          }
          other => panic!("{:?}", other),
        }
      }
    }
    other => panic!("{}", other),
  }
}
