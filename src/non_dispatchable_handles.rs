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
