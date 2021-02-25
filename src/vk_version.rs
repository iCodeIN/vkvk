use super::*;

/// Provides simple access to a vulkan version value.
///
/// This is **not** an official Vulkan type, it's just a Rusty helper type.
/// Within official Vulkan, version numbers are just `u32` values with a special
/// bit encoding.
///
/// [Spec 39.2.1: Version
/// Numbers](https://renderdoc.org/vkspec_chunked/chap40.html#extendingvulkan-coreversions-versionnumbers)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct VulkanVersion(pub u32);
impl VulkanVersion {
  pub const fn major(self) -> u32 {
    self.0 >> 22
  }
  pub const fn minor(self) -> u32 {
    (self.0 >> 12) & 0x3ff
  }
  pub const fn patch(self) -> u32 {
    self.0 & 0xfff
  }
  pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
    Self((major << 22) | (minor << 22) | patch)
  }
  pub const _1_0: VulkanVersion = VulkanVersion::new(1, 0, 0);
  pub const _1_1: VulkanVersion = VulkanVersion::new(1, 1, 0);
  pub const _1_2: VulkanVersion = VulkanVersion::new(1, 2, 0);
  pub const HEADER: VulkanVersion = VulkanVersion::new(1, 2, 167);
}
impl core::fmt::Debug for VulkanVersion {
  /// 
  /// * Standard: "VulkanVersion({major}.{minor}.{patch})"
  /// * Alternate: "VulkanVersion({self.0})"
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if f.alternate() {
      write!(f, "VulkanVersion({})", self.0)
    } else {
      write!(f, "VulkanVersion({major}.{minor}.{patch})", major = self.major(), minor = self.minor(), patch = self.patch(),)
    }
  }
}
