//! Handles are pointer-like values to objects that *can* accept commands.
//!
//! Because they're pointers, handles are either 32-bit or 64-bit depending on
//! the platform (the same size as `usize`).

use super::*;

/// [VkInstance](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkInstance.html)
///
/// Handle to an Instance (a connection to a Vulkan implementation).
///
/// * Parent Object: none
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_INSTANCE`]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkInstance(*mut c_void);
impl Default for VkInstance {
  fn default() -> Self {
    Self::null()
  }
}
impl VkInstance {
  pub const fn null() -> Self {
    Self(core::ptr::null_mut())
  }
  pub fn is_null(&self) -> bool {
    self.0.is_null()
  }
}

/// [VkPhysicalDevice](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevice.html)
///
/// Handle to a Physical Device.
///
/// * Parent Object: [`VkInstance`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_PHYSICAL_DEVICE`]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkPhysicalDevice(*mut c_void);
impl Default for VkPhysicalDevice {
  fn default() -> Self {
    Self::null()
  }
}
impl VkPhysicalDevice {
  pub const fn null() -> Self {
    Self(core::ptr::null_mut())
  }
  pub fn is_null(&self) -> bool {
    self.0.is_null()
  }
}

/// [VkDevice](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDevice.html)
///
/// Handle to a Device.
///
/// * Parent: [`VkPhysicalDevice`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_DEVICE`]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkDevice(*mut c_void);
impl Default for VkDevice {
  fn default() -> Self {
    Self::null()
  }
}
impl VkDevice {
  pub const fn null() -> Self {
    Self(core::ptr::null_mut())
  }
  pub fn is_null(&self) -> bool {
    self.0.is_null()
  }
}

/// [VkQueue](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueue.html)
///
/// Handle to a Queue.
///
/// * Parent: [`VkDevice`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_QUEUE`]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkQueue(*mut c_void);
impl Default for VkQueue {
  fn default() -> Self {
    Self::null()
  }
}
impl VkQueue {
  pub const fn null() -> Self {
    Self(core::ptr::null_mut())
  }
  pub fn is_null(&self) -> bool {
    self.0.is_null()
  }
}

/// [VkCommandBuffer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBuffer.html)
///
/// Handle to a command buffer object.
///
/// * Parent: [`VkCommandPool`]
/// * ObjectTypeEnum: [`VK_OBJECT_TYPE_COMMAND_BUFFER`]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkCommandBuffer(*mut c_void);
impl Default for VkCommandBuffer {
  fn default() -> Self {
    Self::null()
  }
}
impl VkCommandBuffer {
  pub const fn null() -> Self {
    Self(core::ptr::null_mut())
  }
  pub fn is_null(&self) -> bool {
    self.0.is_null()
  }
}
