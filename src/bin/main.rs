use vkvk::xml::*;

fn main() {
  let bytes = std::fs::read("vk.xml").unwrap();
  let text = String::from_utf8(bytes).unwrap();
  let main_text = drop_declaration(&text);
  for x in XmlIterator::new(main_text) {
    println!("{:?}", x);
  }
}
