use super::*;

/// Eats the comment, up to and including the end tag.
pub fn burn_comment<'s>(elem_iter: &mut impl Iterator<Item = XmlElement<'s>>) {
  loop {
    match elem_iter.next().unwrap() {
      EndTag { name: "comment" } => return,
      Text(_) => (),
      _ => (),
    }
  }
}

/// We discard all the platforms information.
pub fn burn_platforms<'s>(elem_iter: &mut impl Iterator<Item = XmlElement<'s>>, _attrs: &'s str) {
  loop {
    match elem_iter.next().unwrap() {
      EndTag { name: "platforms" } => return,
      EmptyTag { name: "platform", attrs: _ } => {}
      other => {
        panic!("PLATFORMS! {:?}", other);
      }
    }
  }
}

/// We discard all tags info.
pub fn burn_tags<'s>(elem_iter: &mut impl Iterator<Item = XmlElement<'s>>, _attrs: &'s str) {
  loop {
    match elem_iter.next().unwrap() {
      EndTag { name: "tags" } => return,
      EmptyTag { name: "tag", attrs: _ } => (),
      other => panic!("{:?}", other),
    }
  }
}

/// We discard all info about SPIR-V extensions.
pub fn burn_spirvextensions<'s>(elem_iter: &mut impl Iterator<Item = XmlElement<'s>>, _attrs: &'s str) {
  loop {
    match elem_iter.next().unwrap() {
      EndTag { name: "spirvextensions" } => return,
      StartTag { name: "spirvextension", .. } => loop {
        match elem_iter.next().unwrap() {
          EndTag { name: "spirvextension" } => break,
          EmptyTag { name: "enable", attrs: _ } => continue,
          other => panic!("{:?}", other),
        }
      },
      other => panic!("{:?}", other),
    }
  }
}

/// We discard all info about SPIR-V capabilities.
pub fn burn_spirvcapabilities<'s>(elem_iter: &mut impl Iterator<Item = XmlElement<'s>>, _attrs: &'s str) {
  loop {
    match elem_iter.next().unwrap() {
      EndTag { name: "spirvcapabilities" } => return,
      StartTag { name: "spirvcapability", attrs: _ } => loop {
        match elem_iter.next().unwrap() {
          EndTag { name: "spirvcapability" } => break,
          EmptyTag { name: "enable", attrs: _ } => continue,
          other => panic!("{:?}", other),
        }
      },
      other => panic!("{:?}", other),
    }
  }
}
