//! Non-dispatchable Handles are identifiers for objects that *don't* accept
//! commands.
//!
//! A non-dispatchable handle is always a 64-bit value, regardless of platform.

use super::*;

/// [VkBuffer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBuffer.html)
///
/// * Parent: [`VkDevice`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_BUFFER`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkBuffer(pub u64);

/// [VkImage](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImage.html)
///
/// * Parent: [`VkDevice`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_IMAGE`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkImage(pub u64);

/// [VkSemaphore](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphore.html)
///
/// * Parent: [`VkDevice`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_SEMAPHORE`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkSemaphore(pub u64);

/// [VkCommandPool](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandPool.html)
///
/// * Parent: [`VkDevice`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_COMMAND_POOL`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkCommandPool(pub u64);

/// [VkFence](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFence.html)
///
/// * Parent: [`VkDevice`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_FENCE`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkFence(pub u64);

/// [VkDeviceMemory](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceMemory.html)
///
/// * Parent: [`VkDevice`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_DEVICE_MEMORY`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkDeviceMemory(pub u64);
