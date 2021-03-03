//! Vulkan types where one or more flags are bit-packed together.
//!
//! These types support the standard bitwise operations for enabling and
//! disabling specific bits:
//! * Add a new flag to a base value: `base | new`
//! * Remove a flag from a base value: `base & (!removal)`

use super::*;

vk_flag_bits! {
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
    //// Provided by `VK_EXT_transform_feedback`
    VK_ACCESS_TRANSFORM_FEEDBACK_WRITE_BIT_EXT = 0x02000000,
    //// Provided by `VK_EXT_transform_feedback`
    VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT = 0x04000000,
    //// Provided by `VK_EXT_transform_feedback`
    VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT = 0x08000000,
    //// Provided by `VK_EXT_conditional_rendering`
    VK_ACCESS_CONDITIONAL_RENDERING_READ_BIT_EXT = 0x00100000,
    //// Provided by `VK_EXT_blend_operation_advanced`
    VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT = 0x00080000,
    //// Provided by `VK_KHR_acceleration_structure`
    VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR = 0x00200000,
    //// Provided by `VK_KHR_acceleration_structure`
    VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR = 0x00400000,
    //// Provided by `VK_NV_shading_rate_image`
    VK_ACCESS_SHADING_RATE_IMAGE_READ_BIT_NV = 0x00800000,
    //// Provided by `VK_EXT_fragment_density_map`
    VK_ACCESS_FRAGMENT_DENSITY_MAP_READ_BIT_EXT = 0x01000000,
    //// Provided by `VK_NV_device_generated_commands`
    VK_ACCESS_COMMAND_PREPROCESS_READ_BIT_NV = 0x00020000,
    //// Provided by `VK_NV_device_generated_commands`
    VK_ACCESS_COMMAND_PREPROCESS_WRITE_BIT_NV = 0x00040000,
  }
}
//// Provided by `VK_KHR_synchronization2`
pub const VK_ACCESS_NONE_KHR: VkAccessFlags = VkAccessFlags(0);
//// Provided by `VK_NV_ray_tracing`
pub const VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_NV: VkAccessFlags = VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR;
//// Provided by `VK_NV_ray_tracing`
pub const VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_NV: VkAccessFlags = VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR;
//// Provided by `VK_KHR_fragment_shading_rate`
pub const VK_ACCESS_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR: VkAccessFlags = VK_ACCESS_SHADING_RATE_IMAGE_READ_BIT_NV;

vk_flag_bits! {
  /// [VkImageAspectFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageAspectFlagBits.html)
  VkImageAspectFlagBits = VkImageAspectFlags {
    VK_IMAGE_ASPECT_COLOR_BIT = (1<<0),
    VK_IMAGE_ASPECT_DEPTH_BIT = (1<<1),
    VK_IMAGE_ASPECT_STENCIL_BIT = (1<<2),
    VK_IMAGE_ASPECT_METADATA_BIT = (1<<3),
    //// Provided by `VK_VERSION_1_1`
    VK_IMAGE_ASPECT_PLANE_0_BIT = 0x00000010,
    //// Provided by `VK_VERSION_1_1`
    VK_IMAGE_ASPECT_PLANE_1_BIT = 0x00000020,
    //// Provided by `VK_VERSION_1_1`
    VK_IMAGE_ASPECT_PLANE_2_BIT = 0x00000040,
    //// Provided by `VK_EXT_image_drm_format_modifier`
    VK_IMAGE_ASPECT_MEMORY_PLANE_0_BIT_EXT = 0x00000080,
    //// Provided by `VK_EXT_image_drm_format_modifier`
    VK_IMAGE_ASPECT_MEMORY_PLANE_1_BIT_EXT = 0x00000100,
    //// Provided by `VK_EXT_image_drm_format_modifier`
    VK_IMAGE_ASPECT_MEMORY_PLANE_2_BIT_EXT = 0x00000200,
    //// Provided by `VK_EXT_image_drm_format_modifier`
    VK_IMAGE_ASPECT_MEMORY_PLANE_3_BIT_EXT = 0x00000400,
  }
}
//// Provided by `VK_KHR_sampler_ycbcr_conversion`
pub const VK_IMAGE_ASPECT_PLANE_0_BIT_KHR: VkImageAspectFlags = VK_IMAGE_ASPECT_PLANE_0_BIT;
//// Provided by `VK_KHR_sampler_ycbcr_conversion`
pub const VK_IMAGE_ASPECT_PLANE_1_BIT_KHR: VkImageAspectFlags = VK_IMAGE_ASPECT_PLANE_1_BIT;
//// Provided by `VK_KHR_sampler_ycbcr_conversion`
pub const VK_IMAGE_ASPECT_PLANE_2_BIT_KHR: VkImageAspectFlags = VK_IMAGE_ASPECT_PLANE_2_BIT;

