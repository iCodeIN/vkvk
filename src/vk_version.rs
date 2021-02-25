use super::*;

/// A [`u32`] that encodes a vulkan API version.
///
/// This is **not** an official Vulkan type, it's just a Rusty helper type.
/// Within official Vulkan, version numbers are just plain `u32` values with a
/// special bit encoding.
///
/// Vulkan versions *roughly* follow SemVer.
///
/// * [Spec 39.2.1: Version Numbers][version-numbers]
/// * [Spec 39.6.1: Core Versions][core-versions]
///
/// [version-numbers]: https://renderdoc.org/vkspec_chunked/chap40.html#extendingvulkan-coreversions-versionnumbers
///
/// [core-versions]: https://renderdoc.org/vkspec_chunked/chap40.html#extendingvulkan-compatibility-coreversions
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct VulkanVersion(pub u32);
impl VulkanVersion {
  /// Gets out the major version.
  pub const fn major(self) -> u32 {
    self.0 >> 22
  }

  /// Gets out the minor version.
  pub const fn minor(self) -> u32 {
    (self.0 >> 12) & 0x3ff
  }

  /// Gets out the patch version.
  pub const fn patch(self) -> u32 {
    self.0 & 0xfff
  }

  /// Constructs a new version value.
  pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
    Self((major << 22) | (minor << 22) | patch)
  }

  /// Vulkan 1.0
  pub const _1_0: VulkanVersion = VulkanVersion::new(1, 0, 0);

  /// Vulkan 1.1
  pub const _1_1: VulkanVersion = VulkanVersion::new(1, 1, 0);

  /// Vulkan 1.2
  pub const _1_2: VulkanVersion = VulkanVersion::new(1, 2, 0);

  /// The header version used to construct this crate.
  ///
  /// This value can change in any new release of the crate.
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
