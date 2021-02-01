use super::*;

macro_rules! flag_bits {
  // with alias
  (
    $(#[$s_meta:meta])*
    $bits_name:ident = $flags_name:ident {
      $($(#[$c_meta:meta])* $i:ident = $val:expr),*
      $(,)?
    }
  ) => {
    pub type $flags_name = $bits_name;
    flag_bits!($(#[$s_meta])* $bits_name {
      $($(#[$c_meta])* $i = $val),*
    });
  };
  // no alias
  (
    $(#[$s_meta:meta])*
    $bits_name:ident {
      $($(#[$c_meta:meta])* $i:ident = $val:expr),*
      $(,)?
    }
  ) => {
    #[derive(PartialEq, Eq, Hash)]
    #[repr(transparent)]
    $(#[$s_meta])*
    pub struct $bits_name(pub u32);
    #[cfg(feature = "impl_flag_bits_precise_debug")]
    impl core::fmt::Debug for $bits_name {
      fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut printed_yet = false;
        f.write_str(concat!(stringify!($bits_name)," {"))?;
        $(if self.0 & $val > 0 {
          if printed_yet {
            f.write_str(", ")?;
          }
          f.write_str(stringify!($i))?;
          printed_yet = true;
        })*
        f.write_str(" }")
      }
    }
    impl Copy for $bits_name {}
    impl Clone for $bits_name {
      fn clone(&self) -> Self {
        *self
      }
    }
    impl Default for $bits_name {
      fn default() -> Self {
        Self(0)
      }
    }
    impl core::ops::BitAnd for $bits_name {
      type Output = Self;
      fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
      }
    }
    impl core::ops::BitAndAssign for $bits_name {
      fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
      }
    }
    impl core::ops::BitOr for $bits_name {
      type Output = Self;
      fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
      }
    }
    impl core::ops::BitOrAssign for $bits_name {
      fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
      }
    }
    impl core::ops::BitXor for $bits_name {
      type Output = Self;
      fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
      }
    }
    impl core::ops::BitXorAssign for $bits_name {
      fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
      }
    }
    impl core::ops::Not for $bits_name {
      type Output = Self;
      fn not(self) -> Self::Output {
        Self(!self.0)
      }
    }
    $( $(#[$c_meta])* pub const $i: $bits_name = $bits_name($val); )*
  };
}

flag_bits!(
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
  }
);

flag_bits!(
  /// [VkCullModeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCullModeFlagBits.html)
  VkCullModeFlagBits = VkCullModeFlags {
    VK_CULL_MODE_NONE = (1<<0),
    VK_CULL_MODE_FRONT_BIT = (1<<0),
    VK_CULL_MODE_BACK_BIT = (1<<1),
  }
);
pub const VK_CULL_MODE_FRONT_AND_BACK: VkCullModeFlagBits = VkCullModeFlagBits(0x00000003);

flag_bits!(
  /// [VkRenderPassCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassCreateFlagBits.html)
  VkRenderPassCreateFlagBits = VkRenderPassCreateFlags {}
);

flag_bits!(
  /// [VkDeviceQueueCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceQueueCreateFlagBits.html)
  VkDeviceQueueCreateFlagBits = VkDeviceQueueCreateFlags {}
);

flag_bits!(
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
  }
);

flag_bits!(
  /// [VkMemoryHeapFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryHeapFlagBits.html)
  VkMemoryHeapFlagBits = VkMemoryHeapFlags {
    /// If set, heap represents device memory
    VK_MEMORY_HEAP_DEVICE_LOCAL_BIT = (1<<0),
  }
);

flag_bits!(
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
);

flag_bits!(
  /// [VkBufferUsageFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferUsageFlagBits.html)
  VkBufferUsageFlagBits = VkBufferUsageFlags {
    /// Can be used as a source of transfer operations
    VK_BUFFER_USAGE_TRANSFER_SRC_BIT = (1<<0),
    /// Can be used as a destination of transfer operations
    VK_BUFFER_USAGE_TRANSFER_DST_BIT = (1<<1),
    /// Can be used as TBO
    VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT = (1<<2),
    /// Can be used as IBO
    VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT = (1<<3),
    /// Can be used as UBO
    VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT = (1<<4),
    /// Can be used as SSBO
    VK_BUFFER_USAGE_STORAGE_BUFFER_BIT = (1<<5),
    /// Can be used as source of fixed-function index fetch (index buffer)
    VK_BUFFER_USAGE_INDEX_BUFFER_BIT = (1<<6),
    /// Can be used as source of fixed-function vertex fetch (VBO)
    VK_BUFFER_USAGE_VERTEX_BUFFER_BIT = (1<<7),
    /// Can be the source of indirect parameters (e.g. indirect buffer, parameter buffer)
    VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT = (1<<8),
  }
);

flag_bits!(
  /// [VkBufferCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCreateFlagBits.html)
  VkBufferCreateFlagBits = VkBufferCreateFlags {
    /// Buffer should support sparse backing
    VK_BUFFER_CREATE_SPARSE_BINDING_BIT = (1<<0),
    /// Buffer should support sparse backing with partial residency
    VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT = (1<<1),
    /// Buffer should support constant data access to physical memory ranges mapped into multiple locations of sparse buffers
    VK_BUFFER_CREATE_SPARSE_ALIASED_BIT = (1<<2),
  }
);

flag_bits!(
  /// [VkShaderStageFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderStageFlagBits.html)
  VkShaderStageFlagBits = VkShaderStageFlags {
    VK_SHADER_STAGE_VERTEX_BIT = (1<<0),
    VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT = (1<<1),
    VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT = (1<<2),
    VK_SHADER_STAGE_GEOMETRY_BIT = (1<<3),
    VK_SHADER_STAGE_FRAGMENT_BIT = (1<<4),
    VK_SHADER_STAGE_COMPUTE_BIT = (1<<5),
  }
);
pub const VK_SHADER_STAGE_ALL_GRAPHICS: VkShaderStageFlagBits = VkShaderStageFlagBits(0x0000001F);
pub const VK_SHADER_STAGE_ALL: VkShaderStageFlagBits = VkShaderStageFlagBits(0x7FFFFFFF);

flag_bits!(
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
  }
);

flag_bits!(
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
  }
);