vk_flag_bits! {
  /// [VkFormatFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFormatFeatureFlagBits.html)
  VkFormatFeatureFlagBits = VkFormatFeatureFlags {
    /// Format can be used for sampled images (SAMPLED_IMAGE and COMBINED_IMAGE_SAMPLER descriptor types)
    VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT = (1<<0),
    /// Format can be used for storage images (STORAGE_IMAGE descriptor type)
    VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT = (1<<1),
    /// Format supports atomic operations in case it is used for storage images
    VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT = (1<<2),
    /// Format can be used for uniform texel buffers (TBOs)
    VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT = (1<<3),
    /// Format can be used for storage texel buffers (IBOs)
    VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT = (1<<4),
    /// Format supports atomic operations in case it is used for storage texel buffers
    VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT = (1<<5),
    /// Format can be used for vertex buffers (VBOs)
    VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT = (1<<6),
    /// Format can be used for color attachment images
    VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT = (1<<7),
    /// Format supports blending in case it is used for color attachment images
    VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT = (1<<8),
    /// Format can be used for depth/stencil attachment images
    VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT = (1<<9),
    /// Format can be used as the source image of blits with vkCmdBlitImage
    VK_FORMAT_FEATURE_BLIT_SRC_BIT = (1<<10),
    /// Format can be used as the destination image of blits with vkCmdBlitImage
    VK_FORMAT_FEATURE_BLIT_DST_BIT = (1<<11),
    /// Format can be filtered with VK_FILTER_LINEAR when being sampled
    VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT = (1<<12),
    /// Provided by `VK_VERSION_1_1`
    VK_FORMAT_FEATURE_TRANSFER_SRC_BIT = 0x00004000,
    /// Provided by `VK_VERSION_1_1`
    VK_FORMAT_FEATURE_TRANSFER_DST_BIT = 0x00008000,
    /// Provided by `VK_VERSION_1_1`
    VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT = 0x00020000,
    /// Provided by `VK_VERSION_1_1`
    VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT = 0x00040000,
    /// Provided by `VK_VERSION_1_1`
    VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT = 0x00080000,
    /// Provided by `VK_VERSION_1_1`
    VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT = 0x00100000,
    /// Provided by `VK_VERSION_1_1`
    VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT = 0x00200000,
    /// Provided by `VK_VERSION_1_1`
    VK_FORMAT_FEATURE_DISJOINT_BIT = 0x00400000,
    /// Provided by `VK_VERSION_1_1`
    VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT = 0x00800000,
    /// Provided by `VK_VERSION_1_2`
    VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT = 0x00010000,
    /// Provided by `VK_IMG_filter_cubic`
    VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG = 0x00002000,
    /// Provided by `VK_KHR_acceleration_structure`
    VK_FORMAT_FEATURE_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR = 0x20000000,
    /// Provided by `VK_EXT_fragment_density_map`
    VK_FORMAT_FEATURE_FRAGMENT_DENSITY_MAP_BIT_EXT = 0x01000000,
    /// Provided by `VK_KHR_fragment_shading_rate`
    VK_FORMAT_FEATURE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 0x40000000,
  }
}
/// Provided by `VK_KHR_maintenance1`
pub const VK_FORMAT_FEATURE_TRANSFER_SRC_BIT_KHR: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_TRANSFER_SRC_BIT;
/// Provided by `VK_KHR_maintenance1`
pub const VK_FORMAT_FEATURE_TRANSFER_DST_BIT_KHR: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_TRANSFER_DST_BIT;
/// Provided by `VK_EXT_sampler_filter_minmax`
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT;
/// Provided by `VK_KHR_sampler_ycbcr_conversion`
pub const VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT_KHR: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT;
/// Provided by `VK_KHR_sampler_ycbcr_conversion`
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR: VkFormatFeatureFlagBits =
  VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT;
