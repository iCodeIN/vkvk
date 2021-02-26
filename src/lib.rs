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
  pub use crate::structures::*;
  pub use crate::{
    VK_ATTACHMENT_UNUSED,
    VK_LOD_CLAMP_NONE,
    VK_QUEUE_FAMILY_IGNORED,
    VK_REMAINING_ARRAY_LAYERS,
    VK_REMAINING_MIP_LEVELS,
    VK_SUBPASS_EXTERNAL,
    VK_WHOLE_SIZE,
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

pub mod dynamic_link;
use dynamic_link::*;

pub mod enumerations;
use enumerations::*;

pub mod flag_bits;
use flag_bits::*;

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
/// Each heap supports 1 or more memory types.
pub const VK_MAX_MEMORY_HEAPS: usize = 16;

pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;

pub const VK_UUID_SIZE: usize = 16;

// TODO: PreInstanceFns

// TODO: InstanceFns

// TODO: PhysicalDeviceFns