flag_bits!(
  /// [VkImageViewCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewCreateFlagBits.html)
  VkImageViewCreateFlagBits = VkImageViewCreateFlags {}
);

flag_bits!(
  /// [VkSamplerCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerCreateFlagBits.html)
  VkSamplerCreateFlagBits = VkSamplerCreateFlags {}
);

flag_bits!(
  /// [VkPipelineCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCreateFlagBits.html)
  VkPipelineCreateFlagBits = VkPipelineCreateFlags {
    VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT = (1<<0),
    VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT = (1<<1),
    VK_PIPELINE_CREATE_DERIVATIVE_BIT = (1<<2),
  }
);

flag_bits!(
  /// [VkPipelineShaderStageCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineShaderStageCreateFlagBits.html)
  VkPipelineShaderStageCreateFlagBits = VkPipelineShaderStageCreateFlags {}
);

flag_bits!(
  /// [VkColorComponentFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkColorComponentFlagBits.html)
  VkColorComponentFlagBits = VkColorComponentFlags {
    VK_COLOR_COMPONENT_R_BIT = (1<<0),
    VK_COLOR_COMPONENT_G_BIT = (1<<1),
    VK_COLOR_COMPONENT_B_BIT = (1<<2),
    VK_COLOR_COMPONENT_A_BIT = (1<<3),
  }
);

flag_bits!(
  /// [VkFenceCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceCreateFlagBits.html)
  VkFenceCreateFlagBits = VkFenceCreateFlags {
    VK_FENCE_CREATE_SIGNALED_BIT = (1<<0),
  }
);

flag_bits!(
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
  }
);