/// Provided by `VK_KHR_sampler_ycbcr_conversion`
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR: VkFormatFeatureFlagBits =
  VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT;
/// Provided by `VK_KHR_sampler_ycbcr_conversion`
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR: VkFormatFeatureFlagBits =
  VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT;
/// Provided by `VK_KHR_sampler_ycbcr_conversion`
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR: VkFormatFeatureFlagBits =
  VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT;
/// Provided by `VK_KHR_sampler_ycbcr_conversion`
pub const VK_FORMAT_FEATURE_DISJOINT_BIT_KHR: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_DISJOINT_BIT;
/// Provided by `VK_KHR_sampler_ycbcr_conversion`
pub const VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT_KHR: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT;
/// Provided by `VK_EXT_filter_cubic`
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG;

vk_flag_bits! {
  /// [VkImageCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageCreateFlagBits.html)
  VkImageCreateFlagBits = VkImageCreateFlags {
    /// Image should support sparse backing
    VK_IMAGE_CREATE_SPARSE_BINDING_BIT = (1<<0),
    /// Image should support sparse backing with partial residency
    VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT = (1<<1),
    /// Image should support constant data access to physical memory ranges mapped into multiple locations of sparse images
    VK_IMAGE_CREATE_SPARSE_ALIASED_BIT = (1<<2),
    /// Allows image views to have different format than the base image
    VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT = (1<<3),
    /// Allows creating image views with cube type from the created image
    VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT = (1<<4),
    /// Provided by `VK_VERSION_1_1`
    VK_IMAGE_CREATE_ALIAS_BIT = 0x00000400,
    /// Provided by `VK_VERSION_1_1`
    VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT = 0x00000040,
    /// Provided by `VK_VERSION_1_1`
    VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT = 0x00000020,
    /// Provided by `VK_VERSION_1_1`
    VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT = 0x00000080,
    /// Provided by `VK_VERSION_1_1`
    VK_IMAGE_CREATE_EXTENDED_USAGE_BIT = 0x00000100,
    /// Provided by `VK_VERSION_1_1`
    VK_IMAGE_CREATE_PROTECTED_BIT = 0x00000800,
    /// Provided by `VK_VERSION_1_1`
    VK_IMAGE_CREATE_DISJOINT_BIT = 0x00000200,
    /// Provided by `VK_NV_corner_sampled_image`
    VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV = 0x00002000,
    /// Provided by `VK_EXT_sample_locations`
    VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT = 0x00001000,
    /// Provided by `VK_EXT_fragment_density_map`
    VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT = 0x00004000,
  }
}

/// Provided by `VK_KHR_device_group` with `VK_KHR_bind_memory2`
pub const VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR: VkImageCreateFlagBits = VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT;
/// Provided by `VK_KHR_maintenance1`
pub const VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT_KHR: VkImageCreateFlagBits = VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT;
/// Provided by `VK_KHR_maintenance2`
pub const VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR: VkImageCreateFlagBits = VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT;
/// Provided by `VK_KHR_maintenance2`
pub const VK_IMAGE_CREATE_EXTENDED_USAGE_BIT_KHR: VkImageCreateFlagBits = VK_IMAGE_CREATE_EXTENDED_USAGE_BIT;
/// Provided by `VK_KHR_sampler_ycbcr_conversion`
pub const VK_IMAGE_CREATE_DISJOINT_BIT_KHR: VkImageCreateFlagBits = VK_IMAGE_CREATE_DISJOINT_BIT;
/// Provided by `VK_KHR_bind_memory2`
pub const VK_IMAGE_CREATE_ALIAS_BIT_KHR: VkImageCreateFlagBits = VK_IMAGE_CREATE_ALIAS_BIT;

