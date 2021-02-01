#![allow(unused_imports)]

use magnesium::{skip_comments, skip_empty_text_elements, ElementIterator, TagAttribute, TagAttributeIterator, XmlElement, XmlElement::*};
use std::collections::HashMap;
use tinyvec::TinyVec;

mod burners;
use burners::*;

mod collectors;
use collectors::*;

mod data_types;
use data_types::*;

type AttrList<'s> = TinyVec<[TagAttribute<'s>; 4]>;

fn main() {
  let filename = std::env::args().skip(1).next().unwrap();
  let file_string = std::fs::read_to_string(filename).expect("couldn't open vk.xml");
  let mut elem_iter = ElementIterator::new(&file_string).filter_map(skip_comments).filter_map(skip_empty_text_elements);
  assert_eq!(elem_iter.next(), Some(StartTag { name: "registry", attrs: "" }));
  let mut registry = collect_registry(&mut elem_iter);
  registry.definitions.sort();
  print_standard_definitions();
  for def in registry.definitions.iter().filter(|f| {
    matches!(
      f,
      VulkanDefinition::Handle(_)
        | VulkanDefinition::NonDispatchableHandle(_)
        | VulkanDefinition::Enumeration(_)
        | VulkanDefinition::Bitmask(_)
        | VulkanDefinition::Struct(_)
        | VulkanDefinition::Union(_)
    )
  }) {
    println!("{}", def);
  }
}

#[allow(unused)]
pub fn collect_registry<'s>(elem_iter: &mut impl Iterator<Item = XmlElement<'s>>) -> Registry {
  // TODO
  let mut registry = Registry::default();
  loop {
    match elem_iter.next().unwrap() {
      EndTag { name: "registry" } => return registry,
      StartTag { name: "comment", attrs: "" } => burn_comment(elem_iter),
      StartTag { name: "platforms", attrs } => burn_platforms(elem_iter, attrs),
      StartTag { name: "tags", attrs } => burn_tags(elem_iter, attrs),
      StartTag { name: "types", attrs } => registry.definitions.extend(collect_types(elem_iter, attrs)),
      StartTag { name: "enums", attrs } => registry.definitions.extend(collect_enums(elem_iter, attrs)),
      StartTag { name: "commands", .. } => registry.definitions.extend(collect_commands(elem_iter)),
      StartTag { name: "feature", .. } => collect_feature(elem_iter),
      StartTag { name: "extensions", .. } => collect_extensions(elem_iter),
      StartTag { name: "spirvextensions", attrs } => burn_spirvextensions(elem_iter, attrs),
      StartTag { name: "spirvcapabilities", attrs } => burn_spirvcapabilities(elem_iter, attrs),
      other => panic!("{:?}", other),
    }
  }
}

#[allow(unused)]
pub fn collect_feature<'s>(elem_iter: &mut impl Iterator<Item = XmlElement<'s>>) {
  // TODO
  loop {
    match elem_iter.next().unwrap() {
      EndTag { name: "feature" } => return,
      StartTag { name: "comment", .. } => burn_comment(elem_iter),
      StartTag { name: "require", .. } => loop {
        match elem_iter.next().unwrap() {
          EndTag { name: "require" } => break,
          _ => (),
        }
      },
      EmptyTag { name: "require", .. } => (),
      other => panic!("{:?}", other),
    }
  }
}

#[allow(unused)]
pub fn collect_extensions<'s>(elem_iter: &mut impl Iterator<Item = XmlElement<'s>>) {
  // TODO
  loop {
    match elem_iter.next().unwrap() {
      EndTag { name: "extensions" } => return,
      StartTag { name: "comment", .. } => burn_comment(elem_iter),
      StartTag { name: "extension", .. } => loop {
        match elem_iter.next().unwrap() {
          EndTag { name: "extension" } => break,
          _ => (),
        }
      },
      EmptyTag { name: "extension", .. } => (),
      other => panic!("{:?}", other),
    }
  }
}