flag_bits!(
  /// [VkQueryControlFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryControlFlagBits.html)
  VkQueryControlFlagBits = VkQueryControlFlags {
    /// Require precise results to be collected by the query
    VK_QUERY_CONTROL_PRECISE_BIT = (1<<0),
  }
);

flag_bits!(
  /// [VkQueryResultFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryResultFlagBits.html)
  VkQueryResultFlagBits = VkQueryResultFlags {
    /// Results of the queries are written to the destination buffer as 64-bit values
    VK_QUERY_RESULT_64_BIT = (1<<0),
    /// Results of the queries are waited on before proceeding with the result copy
    VK_QUERY_RESULT_WAIT_BIT = (1<<1),
    /// Besides the results of the query, the availability of the results is also written
    VK_QUERY_RESULT_WITH_AVAILABILITY_BIT = (1<<2),
    /// Copy the partial results of the query even if the final results are not available
    VK_QUERY_RESULT_PARTIAL_BIT = (1<<3),
  }
);

flag_bits!(
  /// [VkCommandBufferUsageFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferUsageFlagBits.html)
  VkCommandBufferUsageFlagBits = VkCommandBufferUsageFlags {
    VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT = (1<<0),
    VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT = (1<<1),
    /// Command buffer may be submitted/executed more than once simultaneously
    VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT = (1<<2),
  }
);

flag_bits!(
  /// [VkQueryPipelineStatisticFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPipelineStatisticFlagBits.html)
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlags {
    /// Optional
    VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT = (1<<0),
    /// Optional
    VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT = (1<<1),
    /// Optional
    VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT = (1<<2),
    /// Optional
    VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT = (1<<3),
    /// Optional
    VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT = (1<<4),
    /// Optional
    VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT = (1<<5),
    /// Optional
    VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT = (1<<6),
    /// Optional
    VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT = (1<<7),
    /// Optional
    VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT = (1<<8),
    /// Optional
    VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT = (1<<9),
    /// Optional
    VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT = (1<<10),
  }
);

flag_bits!(
  /// [VkImageAspectFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageAspectFlagBits.html)
  VkImageAspectFlagBits = VkImageAspectFlags {
    VK_IMAGE_ASPECT_COLOR_BIT = (1<<0),
    VK_IMAGE_ASPECT_DEPTH_BIT = (1<<1),
    VK_IMAGE_ASPECT_STENCIL_BIT = (1<<2),
    VK_IMAGE_ASPECT_METADATA_BIT = (1<<3),
  }
);

flag_bits!(
  /// [VkSparseImageFormatFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageFormatFlagBits.html)
  VkSparseImageFormatFlagBits = VkSparseImageFormatFlags {
    /// Image uses a single mip tail region for all array layers
    VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT = (1<<0),
    /// Image requires mip level dimensions to be an integer multiple of the sparse image block dimensions for non-tail mip levels.
    VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT = (1<<1),
    /// Image uses a non-standard sparse image block dimensions
    VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT = (1<<2),
  }
);

flag_bits!(
  /// [VkSparseMemoryBindFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseMemoryBindFlagBits.html)
  VkSparseMemoryBindFlagBits = VkSparseMemoryBindFlags {
    /// Operation binds resource metadata to memory
    VK_SPARSE_MEMORY_BIND_METADATA_BIT = (1<<0),
  }
);

flag_bits!(
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
  }
);

flag_bits!(
  /// [VkCommandPoolCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandPoolCreateFlagBits.html)
  VkCommandPoolCreateFlagBits = VkCommandPoolCreateFlags {
    /// Command buffers have a short lifetime
    VK_COMMAND_POOL_CREATE_TRANSIENT_BIT = (1<<0),
    /// Command buffers may release their memory individually
    VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT = (1<<1),
  }
);

flag_bits!(
  /// [VkCommandPoolResetFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandPoolResetFlagBits.html)
  VkCommandPoolResetFlagBits = VkCommandPoolResetFlags {
    /// Release resources owned by the pool
    VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT = (1<<0),
  }
);

