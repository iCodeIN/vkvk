#![no_std]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[macro_use]
mod macros;

// TODO: prelude module

pub mod vk_platform;
use vk_platform::*;

pub use version_1_0::*;
mod version_1_0 {
  use super::*;

  mod structures;
  pub use structures::*;
}

pub mod handles;

pub mod non_dispatchable_handles;

mod vk_bool;
pub use vk_bool::*;

pub mod vk_result;
use vk_result::*;

pub mod vk_structure_type;
use vk_structure_type::*;
