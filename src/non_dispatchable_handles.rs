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

/// [VkEvent](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkEvent.html)
///
/// * Parent: [`VkDevice`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_EVENT`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkEvent(pub u64);

/// [VkQueryPool](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPool.html)
///
/// * Parent: [`VkDevice`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_QUERY_POOL`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkQueryPool(pub u64);

/// [VkBufferView](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferView.html)
///
/// * Parent: [`VkDevice`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_BUFFER_VIEW`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkBufferView(pub u64);

/// [VkImageView](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageView.html)
///
/// * Parent: [`VkDevice`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_IMAGE_VIEW`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkImageView(pub u64);

/// [VkShaderModule](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderModule.html)
///
/// * Parent: [`VkDevice`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_SHADER_MODULE`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkShaderModule(pub u64);

/// [VkPipelineCache](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCache.html)
///
/// * Parent: [`VkDevice`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_PIPELINE_CACHE`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkPipelineCache(pub u64);

/// [VkPipeline](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipeline.html)
///
/// * Parent: [`VkDevice`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_PIPELINE`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkPipeline(pub u64);

/// [VkPipelineLayout](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineLayout.html)
///
/// * Parent: [`VkDevice`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_PIPELINE_LAYOUT`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkPipelineLayout(pub u64);

/// [VkRenderPass](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPass.html)
///
/// * Parent: [`VkDevice`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_RENDER_PASS`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkRenderPass(pub u64);

/// [VkDescriptorSetLayout](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayout.html)
///
/// * Parent: [`VkDevice`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkDescriptorSetLayout(pub u64);

/// [VkSampler](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSampler.html)
///
/// * Parent: [`VkDevice`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_SAMPLER`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkSampler(pub u64);

/// [VkDescriptorPool](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorPool.html)
///
/// * Parent: [`VkDevice`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_DESCRIPTOR_POOL`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkDescriptorPool(pub u64);

/// [VkDescriptorSet](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSet.html)
///
/// * Parent: [`VkDescriptorPool`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_DESCRIPTOR_SET`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkDescriptorSet(pub u64);

/// [VkFramebuffer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebuffer.html)
///
/// * Parent: [`VkDevice`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_FRAMEBUFFER`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkFramebuffer(pub u64);