flag_bits!(
  /// [VkCommandBufferResetFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferResetFlagBits.html)
  VkCommandBufferResetFlagBits = VkCommandBufferResetFlags {
    /// Release resources owned by the buffer
    VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT = (1<<0),
  }
);

flag_bits!(
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
);

flag_bits!(
  /// [VkAttachmentDescriptionFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentDescriptionFlagBits.html)
  VkAttachmentDescriptionFlagBits = VkAttachmentDescriptionFlags {
    /// The attachment may alias physical memory of another attachment in the same render pass
    VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT = (1<<0),
  }
);

flag_bits!(
  /// [VkStencilFaceFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStencilFaceFlagBits.html)
  VkStencilFaceFlagBits = VkStencilFaceFlags {
    /// Front face
    VK_STENCIL_FACE_FRONT_BIT = (1<<0),
    /// Back face
    VK_STENCIL_FACE_BACK_BIT = (1<<1),
  }
);
/// Front and back faces
pub const VK_STENCIL_FACE_FRONT_AND_BACK: VkStencilFaceFlagBits = VkStencilFaceFlagBits(0x00000003);
/// Alias for backwards compatibility
pub const VK_STENCIL_FRONT_AND_BACK: VkStencilFaceFlagBits = VK_STENCIL_FACE_FRONT_AND_BACK;

flag_bits!(
  /// [VkDescriptorPoolCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorPoolCreateFlagBits.html)
  VkDescriptorPoolCreateFlagBits = VkDescriptorPoolCreateFlags {
    /// Descriptor sets may be freed individually
    VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT = (1<<0),
  }
);

flag_bits!(
  /// [VkDependencyFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDependencyFlagBits.html)
  VkDependencyFlagBits = VkDependencyFlags {
    /// Dependency is per pixel region
    VK_DEPENDENCY_BY_REGION_BIT = (1<<0),
  }
);

flag_bits!(
  /// [VkSemaphoreWaitFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreWaitFlagBits.html)
  VkSemaphoreWaitFlagBits = VkSemaphoreWaitFlags {
    VK_SEMAPHORE_WAIT_ANY_BIT = (1<<0),
  }
);

flag_bits!(
  /// [VkDisplayPlaneAlphaFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlaneAlphaFlagBitsKHR.html)
  VkDisplayPlaneAlphaFlagBitsKHR {
    VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR = (1<<0),
    VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR = (1<<1),
    VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR = (1<<2),
    VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR = (1<<3),
  }
);

flag_bits!(
  /// [VkCompositeAlphaFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCompositeAlphaFlagBitsKHR.html)
  VkCompositeAlphaFlagBitsKHR {
    VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR = (1<<0),
    VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR = (1<<1),
    VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR = (1<<2),
    VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR = (1<<3),
  }
);

flag_bits!(
  /// [VkSurfaceTransformFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceTransformFlagBitsKHR.html)
  VkSurfaceTransformFlagBitsKHR {
    VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR = (1<<0),
    VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR = (1<<1),
    VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR = (1<<2),
    VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR = (1<<3),
    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR = (1<<4),
    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR = (1<<5),
    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR = (1<<6),
    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR = (1<<7),
    VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR = (1<<8),
  }
);

flag_bits!(
  /// [VkSwapchainImageUsageFlagBitsANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainImageUsageFlagBitsANDROID.html)
  VkSwapchainImageUsageFlagBitsANDROID {
    VK_SWAPCHAIN_IMAGE_USAGE_SHARED_BIT_ANDROID = (1<<0),
  }
);

flag_bits!(
  /// [VkDebugReportFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugReportFlagBitsEXT.html)
  VkDebugReportFlagBitsEXT = VkDebugReportFlagsEXT {
    VK_DEBUG_REPORT_INFORMATION_BIT_EXT = (1<<0),
    VK_DEBUG_REPORT_WARNING_BIT_EXT = (1<<1),
    VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT = (1<<2),
    VK_DEBUG_REPORT_ERROR_BIT_EXT = (1<<3),
    VK_DEBUG_REPORT_DEBUG_BIT_EXT = (1<<4),
  }
);