pub fn print_standard_definitions() {
  println!("#![no_std]");
  println!("#![allow(bad_style)]");
  println!("use core::ffi::c_void;");
  println!("#[repr(transparent)] pub struct VkSampleMask(pub uint32_t); impl Copy for VkSampleMask {{}} impl Clone for VkSampleMask{{ fn clone(&self) -> Self {{ *self }} }}");
  println!("#[repr(transparent)] pub struct VkBool32(pub uint32_t); impl Copy for VkBool32 {{}} impl Clone for VkBool32{{ fn clone(&self) -> Self {{ *self }} }}");
  println!("#[repr(transparent)] pub struct VkDeviceSize(pub uint64_t); impl Copy for VkDeviceSize {{}} impl Clone for VkDeviceSize{{ fn clone(&self) -> Self {{ *self }} }}");
  println!("#[repr(transparent)] pub struct VkDeviceAddress(pub uint64_t); impl Copy for VkDeviceAddress {{}} impl Clone for VkDeviceAddress{{ fn clone(&self) -> Self {{ *self }} }}");
  println!("#[repr(transparent)] pub struct CAMetalLayer(c_void);");
  println!("#[repr(transparent)] pub struct ANativeWindow(c_void);");
  println!("#[repr(transparent)] pub struct AHardwareBuffer(c_void);");
  println!("#[allow(unused)] type char = u8;");
  println!("#[allow(unused)] type float = f32;");
  println!("#[allow(unused)] type double = f64;");
  println!("#[allow(unused)] type uint8_t = u8;");
  println!("#[allow(unused)] type uint16_t = u16;");
  println!("#[allow(unused)] type uint32_t = u32;");
  println!("#[allow(unused)] type uint64_t = u64;");
  println!("#[allow(unused)] type int32_t = i32;");
  println!("#[allow(unused)] type int64_t = i64;");
  println!("#[allow(unused)] type size_t = usize;");
  println!("#[allow(unused)] type c_int = i32;");
  println!("#[allow(unused)] type int = i32;");
  println!("type TODO = ();");
  println!("#[allow(unused)] type HANDLE = *mut c_void;");
  println!("#[allow(unused)] type HWND = *mut c_void;");
  println!("#[allow(unused)] type HMONITOR = *mut c_void;");
  println!("#[allow(unused)] type HINSTANCE = *mut c_void;");
  println!("#[allow(unused)] type LPCWSTR = *mut u16;");
  println!("#[allow(unused)] type Window = TODO;");
  println!("#[allow(unused)] type Display = TODO;");
  println!("#[allow(unused)] type xcb_connection_t = TODO;");
  println!("#[allow(unused)] type xcb_window_t = TODO;");
  println!("#[allow(unused)] type wl_display = TODO;");
  println!("#[allow(unused)] type wl_surface = TODO;");
  println!("pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;");
  println!("pub const VK_UUID_SIZE: usize = 16;");
  println!("pub const VK_LUID_SIZE: usize = 8;");
  println!("pub const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;");
  println!("pub const VK_MAX_DESCRIPTION_SIZE: usize = 256;");
  println!("pub const VK_MAX_MEMORY_TYPES: usize = 32;");
  println!(
    "#[doc=\"The maximum number of unique memory heaps, each of which supporting 1 or more memory types\"]pub const VK_MAX_MEMORY_HEAPS: usize = 16;"
  );
  println!("pub const VK_LOD_CLAMP_NONE: f32 = 1000.0;");
  println!("pub const VK_REMAINING_MIP_LEVELS: usize = usize::MAX;");
  println!("pub const VK_REMAINING_ARRAY_LAYERS: usize = usize::MAX;");
  println!("pub const VK_WHOLE_SIZE: usize = usize::MAX;");
  println!("pub const VK_ATTACHMENT_UNUSED: usize = usize::MAX;");
  println!("pub const VK_TRUE: usize = 1;");
  println!("pub const VK_FALSE: usize = 0;");
  println!("pub const VK_QUEUE_FAMILY_IGNORED: usize = usize::MAX;");
  println!("pub const VK_QUEUE_FAMILY_EXTERNAL: usize = usize::MAX-1;");
  println!("pub const VK_QUEUE_FAMILY_FOREIGN_EXT: usize = usize::MAX-2;");
  println!("pub const VK_SUBPASS_EXTERNAL: usize = usize::MAX;");
  println!("pub const VK_MAX_DEVICE_GROUP_SIZE: usize = 32;");
  println!("pub const VK_MAX_DRIVER_NAME_SIZE: usize = 256;");
  println!("pub const VK_MAX_DRIVER_INFO_SIZE: usize = 256;");
  println!("pub const VK_SHADER_UNUSED_KHR: usize = usize::MAX;");
  println!("pub const VK_SHADER_UNUSED_NV: usize = VK_SHADER_UNUSED_KHR;");
  println!("pub const VK_LUID_SIZE_KHR: usize = VK_LUID_SIZE;");
  println!("pub const VK_MAX_DRIVER_INFO_SIZE_KHR: usize = VK_MAX_DRIVER_INFO_SIZE;");
  println!("pub const VK_MAX_DRIVER_NAME_SIZE_KHR: usize = VK_MAX_DRIVER_NAME_SIZE;");
  println!("pub const VK_MAX_DEVICE_GROUP_SIZE_KHR: usize = VK_MAX_DEVICE_GROUP_SIZE;");
  println!("pub const VK_QUEUE_FAMILY_EXTERNAL_KHR: usize = VK_QUEUE_FAMILY_EXTERNAL;");
  println!("pub trait VkFlags: Copy {{");
  println!("  fn from_u32(flags: u32) -> Self; fn to_u32(self) -> u32;");
  println!("}}");

  println!();
  println!("/* --- --- --- */");
  println!();
}