vk_flag_bits! {
  /// [VkImageUsageFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageUsageFlagBits.html)
  VkImageUsageFlagBits = VkImageUsageFlags {
    /// Can be used as a source of transfer operations
    VK_IMAGE_USAGE_TRANSFER_SRC_BIT = (1<<0),
    /// Can be used as a destination of transfer operations
    VK_IMAGE_USAGE_TRANSFER_DST_BIT = (1<<1),
    /// Can be sampled from (SAMPLED_IMAGE and COMBINED_IMAGE_SAMPLER descriptor types)
    VK_IMAGE_USAGE_SAMPLED_BIT = (1<<2),
    /// Can be used as storage image (STORAGE_IMAGE descriptor type)
    VK_IMAGE_USAGE_STORAGE_BIT = (1<<3),
    /// Can be used as framebuffer color attachment
    VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT = (1<<4),
    /// Can be used as framebuffer depth/stencil attachment
    VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT = (1<<5),
    /// Image data not needed outside of rendering
    VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT = (1<<6),
    /// Can be used as framebuffer input attachment
    VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT = (1<<7),
    /// Provided by `VK_NV_shading_rate_image`
    VK_IMAGE_USAGE_SHADING_RATE_IMAGE_BIT_NV = 0x00000100,
    /// Provided by `VK_EXT_fragment_density_map`
    VK_IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT = 0x00000200,
  }
}
/// Provided by `VK_KHR_fragment_shading_rate`
pub const VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: VkImageUsageFlagBits = VK_IMAGE_USAGE_SHADING_RATE_IMAGE_BIT_NV;

vk_flag_bits! {
  /// [VkInstanceCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkInstanceCreateFlags.html)
  ///
  /// currently reserved for future use.
  VkInstanceCreateFlagBits = VkInstanceCreateFlags {}
}

vk_flag_bits! {
  /// [VkMemoryHeapFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryHeapFlagBits.html)
  VkMemoryHeapFlagBits = VkMemoryHeapFlags {
    /// If set, heap represents device memory
    VK_MEMORY_HEAP_DEVICE_LOCAL_BIT = (1<<0),
    /// Provided by `VK_VERSION_1_1`
    VK_MEMORY_HEAP_MULTI_INSTANCE_BIT = 0x00000002,
  }
}
/// Provided by `VK_KHR_device_group_creation`
pub const VK_MEMORY_HEAP_MULTI_INSTANCE_BIT_KHR: VkMemoryHeapFlags = VK_MEMORY_HEAP_MULTI_INSTANCE_BIT;

vk_flag_bits! {
  /// [VkMemoryPropertyFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryPropertyFlagBits.html)
  VkMemoryPropertyFlagBits = VkMemoryPropertyFlags {
    /// If otherwise stated, then allocate memory on device
    VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT = (1<<0),
    /// Memory is mappable by host
    VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT = (1<<1),
    /// Memory will have i/o coherency. If not set, application may need to use vkFlushMappedMemoryRanges and vkInvalidateMappedMemoryRanges to flush/invalidate host cache
    VK_MEMORY_PROPERTY_HOST_COHERENT_BIT = (1<<2),
    /// Memory will be cached by the host
    VK_MEMORY_PROPERTY_HOST_CACHED_BIT = (1<<3),
    /// Memory may be allocated by the driver when it is required
    VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT = (1<<4),
    /// Provided by `VK_VERSION_1_1`
    VK_MEMORY_PROPERTY_PROTECTED_BIT = 0x00000020,
    /// Provided by `VK_AMD_device_coherent_memory`
    VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD = 0x00000040,
    /// Provided by `VK_AMD_device_coherent_memory`
    VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD = 0x00000080,
  }
}

vk_flag_bits! {
  /// [VkQueueFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueFlagBits.html)
  VkQueueFlagBits = VkQueueFlags {
    /// Queue supports graphics operations
    VK_QUEUE_GRAPHICS_BIT = (1<<0),
    /// Queue supports compute operations
    VK_QUEUE_COMPUTE_BIT = (1<<1),
    /// Queue supports transfer operations
    VK_QUEUE_TRANSFER_BIT = (1<<2),
    /// Queue supports sparse resource memory management operations
    VK_QUEUE_SPARSE_BINDING_BIT = (1<<3),
    /// Provided by `VK_VERSION_1_1`
    VK_QUEUE_PROTECTED_BIT = 0x00000010,
  }
}