flag_bits!(
  /// [VkExternalMemoryHandleTypeFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryHandleTypeFlagBitsNV.html)
  VkExternalMemoryHandleTypeFlagBitsNV {
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV = (1<<0),
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV = (1<<1),
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV = (1<<2),
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV = (1<<3),
  }
);
flag_bits!(
  /// [VkExternalMemoryFeatureFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryFeatureFlagBitsNV.html)
  VkExternalMemoryFeatureFlagBitsNV {
    VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV = (1<<0),
    VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV = (1<<1),
    VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV = (1<<2),
  }
);

flag_bits!(
  /// [VkSubgroupFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubgroupFeatureFlagBits.html)
  VkSubgroupFeatureFlagBits = VkSubgroupFeatureFlags {
    /// Basic subgroup operations
    VK_SUBGROUP_FEATURE_BASIC_BIT = (1<<0),
    /// Vote subgroup operations
    VK_SUBGROUP_FEATURE_VOTE_BIT = (1<<1),
    /// Arithmetic subgroup operations
    VK_SUBGROUP_FEATURE_ARITHMETIC_BIT = (1<<2),
    /// Ballot subgroup operations
    VK_SUBGROUP_FEATURE_BALLOT_BIT = (1<<3),
    /// Shuffle subgroup operations
    VK_SUBGROUP_FEATURE_SHUFFLE_BIT = (1<<4),
    /// Shuffle relative subgroup operations
    VK_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT = (1<<5),
    /// Clustered subgroup operations
    VK_SUBGROUP_FEATURE_CLUSTERED_BIT = (1<<6),
    /// Quad subgroup operations
    VK_SUBGROUP_FEATURE_QUAD_BIT = (1<<7),
  }
);

flag_bits!(
  /// [VkIndirectCommandsLayoutUsageFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectCommandsLayoutUsageFlagBitsNV.html)
  VkIndirectCommandsLayoutUsageFlagBitsNV = VkIndirectCommandsLayoutUsageFlagsNV {
    VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV = (1<<0),
    VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV = (1<<1),
    VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV = (1<<2),
  }
);

flag_bits!(
  /// [VkIndirectStateFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectStateFlagBitsNV.html)
  VkIndirectStateFlagBitsNV = VkIndirectStateFlagsNV {
    VK_INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV = (1<<0),
  }
);

flag_bits!(
  /// [VkPrivateDataSlotCreateFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPrivateDataSlotCreateFlagBitsEXT.html)
  VkPrivateDataSlotCreateFlagBitsEXT = VkPrivateDataSlotCreateFlagsEXT {}
);

flag_bits!(
  /// [VkDescriptorSetLayoutCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutCreateFlagBits.html)
  VkDescriptorSetLayoutCreateFlagBits = VkDescriptorSetLayoutCreateFlags {}
);

flag_bits!(
  /// [VkExternalMemoryHandleTypeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryHandleTypeFlagBits.html)
  VkExternalMemoryHandleTypeFlagBits {
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT = (1<<0),
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT = (1<<1),
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT = (1<<2),
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT = (1<<3),
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT = (1<<4),
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT = (1<<5),
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT = (1<<6),
  }
);

flag_bits!(
  /// [VkExternalMemoryFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryFeatureFlagBits.html)
  VkExternalMemoryFeatureFlagBits {
    VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT = (1<<0),
    VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT = (1<<1),
    VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT = (1<<2),
  }
);

flag_bits!(
  /// [VkExternalSemaphoreHandleTypeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreHandleTypeFlagBits.html)
  VkExternalSemaphoreHandleTypeFlagBits {
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT = (1<<0),
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT = (1<<1),
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT = (1<<2),
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT = (1<<3),
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT = (1<<4),
  }
);
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT: VkExternalSemaphoreHandleTypeFlagBits = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT;

flag_bits!(
  /// [VkExternalSemaphoreFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreFeatureFlagBits.html)
  VkExternalSemaphoreFeatureFlagBits {
      VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT = (1<<0),
      VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT = (1<<1),
  }
);

