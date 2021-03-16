#![allow(non_snake_case)]

use super::*;

/// [VkClearColorValue](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearColorValue.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub union VkClearColorValue {
  pub float32: [float; 4],
  pub int32: [int32_t; 4],
  pub uint32: [uint32_t; 4],
}

/// [VkClearValue](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearValue.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub union VkClearValue {
  pub color: VkClearColorValue,
  pub depthStencil: VkClearDepthStencilValue,
}
