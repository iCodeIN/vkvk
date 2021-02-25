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

  mod enumerations;
  pub use enumerations::*;

  mod flag_bits;
  pub use flag_bits::*;

  mod structures;
  pub use structures::*;
}

pub mod handles;
use handles::*;

pub mod non_dispatchable_handles;
use non_dispatchable_handles::*;

mod vk_bool;
pub use vk_bool::*;

mod vk_version;
pub use vk_version::*;

pub mod vk_result;
use vk_result::*;

pub mod vk_structure_type;
use vk_structure_type::*;