flag_bits!(
  /// [VkSemaphoreImportFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreImportFlagBits.html)
  VkSemaphoreImportFlagBits {
      VK_SEMAPHORE_IMPORT_TEMPORARY_BIT = (1<<0),
  }
);

flag_bits!(
  /// [VkExternalFenceHandleTypeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceHandleTypeFlagBits.html)
  VkExternalFenceHandleTypeFlagBits {
      VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT = (1<<0),
      VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT = (1<<1),
      VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT = (1<<2),
      VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT = (1<<3),
  }
);

flag_bits!(
  /// [VkExternalFenceFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceFeatureFlagBits.html)
  VkExternalFenceFeatureFlagBits {
      VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT = (1<<0),
      VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT = (1<<1),
  }
);

flag_bits!(
  /// [VkFenceImportFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceImportFlagBits.html)
  VkFenceImportFlagBits {
      VK_FENCE_IMPORT_TEMPORARY_BIT = (1<<0),
  }
);

flag_bits!(
  /// [VkSurfaceCounterFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCounterFlagBitsEXT.html)
  VkSurfaceCounterFlagBitsEXT {
    VK_SURFACE_COUNTER_VBLANK_BIT_EXT = (1<<0),
  }
);
/// Backwards-compatible alias containing a typo
pub const VK_SURFACE_COUNTER_VBLANK_EXT: VkSurfaceCounterFlagBitsEXT = VK_SURFACE_COUNTER_VBLANK_BIT_EXT;

flag_bits!(
  /// [VkPeerMemoryFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPeerMemoryFeatureFlagBits.html)
  VkPeerMemoryFeatureFlagBits {
    /// Can read with vkCmdCopy commands
    VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT = (1<<0),
    /// Can write with vkCmdCopy commands
    VK_PEER_MEMORY_FEATURE_COPY_DST_BIT = (1<<1),
    /// Can read with any access type/command
    VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT = (1<<2),
    /// Can write with and access type/command
    VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT = (1<<3),
  }
);

flag_bits!(
  /// [VkMemoryAllocateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryAllocateFlagBits.html)
  VkMemoryAllocateFlagBits {
    /// Force allocation on specific devices
    VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT = (1<<0),
  }
);

flag_bits!(
  /// [VkDeviceGroupPresentModeFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupPresentModeFlagBitsKHR.html)
  VkDeviceGroupPresentModeFlagBitsKHR {
    /// Present from local memory
    VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR = (1<<0),
    /// Present from remote memory
    VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR = (1<<1),
    /// Present sum of local and/or remote memory
    VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR = (1<<2),
    /// Each physical device presents from local memory
    VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR = (1<<3),
  }
);

flag_bits!(
  /// [VkSwapchainCreateFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainCreateFlagBitsKHR.html)
  VkSwapchainCreateFlagBitsKHR {}
);

flag_bits!(
  /// [VkSubpassDescriptionFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDescriptionFlagBits.html)
  VkSubpassDescriptionFlagBits = VkSubpassDescriptionFlags {}
);

flag_bits!(
  /// [VkDebugUtilsMessageSeverityFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessageSeverityFlagBitsEXT.html)
  VkDebugUtilsMessageSeverityFlagBitsEXT {
    VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT = (1<<0),
    VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT = (1<<4),
    VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT = (1<<8),
    VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT = (1<<12),
  }
);

flag_bits!(
  /// [VkDebugUtilsMessageTypeFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessageTypeFlagBitsEXT.html)
  VkDebugUtilsMessageTypeFlagBitsEXT = VkDebugUtilsMessageTypeFlagsEXT{
    VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT = (1<<0),
    VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT = (1<<1),
    VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT = (1<<2),
  }
);

flag_bits!(
  /// [VkDescriptorBindingFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorBindingFlagBits.html)
  VkDescriptorBindingFlagBits {
    VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT = (1<<0),
    VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT = (1<<1),
    VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT = (1<<2),
    VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT = (1<<3),
  }
);

