#![allow(non_snake_case)]

use super::*;

use core::mem::zeroed;

/// [VkDeviceAddress](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceAddress.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkDeviceAddress(pub uint64_t);

/// [VkDeviceSize](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceSize.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkDeviceSize(pub uint64_t);

/// [VkExtent2D](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExtent2D.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkExtent2D {
  pub width: uint32_t,
  pub height: uint32_t,
}

/// [VkExtent3D](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExtent3D.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkExtent3D {
  pub width: uint32_t,
  pub height: uint32_t,
  pub depth: uint32_t,
}

/// [VkOffset2D](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkOffset2D.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkOffset2D {
  pub x: int32_t,
  pub y: int32_t,
}

/// [VkOffset3D](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkOffset3D.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkOffset3D {
  pub x: int32_t,
  pub y: int32_t,
  pub z: int32_t,
}

/// [VkRect2D](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRect2D.html)
///
/// The `extent` forms the *exclusive* edges of the rectangle.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkRect2D {
  pub offset: VkOffset2D,
  pub extent: VkExtent2D,
}

/// [VkBaseInStructure](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBaseInStructure.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkBaseInStructure {
  pub sType: VkStructureType,
  pub pNext: *const VkBaseInStructure,
}
impl Default for VkBaseInStructure {
  fn default() -> Self {
    unsafe { zeroed() }
  }
}

/// [VkBaseOutStructure](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBaseOutStructure.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkBaseOutStructure {
  pub sType: VkStructureType,
  pub pNext: *mut VkBaseOutStructure,
}
impl Default for VkBaseOutStructure {
  fn default() -> Self {
    unsafe { zeroed() }
  }
}

/// [VkBufferMemoryBarrier](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferMemoryBarrier.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkBufferMemoryBarrier {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER`]
  pub sType: VkStructureType,
  ///
  /// * **Optional:** true
  pub pNext: *const c_void,
  /// Memory accesses from the source of the dependency to synchronize
  /// * **No Auto-validity:** true
  pub srcAccessMask: VkAccessFlags,
  /// Memory accesses from the destination of the dependency to synchronize
  /// * **No Auto-validity:** true
  pub dstAccessMask: VkAccessFlags,
  /// Queue family to transition ownership from
  pub srcQueueFamilyIndex: uint32_t,
  /// Queue family to transition ownership to
  pub dstQueueFamilyIndex: uint32_t,
  /// Buffer to sync
  pub buffer: VkBuffer,
  /// Offset within the buffer to sync
  pub offset: VkDeviceSize,
  /// Amount of bytes to sync
  pub size: VkDeviceSize,
}
impl Default for VkBufferMemoryBarrier {
  fn default() -> Self {
    VkBufferMemoryBarrier { sType: VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER, ..unsafe { zeroed() } }
  }
}

/// [VkDispatchIndirectCommand](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDispatchIndirectCommand.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkDispatchIndirectCommand {
  ///
  /// * **No Auto-validity:** true
  pub x: uint32_t,
  ///
  /// * **No Auto-validity:** true
  pub y: uint32_t,
  ///
  /// * **No Auto-validity:** true
  pub z: uint32_t,
}

/// [VkDrawIndexedIndirectCommand](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDrawIndexedIndirectCommand.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkDrawIndexedIndirectCommand {
  pub indexCount: uint32_t,
  pub instanceCount: uint32_t,
  pub firstIndex: uint32_t,
  pub vertexOffset: int32_t,
  ///
  /// * **No Auto-validity:** true
  pub firstInstance: uint32_t,
}

/// [VkDrawIndirectCommand](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDrawIndirectCommand.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkDrawIndirectCommand {
  pub vertexCount: uint32_t,
  pub instanceCount: uint32_t,
  pub firstVertex: uint32_t,
  ///
  /// * **No Auto-validity:** true
  pub firstInstance: uint32_t,
}

/// [VkImageMemoryBarrier](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageMemoryBarrier.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkImageMemoryBarrier {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER`]
  pub sType: VkStructureType,
  ///
  /// * **Optional:** true
  pub pNext: *const c_void,
  /// Memory accesses from the source of the dependency to synchronize
  /// * **No Auto-validity:** true
  pub srcAccessMask: VkAccessFlags,
  /// Memory accesses from the destination of the dependency to synchronize
  /// * **No Auto-validity:** true
  pub dstAccessMask: VkAccessFlags,
  /// Current layout of the image
  pub oldLayout: VkImageLayout,
  /// New layout to transition the image to
  pub newLayout: VkImageLayout,
  /// Queue family to transition ownership from
  pub srcQueueFamilyIndex: uint32_t,
  /// Queue family to transition ownership to
  pub dstQueueFamilyIndex: uint32_t,
  /// Image to sync
  pub image: VkImage,
  /// Subresource range to sync
  pub subresourceRange: VkImageSubresourceRange,
}
impl Default for VkImageMemoryBarrier {
  fn default() -> Self {
    VkImageMemoryBarrier { sType: VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER, ..unsafe { zeroed() } }
  }
}

/// [VkMemoryBarrier](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryBarrier.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkMemoryBarrier {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_MEMORY_BARRIER`]
  pub sType: VkStructureType,
  ///
  /// * **Optional:** true
  pub pNext: *const c_void,
  /// Memory accesses from the source of the dependency to synchronize
  /// * **Optional:** true
  pub srcAccessMask: VkAccessFlags,
  /// Memory accesses from the destination of the dependency to synchronize
  /// * **Optional:** true
  pub dstAccessMask: VkAccessFlags,
}
impl Default for VkMemoryBarrier {
  fn default() -> Self {
    VkMemoryBarrier { sType: VK_STRUCTURE_TYPE_MEMORY_BARRIER, ..unsafe { zeroed() } }
  }
}

/// [VkImageSubresourceRange](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSubresourceRange.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkImageSubresourceRange {
  pub aspectMask: VkImageAspectFlags,
  pub baseMipLevel: uint32_t,
  pub levelCount: uint32_t,
  pub baseArrayLayer: uint32_t,
  pub layerCount: uint32_t,
}
