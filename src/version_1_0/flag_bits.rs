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
    /// Provided by `VK_EXT_transform_feedback`
    VK_ACCESS_TRANSFORM_FEEDBACK_WRITE_BIT_EXT = 0x02000000,
    /// Provided by `VK_EXT_transform_feedback`
    VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT = 0x04000000,
    /// Provided by `VK_EXT_transform_feedback`
    VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT = 0x08000000,
    /// Provided by `VK_EXT_conditional_rendering`
    VK_ACCESS_CONDITIONAL_RENDERING_READ_BIT_EXT = 0x00100000,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT = 0x00080000,
    /// Provided by `VK_KHR_acceleration_structure`
    VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR = 0x00200000,
    /// Provided by `VK_KHR_acceleration_structure`
    VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR = 0x00400000,
    /// Provided by `VK_NV_shading_rate_image`
    VK_ACCESS_SHADING_RATE_IMAGE_READ_BIT_NV = 0x00800000,
    /// Provided by `VK_EXT_fragment_density_map`
    VK_ACCESS_FRAGMENT_DENSITY_MAP_READ_BIT_EXT = 0x01000000,
    /// Provided by `VK_NV_device_generated_commands`
    VK_ACCESS_COMMAND_PREPROCESS_READ_BIT_NV = 0x00020000,
    /// Provided by `VK_NV_device_generated_commands`
    VK_ACCESS_COMMAND_PREPROCESS_WRITE_BIT_NV = 0x00040000,
  }
}
/// Provided by `VK_KHR_synchronization2`
pub const VK_ACCESS_NONE_KHR: VkAccessFlagBits = VkAccessFlagBits(0);
/// Provided by `VK_NV_ray_tracing`
pub const VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_NV: VkAccessFlagBits = VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR;
/// Provided by `VK_NV_ray_tracing`
pub const VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_NV: VkAccessFlagBits = VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR;
/// Provided by `VK_KHR_fragment_shading_rate`
pub const VK_ACCESS_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR: VkAccessFlagBits = VK_ACCESS_SHADING_RATE_IMAGE_READ_BIT_NV;

flag_bits! {
  /// [VkImageAspectFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageAspectFlagBits.html)
  VkImageAspectFlagBits = VkImageAspectFlags {
    VK_IMAGE_ASPECT_COLOR_BIT = (1<<0),
    VK_IMAGE_ASPECT_DEPTH_BIT = (1<<1),
    VK_IMAGE_ASPECT_STENCIL_BIT = (1<<2),
    VK_IMAGE_ASPECT_METADATA_BIT = (1<<3),
    /// Provided by `VK_VERSION_1_1`
    VK_IMAGE_ASPECT_PLANE_0_BIT = 0x00000010,
    /// Provided by `VK_VERSION_1_1`
    VK_IMAGE_ASPECT_PLANE_1_BIT = 0x00000020,
    /// Provided by `VK_VERSION_1_1`
    VK_IMAGE_ASPECT_PLANE_2_BIT = 0x00000040,
    /// Provided by `VK_EXT_image_drm_format_modifier`
    VK_IMAGE_ASPECT_MEMORY_PLANE_0_BIT_EXT = 0x00000080,
    /// Provided by `VK_EXT_image_drm_format_modifier`
    VK_IMAGE_ASPECT_MEMORY_PLANE_1_BIT_EXT = 0x00000100,
    /// Provided by `VK_EXT_image_drm_format_modifier`
    VK_IMAGE_ASPECT_MEMORY_PLANE_2_BIT_EXT = 0x00000200,
    /// Provided by `VK_EXT_image_drm_format_modifier`
    VK_IMAGE_ASPECT_MEMORY_PLANE_3_BIT_EXT = 0x00000400,
  }
}
/// Provided by `VK_KHR_sampler_ycbcr_conversion`
pub const VK_IMAGE_ASPECT_PLANE_0_BIT_KHR: VkImageAspectFlagBits = VK_IMAGE_ASPECT_PLANE_0_BIT;
/// Provided by `VK_KHR_sampler_ycbcr_conversion`
pub const VK_IMAGE_ASPECT_PLANE_1_BIT_KHR: VkImageAspectFlagBits = VK_IMAGE_ASPECT_PLANE_1_BIT;
/// Provided by `VK_KHR_sampler_ycbcr_conversion`
pub const VK_IMAGE_ASPECT_PLANE_2_BIT_KHR: VkImageAspectFlagBits = VK_IMAGE_ASPECT_PLANE_2_BIT;
