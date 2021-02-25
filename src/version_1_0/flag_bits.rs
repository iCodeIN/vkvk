use super::*;

flag_bits! {
  /// [VkAccessFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccessFlagBits.html)
  VkAccessFlagBits = VkAccessFlags {
    /// Controls coherency of indirect command reads
    VK_ACCESS_INDIRECT_COMMAND_READ_BIT = (1<<0),
    /// Controls coherency of index reads
    VK_ACCESS_INDEX_READ_BIT = (1<<1),
    /// Controls coherency of vertex attribute reads
    VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT = (1<<2),
    /// Controls coherency of uniform buffer reads
    VK_ACCESS_UNIFORM_READ_BIT = (1<<3),
    /// Controls coherency of input attachment reads
    VK_ACCESS_INPUT_ATTACHMENT_READ_BIT = (1<<4),
    /// Controls coherency of shader reads
    VK_ACCESS_SHADER_READ_BIT = (1<<5),
    /// Controls coherency of shader writes
    VK_ACCESS_SHADER_WRITE_BIT = (1<<6),
    /// Controls coherency of color attachment reads
    VK_ACCESS_COLOR_ATTACHMENT_READ_BIT = (1<<7),
    /// Controls coherency of color attachment writes
    VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT = (1<<8),
    /// Controls coherency of depth/stencil attachment reads
    VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT = (1<<9),
    /// Controls coherency of depth/stencil attachment writes
    VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = (1<<10),
    /// Controls coherency of transfer reads
    VK_ACCESS_TRANSFER_READ_BIT = (1<<11),
    /// Controls coherency of transfer writes
    VK_ACCESS_TRANSFER_WRITE_BIT = (1<<12),
    /// Controls coherency of host reads
    VK_ACCESS_HOST_READ_BIT = (1<<13),
    /// Controls coherency of host writes
    VK_ACCESS_HOST_WRITE_BIT = (1<<14),
    /// Controls coherency of memory reads
    VK_ACCESS_MEMORY_READ_BIT = (1<<15),
    /// Controls coherency of memory writes
    VK_ACCESS_MEMORY_WRITE_BIT = (1<<16),
  }
}

flag_bits! {
  /// [VkImageAspectFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageAspectFlagBits.html)
  VkImageAspectFlagBits = VkImageAspectFlags {
    VK_IMAGE_ASPECT_COLOR_BIT = (1<<0),
    VK_IMAGE_ASPECT_DEPTH_BIT = (1<<1),
    VK_IMAGE_ASPECT_STENCIL_BIT = (1<<2),
    VK_IMAGE_ASPECT_METADATA_BIT = (1<<3),
  }
}