vk_flag_bits! {
  /// [VkSampleCountFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSampleCountFlagBits.html)
  VkSampleCountFlagBits = VkSampleCountFlags {
    /// Sample count 1 supported
    VK_SAMPLE_COUNT_1_BIT = (1<<0),
    /// Sample count 2 supported
    VK_SAMPLE_COUNT_2_BIT = (1<<1),
    /// Sample count 4 supported
    VK_SAMPLE_COUNT_4_BIT = (1<<2),
    /// Sample count 8 supported
    VK_SAMPLE_COUNT_8_BIT = (1<<3),
    /// Sample count 16 supported
    VK_SAMPLE_COUNT_16_BIT = (1<<4),
    /// Sample count 32 supported
    VK_SAMPLE_COUNT_32_BIT = (1<<5),
    /// Sample count 64 supported
    VK_SAMPLE_COUNT_64_BIT = (1<<6),
  }
}

vk_flag_bits! {
  /// [VkDeviceCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceCreateFlags.html)
  ///
  /// currently reserved for future use.
  VkDeviceCreateFlagBits = VkDeviceCreateFlags {}
}

vk_flag_bits! {
  /// [VkDeviceQueueCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceQueueCreateFlagBits.html)
  ///
  /// currently reserved for future use.
  VkDeviceQueueCreateFlagBits = VkDeviceQueueCreateFlags {}
}

vk_flag_bits! {
  /// [VkPipelineStageFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineStageFlagBits.html)
  VkPipelineStageFlagBits = VkPipelineStageFlags {
    /// Before subsequent commands are processed
    VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT = (1<<0),
    /// Draw/DispatchIndirect command fetch
    VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT = (1<<1),
    /// Vertex/index fetch
    VK_PIPELINE_STAGE_VERTEX_INPUT_BIT = (1<<2),
    /// Vertex shading
    VK_PIPELINE_STAGE_VERTEX_SHADER_BIT = (1<<3),
    /// Tessellation control shading
    VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT = (1<<4),
    /// Tessellation evaluation shading
    VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT = (1<<5),
    /// Geometry shading
    VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT = (1<<6),
    /// Fragment shading
    VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT = (1<<7),
    /// Early fragment (depth and stencil) tests
    VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT = (1<<8),
    /// Late fragment (depth and stencil) tests
    VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT = (1<<9),
    /// Color attachment writes
    VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT = (1<<10),
    /// Compute shading
    VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT = (1<<11),
    /// Transfer/copy operations
    VK_PIPELINE_STAGE_TRANSFER_BIT = (1<<12),
    /// After previous commands have completed
    VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT = (1<<13),
    /// Indicates host (CPU) is a source/sink of the dependency
    VK_PIPELINE_STAGE_HOST_BIT = (1<<14),
    /// All stages of the graphics pipeline
    VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT = (1<<15),
    /// All stages supported on the queue
    VK_PIPELINE_STAGE_ALL_COMMANDS_BIT = (1<<16),
    // Provided by `VK_EXT_transform_feedback`
    VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT = 0x01000000,
    // Provided by `VK_EXT_conditional_rendering`
    VK_PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT = 0x00040000,
    // Provided by `VK_KHR_acceleration_structure`
    VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR = 0x02000000,
    // Provided by `VK_KHR_ray_tracing_pipeline`
    VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_KHR = 0x00200000,
    // Provided by `VK_NV_shading_rate_image`
    VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV = 0x00400000,
    // Provided by `VK_NV_mesh_shader`
    VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV = 0x00080000,
    // Provided by `VK_NV_mesh_shader`
    VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV = 0x00100000,
    // Provided by `VK_EXT_fragment_density_map`
    VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT = 0x00800000,
    // Provided by `VK_NV_device_generated_commands`
    VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_NV = 0x00020000,
    // Provided by `VK_KHR_synchronization2`
    VK_PIPELINE_STAGE_NONE_KHR = 0,
  }
}
// Provided by `VK_NV_ray_tracing`
pub const VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_NV: VkPipelineStageFlagBits = VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_KHR;
// Provided by `VK_NV_ray_tracing`
pub const VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_NV: VkPipelineStageFlagBits = VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR;
// Provided by `VK_KHR_fragment_shading_rate`
pub const VK_PIPELINE_STAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: VkPipelineStageFlagBits = VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV;
