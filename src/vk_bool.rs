use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkBool32(pub uint32_t);

impl From<bool> for VkBool32 {
  fn from(b: bool) -> Self {
    Self(b as _)
  }
}

impl From<VkBool32> for bool {
  fn from(vk_b: VkBool32) -> Self {
    vk_b.0 != 0
  }
}

pub const VK_FALSE: VkBool32 = VkBool32(0);

pub const VK_TRUE: VkBool32 = VkBool32(1);