flag_bits!(
  /// [VkConditionalRenderingFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkConditionalRenderingFlagBitsEXT.html)
  VkConditionalRenderingFlagBitsEXT {
    VK_CONDITIONAL_RENDERING_INVERTED_BIT_EXT = (1<<0),
  }
);

flag_bits!(
  /// [VkResolveModeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkResolveModeFlagBits.html)
  VkResolveModeFlagBits {
    VK_RESOLVE_MODE_NONE = (1<<0),
    VK_RESOLVE_MODE_SAMPLE_ZERO_BIT = (1<<0),
    VK_RESOLVE_MODE_AVERAGE_BIT = (1<<1),
    VK_RESOLVE_MODE_MIN_BIT = (1<<2),
    VK_RESOLVE_MODE_MAX_BIT = (1<<3),
  }
);

flag_bits!(
  /// [VkGeometryInstanceFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryInstanceFlagBitsKHR.html)
  VkGeometryInstanceFlagBitsKHR = VkGeometryInstanceFlagsKHR {
    VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR = (1<<0),
    VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR = (1<<1),
    VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR = (1<<2),
    VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR = (1<<3),
  }
);

flag_bits!(
  /// [VkGeometryFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryFlagBitsKHR.html)
  VkGeometryFlagBitsKHR = VkGeometryFlagsKHR {
    VK_GEOMETRY_OPAQUE_BIT_KHR = (1<<0),
    VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR = (1<<1),
  }
);

flag_bits!(
  /// [VkBuildAccelerationStructureFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBuildAccelerationStructureFlagBitsKHR.html)
  VkBuildAccelerationStructureFlagBitsKHR = VkBuildAccelerationStructureFlagsKHR {
    VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR = (1<<0),
    VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR = (1<<1),
    VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR = (1<<2),
    VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR = (1<<3),
    VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR = (1<<4),
  }
);

flag_bits!(
  /// [VkAccelerationStructureCreateFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureCreateFlagBitsKHR.html)
  VkAccelerationStructureCreateFlagBitsKHR = VkAccelerationStructureCreateFlagsKHR {
    VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR = (1<<0),
  }
);

flag_bits!(
  /// [VkFramebufferCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebufferCreateFlagBits.html)
  VkFramebufferCreateFlagBits = VkFramebufferCreateFlags {}
);

flag_bits!(
  /// [VkDeviceDiagnosticsConfigFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceDiagnosticsConfigFlagBitsNV.html)
  VkDeviceDiagnosticsConfigFlagBitsNV = VkDeviceDiagnosticsConfigFlagsNV {
    VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_DEBUG_INFO_BIT_NV = (1<<0),
    VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_RESOURCE_TRACKING_BIT_NV = (1<<1),
    VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_AUTOMATIC_CHECKPOINTS_BIT_NV = (1<<2),
  }
);

flag_bits!(
  /// [VkPipelineCreationFeedbackFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCreationFeedbackFlagBitsEXT.html)
  VkPipelineCreationFeedbackFlagBitsEXT = VkPipelineCreationFeedbackFlagsEXT {
    VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT_EXT = (1<<0),
    VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT_EXT = (1<<1),
    VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT_EXT = (1<<2),
  }
);

flag_bits!(
  /// [VkPerformanceCounterDescriptionFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterDescriptionFlagBitsKHR.html)
  VkPerformanceCounterDescriptionFlagBitsKHR = VkPerformanceCounterDescriptionFlagsKHR {
    VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR = (1<<0),
    VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR = (1<<1),
  }
);
/// Backwards-compatible alias containing a typo
pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_KHR: VkPerformanceCounterDescriptionFlagBitsKHR = VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR;
/// Backwards-compatible alias containing a typo
pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_KHR: VkPerformanceCounterDescriptionFlagBitsKHR = VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR;

flag_bits!(
  /// [VkAcquireProfilingLockFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAcquireProfilingLockFlagBitsKHR.html)
  VkAcquireProfilingLockFlagBitsKHR = VkAcquireProfilingLockFlagsKHR {}
);

