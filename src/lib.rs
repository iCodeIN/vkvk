#![no_std]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]

//! Don't use this crate. Use [erupt](https://docs.rs/erupt) if you need Vulkan bindings.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[macro_use]
mod macros;

/// Makes a `&str` from all bytes before the first `0` within the byte slice.
///
/// If the slice has no `0` then the entire slice is used.
pub fn str_from_null_terminated_byte_slice(bytes: &[u8]) -> Result<&str, core::str::Utf8Error> {
  let terminal_position = bytes.iter().copied().position(|u| u == 0).unwrap_or(bytes.len());
  core::str::from_utf8(&bytes[..terminal_position])
}

#[rustfmt::skip]
pub mod prelude {
  //! Lets you grab the entire crate as a single flat API.
  
  pub use crate::vk_platform::*;
  pub use crate::vk_version::*;
  pub use crate::handles::*;
  pub use crate::non_dispatchable_handles::*;
  pub use crate::enumerations::*;
  pub use crate::flag_bits::*;
  pub use crate::fn_types::*;
  pub use crate::fn_managers::*;
  pub use crate::fn_managers_rusty::*;
  pub use crate::structures::*;
  pub use crate::{
    VK_ATTACHMENT_UNUSED,
    VK_LOD_CLAMP_NONE,
    VK_QUEUE_FAMILY_IGNORED,
    VK_REMAINING_ARRAY_LAYERS,
    VK_REMAINING_MIP_LEVELS,
    VK_SUBPASS_EXTERNAL,
    VK_WHOLE_SIZE,
    VK_MAX_MEMORY_TYPES,
    VK_MAX_MEMORY_HEAPS,
    VK_MAX_PHYSICAL_DEVICE_NAME_SIZE,
    VK_UUID_SIZE,
    VK_MAX_EXTENSION_NAME_SIZE,
    VK_MAX_DESCRIPTION_SIZE,
  };
}

pub mod vk_platform;
use vk_platform::*;

mod vk_version;
pub use vk_version::*;

pub mod handles;
use handles::*;

pub mod non_dispatchable_handles;
use non_dispatchable_handles::*;

pub mod enumerations;
use enumerations::*;

pub mod flag_bits;
use flag_bits::*;

pub mod fn_managers;
use fn_managers::*;

pub mod fn_managers_rusty;
use fn_managers_rusty::*;

pub mod fn_types;
use fn_types::*;

pub mod structures;
use structures::*;

pub const VK_ATTACHMENT_UNUSED: u32 = !0;

pub const VK_LOD_CLAMP_NONE: f32 = 1000.0;

pub const VK_QUEUE_FAMILY_IGNORED: u32 = !0;

pub const VK_REMAINING_ARRAY_LAYERS: u32 = !0;

pub const VK_REMAINING_MIP_LEVELS: u32 = !0;

pub const VK_SUBPASS_EXTERNAL: u32 = !0;

pub const VK_WHOLE_SIZE: u64 = !0;

pub const VK_MAX_MEMORY_TYPES: usize = 32;

/// The maximum number of unique memory heaps.
///
/// Each heap supports one or more memory types.
pub const VK_MAX_MEMORY_HEAPS: usize = 16;

pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;

pub const VK_UUID_SIZE: usize = 16;

pub const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;

pub const VK_MAX_DESCRIPTION_SIZE: usize = 256;
