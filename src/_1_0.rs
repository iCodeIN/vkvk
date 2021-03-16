use crate::prelude::*;

pub mod vk_platform;

pub mod vk_version;

pub mod handles;

pub mod non_dispatchable_handles;

pub mod enumerations;

pub mod flag_bits;

pub mod fn_types;

pub mod structures;

pub mod unions;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkSampleMask(pub uint32_t);

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
