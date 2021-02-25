#![no_std]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[macro_use]
mod macros;

#[rustfmt::skip]
pub mod prelude {
  pub use crate::vk_platform::*;
  pub use crate::version_1_0::*;
  pub use crate::handles::*;
  pub use crate::non_dispatchable_handles::*;
  pub use crate::vk_bool::*;
  pub use crate::vk_version::*;
  pub use crate::vk_result::*;
  pub use crate::vk_structure_type::*;
}

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

  pub const VK_ATTACHMENT_UNUSED: u32 = !0;

  pub const VK_LOD_CLAMP_NONE: f32 = 1000.0;

  pub const VK_QUEUE_FAMILY_IGNORED: u32 = !0;

  pub const VK_REMAINING_ARRAY_LAYERS: u32 = !0;

  pub const VK_REMAINING_MIP_LEVELS: u32 = !0;

  pub const VK_SUBPASS_EXTERNAL: u32 = !0;

  pub const VK_WHOLE_SIZE: u64 = !0;
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
