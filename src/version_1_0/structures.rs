use super::*;

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

// TODO: VkBaseInStructure

// TODO: VkBaseOutStructure

// TODO: VkBufferMemoryBarrier

// TODO: VkDispatchIndirectCommand

// TODO: VkDrawIndexedIndirectCommand

// TODO: VkDrawIndirectCommand

// TODO: VkImageMemoryBarrier

// TODO: VkMemoryBarrier

// TODO: VkObjectType

// TODO: VkVendorId
