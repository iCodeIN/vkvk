use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkDeviceAddress(pub uint64_t);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkDeviceSize(pub uint64_t);

vk_struct! {
  /// [VkExtent2D](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExtent2D.html)
  VkExtent2D {
    width: uint32_t,
    height: uint32_t,
  }
}

vk_struct! {
  /// [VkExtent3D](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExtent3D.html)
  VkExtent3D {
    width: uint32_t,
    height: uint32_t,
    depth: uint32_t,
  }
}

vk_struct! {
  /// [VkOffset2D](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkOffset2D.html)
  VkOffset2D {
    x: int32_t,
    y: int32_t,
  }
}

vk_struct! {
  /// [VkOffset3D](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkOffset3D.html)
  VkOffset3D {
    x: int32_t,
    y: int32_t,
    z: int32_t,
  }
}

vk_struct! {
  /// [VkRect2D](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRect2D.html)
  VkRect2D {
    offset: VkOffset2D,
    extent: VkExtent2D,
  }
}