flag_bits!(
  /// [VkShaderCorePropertiesFlagBitsAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderCorePropertiesFlagBitsAMD.html)
  VkShaderCorePropertiesFlagBitsAMD = VkShaderCorePropertiesFlagsAMD {}
);

flag_bits!(
  /// [VkShaderModuleCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderModuleCreateFlagBits.html)
  VkShaderModuleCreateFlagBits = VkShaderModuleCreateFlags {}
);

flag_bits!(
  /// [VkPipelineCompilerControlFlagBitsAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCompilerControlFlagBitsAMD.html)
  VkPipelineCompilerControlFlagBitsAMD = VkPipelineCompilerControlFlagsAMD {}
);

flag_bits!(
  /// [VkToolPurposeFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkToolPurposeFlagBitsEXT.html)
  VkToolPurposeFlagBitsEXT {
    VK_TOOL_PURPOSE_VALIDATION_BIT_EXT = (1<<0),
    VK_TOOL_PURPOSE_PROFILING_BIT_EXT = (1<<1),
    VK_TOOL_PURPOSE_TRACING_BIT_EXT = (1<<2),
    VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT_EXT = (1<<3),
    VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT_EXT = (1<<4),
  }
);

flag_bits!(
  /// [VkPipelineCacheCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCacheCreateFlagBits.html)
  VkPipelineCacheCreateFlagBits = VkPipelineCacheCreateFlags {
    /// Provided by VK_EXT_pipeline_creation_cache_control
    VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT_EXT = (1<<0),
  }
);

pub type VkSemaphoreWaitFlagBitsKHR = VkSemaphoreWaitFlagBits;
pub type VkExternalMemoryHandleTypeFlagBitsKHR = VkExternalMemoryHandleTypeFlagBits;
pub type VkExternalMemoryFeatureFlagBitsKHR = VkExternalMemoryFeatureFlagBits;
pub type VkExternalSemaphoreHandleTypeFlagBitsKHR = VkExternalSemaphoreHandleTypeFlagBits;
pub type VkExternalSemaphoreFeatureFlagBitsKHR = VkExternalSemaphoreFeatureFlagBits;
pub type VkPeerMemoryFeatureFlagBitsKHR = VkPeerMemoryFeatureFlagBits;
pub type VkMemoryAllocateFlagBitsKHR = VkMemoryAllocateFlagBits;
pub type VkDescriptorBindingFlagBitsEXT = VkDescriptorBindingFlagBits;
pub type VkResolveModeFlagBitsKHR = VkResolveModeFlagBits;
pub type VkGeometryInstanceFlagBitsNV = VkGeometryInstanceFlagBitsKHR;
pub type VkGeometryFlagBitsNV = VkGeometryFlagBitsKHR;
pub type VkBuildAccelerationStructureFlagBitsNV = VkBuildAccelerationStructureFlagBitsKHR;
pub type VkBuildAccelerationStructureFlagsNV = VkBuildAccelerationStructureFlagsKHR;
pub type VkExternalFenceFeatureFlagBitsKHR = VkExternalFenceFeatureFlagBits;
pub type VkExternalFenceHandleTypeFlagBitsKHR = VkExternalFenceHandleTypeFlagBits;
pub type VkFenceImportFlagBitsKHR = VkFenceImportFlagBits;
pub type VkGeometryFlagsNV = VkGeometryFlagsKHR;
pub type VkGeometryInstanceFlagsNV = VkGeometryInstanceFlagsKHR;
pub type VkSemaphoreImportFlagBitsKHR = VkSemaphoreImportFlagBits;
pub type VkSemaphoreWaitFlagsKHR = VkSemaphoreWaitFlags;

/// currently reserved for future use.
pub type VkDescriptorUpdateTemplateCreateFlags = u32;

/// currently reserved for future use.
pub type VkDebugUtilsMessengerCallbackDataFlagsEXT = u32;

/// currently reserved for future use.
pub type VkDeviceMemoryReportFlagsEXT = u32;
