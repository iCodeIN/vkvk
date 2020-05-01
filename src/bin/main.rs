#![allow(unused_mut)]
#![allow(unused_imports)]

use std::collections::HashMap;

use magnesium::{XmlElement::*, *};

use vkvk::*;

fn main() {
  let file_data =
    std::fs::read_to_string("vk.xml").expect("Couldn't read vk.xml");
  let mut iter = &mut ElementIterator::new(&file_data)
    .filter_map(skip_comments)
    .filter_map(skip_empty_text_elements);
  let registry = loop {
    match iter.next().unwrap() {
      StartTag { name: "registry", attrs: "" } => break parse_registry(iter),
      unknown => panic!("main> {:?}", unknown),
    }
  };
  println!("got a Registry!");
  println!("{:?}", registry);
}
