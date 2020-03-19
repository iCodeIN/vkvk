use vkvk::{
  xml::{XmlElement::*, *},
  *,
};

fn main() {
  let bytes = std::fs::read("vk.xml").unwrap();
  let text = String::from_utf8(bytes).unwrap();
  let main_text = drop_declaration(&text);
  let mut iter = XmlIterator::new(main_text);
  let _registry = loop {
    match iter.next() {
      Some(StartTag { name: "registry", .. }) => {
        break Registry::from_xml(&mut iter);
      }
      other => panic!("bin_main:{:?}", other),
    }
  };
}
