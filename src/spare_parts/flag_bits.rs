use super::*;

flag_bits! {
  /// [VkCullModeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCullModeFlagBits.html)
  VkCullModeFlagBits = VkCullModeFlags {
    VK_CULL_MODE_NONE = (1<<0),
    VK_CULL_MODE_FRONT_BIT = (1<<0),
    VK_CULL_MODE_BACK_BIT = (1<<1),
  }
}
pub const VK_CULL_MODE_FRONT_AND_BACK: VkCullModeFlagBits = VkCullModeFlagBits(0x00000003);

flag_bits! {
  /// [VkRenderPassCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassCreateFlagBits.html)
  ///
  /// currently reserved for future use.
  VkRenderPassCreateFlagBits = VkRenderPassCreateFlags {}
}

flag_bits! {
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
}

flag_bits! {
  /// [VkBufferCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCreateFlagBits.html)
  VkBufferCreateFlagBits = VkBufferCreateFlags {
    /// Buffer should support sparse backing
    VK_BUFFER_CREATE_SPARSE_BINDING_BIT = (1<<0),
    /// Buffer should support sparse backing with partial residency
    VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT = (1<<1),
    /// Buffer should support constant data access to physical memory ranges mapped into multiple locations of sparse buffers
    VK_BUFFER_CREATE_SPARSE_ALIASED_BIT = (1<<2),
  }
}

flag_bits! {
  /// [VkShaderStageFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderStageFlagBits.html)
  VkShaderStageFlagBits = VkShaderStageFlags {
    VK_SHADER_STAGE_VERTEX_BIT = (1<<0),
    VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT = (1<<1),
    VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT = (1<<2),
    VK_SHADER_STAGE_GEOMETRY_BIT = (1<<3),
    VK_SHADER_STAGE_FRAGMENT_BIT = (1<<4),
    VK_SHADER_STAGE_COMPUTE_BIT = (1<<5),
  }
}
pub const VK_SHADER_STAGE_ALL_GRAPHICS: VkShaderStageFlagBits = VkShaderStageFlagBits(0x0000001F);
pub const VK_SHADER_STAGE_ALL: VkShaderStageFlagBits = VkShaderStageFlagBits(0x7FFFFFFF);

flag_bits! {
  /// [VkImageViewCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewCreateFlagBits.html)
  ///
  /// currently reserved for future use.
  VkImageViewCreateFlagBits = VkImageViewCreateFlags {}
}

flag_bits! {
  /// [VkSamplerCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerCreateFlagBits.html)
  ///
  /// currently reserved for future use.
  VkSamplerCreateFlagBits = VkSamplerCreateFlags {}
}

flag_bits! {
  /// [VkPipelineCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCreateFlagBits.html)
  VkPipelineCreateFlagBits = VkPipelineCreateFlags {
    VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT = (1<<0),
    VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT = (1<<1),
    VK_PIPELINE_CREATE_DERIVATIVE_BIT = (1<<2),
  }
}

flag_bits! {
  /// [VkPipelineShaderStageCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineShaderStageCreateFlagBits.html)
  VkPipelineShaderStageCreateFlagBits = VkPipelineShaderStageCreateFlags {}
}

flag_bits! {
  /// [VkColorComponentFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkColorComponentFlagBits.html)
  VkColorComponentFlagBits = VkColorComponentFlags {
    VK_COLOR_COMPONENT_R_BIT = (1<<0),
    VK_COLOR_COMPONENT_G_BIT = (1<<1),
    VK_COLOR_COMPONENT_B_BIT = (1<<2),
    VK_COLOR_COMPONENT_A_BIT = (1<<3),
  }
}

flag_bits! {
  /// [VkQueryControlFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryControlFlagBits.html)
  VkQueryControlFlagBits = VkQueryControlFlags {
    /// Require precise results to be collected by the query
    VK_QUERY_CONTROL_PRECISE_BIT = (1<<0),
  }
}

flag_bits! {
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
}

flag_bits! {
  /// [VkCommandBufferUsageFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferUsageFlagBits.html)
  VkCommandBufferUsageFlagBits = VkCommandBufferUsageFlags {
    VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT = (1<<0),
    VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT = (1<<1),
    /// Command buffer may be submitted/executed more than once simultaneously
    VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT = (1<<2),
  }
}

flag_bits! {
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
}

flag_bits! {
  /// [VkCommandPoolCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandPoolCreateFlagBits.html)
  VkCommandPoolCreateFlagBits = VkCommandPoolCreateFlags {
    /// Command buffers have a short lifetime
    VK_COMMAND_POOL_CREATE_TRANSIENT_BIT = (1<<0),
    /// Command buffers may release their memory individually
    VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT = (1<<1),
  }
}

flag_bits! {
  /// [VkCommandPoolResetFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandPoolResetFlagBits.html)
  VkCommandPoolResetFlagBits = VkCommandPoolResetFlags {
    /// Release resources owned by the pool
    VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT = (1<<0),
  }
}

flag_bits! {
  /// [VkCommandBufferResetFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferResetFlagBits.html)
  VkCommandBufferResetFlagBits = VkCommandBufferResetFlags {
    /// Release resources owned by the buffer
    VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT = (1<<0),
  }
}

flag_bits! {
  /// [VkAttachmentDescriptionFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentDescriptionFlagBits.html)
  VkAttachmentDescriptionFlagBits = VkAttachmentDescriptionFlags {
    /// The attachment may alias physical memory of another attachment in the same render pass
    VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT = (1<<0),
  }
}

flag_bits! {
  /// [VkStencilFaceFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStencilFaceFlagBits.html)
  VkStencilFaceFlagBits = VkStencilFaceFlags {
    /// Front face
    VK_STENCIL_FACE_FRONT_BIT = (1<<0),
    /// Back face
    VK_STENCIL_FACE_BACK_BIT = (1<<1),
  }
}
/// Front and back faces
pub const VK_STENCIL_FACE_FRONT_AND_BACK: VkStencilFaceFlagBits = VkStencilFaceFlagBits(0x00000003);
/// Alias for backwards compatibility
pub const VK_STENCIL_FRONT_AND_BACK: VkStencilFaceFlagBits = VK_STENCIL_FACE_FRONT_AND_BACK;

flag_bits! {
  /// [VkDescriptorPoolCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorPoolCreateFlagBits.html)
  VkDescriptorPoolCreateFlagBits = VkDescriptorPoolCreateFlags {
    /// Descriptor sets may be freed individually
    VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT = (1<<0),
  }
}

flag_bits! {
  /// [VkDependencyFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDependencyFlagBits.html)
  VkDependencyFlagBits = VkDependencyFlags {
    /// Dependency is per pixel region
    VK_DEPENDENCY_BY_REGION_BIT = (1<<0),
  }
}

flag_bits! {
  /// [VkSemaphoreWaitFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreWaitFlagBits.html)
  VkSemaphoreWaitFlagBits = VkSemaphoreWaitFlags {
    VK_SEMAPHORE_WAIT_ANY_BIT = (1<<0),
  }
}

flag_bits! {
  /// [VkDisplayPlaneAlphaFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlaneAlphaFlagBitsKHR.html)
  VkDisplayPlaneAlphaFlagBitsKHR {
    VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR = (1<<0),
    VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR = (1<<1),
    VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR = (1<<2),
    VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR = (1<<3),
  }
}

flag_bits! {
  /// [VkCompositeAlphaFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCompositeAlphaFlagBitsKHR.html)
  VkCompositeAlphaFlagBitsKHR {
    VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR = (1<<0),
    VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR = (1<<1),
    VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR = (1<<2),
    VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR = (1<<3),
  }
}

flag_bits! {
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
}

flag_bits! {
  /// [VkSwapchainImageUsageFlagBitsANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainImageUsageFlagBitsANDROID.html)
  VkSwapchainImageUsageFlagBitsANDROID {
    VK_SWAPCHAIN_IMAGE_USAGE_SHARED_BIT_ANDROID = (1<<0),
  }
}

flag_bits! {
  /// [VkDebugReportFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugReportFlagBitsEXT.html)
  VkDebugReportFlagBitsEXT = VkDebugReportFlagsEXT {
    VK_DEBUG_REPORT_INFORMATION_BIT_EXT = (1<<0),
    VK_DEBUG_REPORT_WARNING_BIT_EXT = (1<<1),
    VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT = (1<<2),
    VK_DEBUG_REPORT_ERROR_BIT_EXT = (1<<3),
    VK_DEBUG_REPORT_DEBUG_BIT_EXT = (1<<4),
  }
}

flag_bits! {
  /// [VkExternalMemoryHandleTypeFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryHandleTypeFlagBitsNV.html)
  VkExternalMemoryHandleTypeFlagBitsNV {
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV = (1<<0),
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV = (1<<1),
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV = (1<<2),
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV = (1<<3),
  }
}
flag_bits! {
  /// [VkExternalMemoryFeatureFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryFeatureFlagBitsNV.html)
  VkExternalMemoryFeatureFlagBitsNV {
    VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV = (1<<0),
    VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV = (1<<1),
    VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV = (1<<2),
  }
}

flag_bits! {
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
}

flag_bits! {
  /// [VkIndirectCommandsLayoutUsageFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectCommandsLayoutUsageFlagBitsNV.html)
  VkIndirectCommandsLayoutUsageFlagBitsNV = VkIndirectCommandsLayoutUsageFlagsNV {
    VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV = (1<<0),
    VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV = (1<<1),
    VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV = (1<<2),
  }
}

flag_bits! {
  /// [VkIndirectStateFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectStateFlagBitsNV.html)
  VkIndirectStateFlagBitsNV = VkIndirectStateFlagsNV {
    VK_INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV = (1<<0),
  }
}

flag_bits! {
  /// [VkPrivateDataSlotCreateFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPrivateDataSlotCreateFlagBitsEXT.html)
  ///
  /// currently reserved for future use.
  VkPrivateDataSlotCreateFlagBitsEXT = VkPrivateDataSlotCreateFlagsEXT {}
}

flag_bits! {
  /// [VkDescriptorSetLayoutCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutCreateFlagBits.html)
  ///
  /// currently reserved for future use.
  VkDescriptorSetLayoutCreateFlagBits = VkDescriptorSetLayoutCreateFlags {}
}

flag_bits! {
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
}

flag_bits! {
  /// [VkExternalMemoryFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryFeatureFlagBits.html)
  VkExternalMemoryFeatureFlagBits {
    VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT = (1<<0),
    VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT = (1<<1),
    VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT = (1<<2),
  }
}

flag_bits! {
  /// [VkExternalSemaphoreHandleTypeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreHandleTypeFlagBits.html)
  VkExternalSemaphoreHandleTypeFlagBits {
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT = (1<<0),
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT = (1<<1),
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT = (1<<2),
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT = (1<<3),
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT = (1<<4),
  }
}
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT: VkExternalSemaphoreHandleTypeFlagBits =
  VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT;

flag_bits! {
  /// [VkExternalSemaphoreFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreFeatureFlagBits.html)
  VkExternalSemaphoreFeatureFlagBits {
    VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT = (1<<0),
    VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT = (1<<1),
  }
}

flag_bits! {
  /// [VkSemaphoreImportFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreImportFlagBits.html)
  VkSemaphoreImportFlagBits {
    VK_SEMAPHORE_IMPORT_TEMPORARY_BIT = (1<<0),
  }
}

flag_bits! {
  /// [VkExternalFenceHandleTypeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceHandleTypeFlagBits.html)
  VkExternalFenceHandleTypeFlagBits {
    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT = (1<<0),
    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT = (1<<1),
    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT = (1<<2),
    VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT = (1<<3),
  }
}

flag_bits! {
  /// [VkExternalFenceFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceFeatureFlagBits.html)
  VkExternalFenceFeatureFlagBits {
    VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT = (1<<0),
    VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT = (1<<1),
  }
}

flag_bits! {
  /// [VkFenceImportFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceImportFlagBits.html)
  VkFenceImportFlagBits {
    VK_FENCE_IMPORT_TEMPORARY_BIT = (1<<0),
  }
}

flag_bits! {
  /// [VkSurfaceCounterFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCounterFlagBitsEXT.html)
  VkSurfaceCounterFlagBitsEXT {
    VK_SURFACE_COUNTER_VBLANK_BIT_EXT = (1<<0),
  }
}
/// Backwards-compatible alias containing a typo
pub const VK_SURFACE_COUNTER_VBLANK_EXT: VkSurfaceCounterFlagBitsEXT = VK_SURFACE_COUNTER_VBLANK_BIT_EXT;

flag_bits! {
  /// [VkPeerMemoryFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPeerMemoryFeatureFlagBits.html)
  VkPeerMemoryFeatureFlagBits = VkPeerMemoryFeatureFlags {
    /// Can read with vkCmdCopy commands
    VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT = (1<<0),
    /// Can write with vkCmdCopy commands
    VK_PEER_MEMORY_FEATURE_COPY_DST_BIT = (1<<1),
    /// Can read with any access type/command
    VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT = (1<<2),
    /// Can write with and access type/command
    VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT = (1<<3),
  }
}

flag_bits! {
  /// [VkMemoryAllocateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryAllocateFlagBits.html)
  VkMemoryAllocateFlagBits {
    /// Force allocation on specific devices
    VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT = (1<<0),
  }
}

flag_bits! {
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
}

flag_bits! {
  /// [VkSwapchainCreateFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainCreateFlagBitsKHR.html)
  ///
  /// currently reserved for future use.
  VkSwapchainCreateFlagBitsKHR {}
}

flag_bits! {
  /// [VkSubpassDescriptionFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDescriptionFlagBits.html)
  ///
  /// currently reserved for future use.
  VkSubpassDescriptionFlagBits = VkSubpassDescriptionFlags {}
}

flag_bits! {
  /// [VkDebugUtilsMessageSeverityFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessageSeverityFlagBitsEXT.html)
  VkDebugUtilsMessageSeverityFlagBitsEXT {
    VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT = (1<<0),
    VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT = (1<<4),
    VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT = (1<<8),
    VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT = (1<<12),
  }
}

flag_bits! {
  /// [VkDebugUtilsMessageTypeFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessageTypeFlagBitsEXT.html)
  VkDebugUtilsMessageTypeFlagBitsEXT = VkDebugUtilsMessageTypeFlagsEXT{
    VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT = (1<<0),
    VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT = (1<<1),
    VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT = (1<<2),
  }
}

flag_bits! {
  /// [VkDescriptorBindingFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorBindingFlagBits.html)
  VkDescriptorBindingFlagBits {
    VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT = (1<<0),
    VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT = (1<<1),
    VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT = (1<<2),
    VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT = (1<<3),
  }
}

flag_bits! {
  /// [VkConditionalRenderingFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkConditionalRenderingFlagBitsEXT.html)
  VkConditionalRenderingFlagBitsEXT {
    VK_CONDITIONAL_RENDERING_INVERTED_BIT_EXT = (1<<0),
  }
}

flag_bits! {
  /// [VkResolveModeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkResolveModeFlagBits.html)
  VkResolveModeFlagBits {
    VK_RESOLVE_MODE_NONE = (1<<0),
    VK_RESOLVE_MODE_SAMPLE_ZERO_BIT = (1<<0),
    VK_RESOLVE_MODE_AVERAGE_BIT = (1<<1),
    VK_RESOLVE_MODE_MIN_BIT = (1<<2),
    VK_RESOLVE_MODE_MAX_BIT = (1<<3),
  }
}

flag_bits! {
  /// [VkGeometryInstanceFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryInstanceFlagBitsKHR.html)
  VkGeometryInstanceFlagBitsKHR = VkGeometryInstanceFlagsKHR {
    VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR = (1<<0),
    VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR = (1<<1),
    VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR = (1<<2),
    VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR = (1<<3),
  }
}

flag_bits! {
  /// [VkGeometryFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryFlagBitsKHR.html)
  VkGeometryFlagBitsKHR = VkGeometryFlagsKHR {
    VK_GEOMETRY_OPAQUE_BIT_KHR = (1<<0),
    VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR = (1<<1),
  }
}

flag_bits! {
  /// [VkBuildAccelerationStructureFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBuildAccelerationStructureFlagBitsKHR.html)
  VkBuildAccelerationStructureFlagBitsKHR = VkBuildAccelerationStructureFlagsKHR {
    VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR = (1<<0),
    VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR = (1<<1),
    VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR = (1<<2),
    VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR = (1<<3),
    VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR = (1<<4),
  }
}

flag_bits! {
  /// [VkAccelerationStructureCreateFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureCreateFlagBitsKHR.html)
  VkAccelerationStructureCreateFlagBitsKHR = VkAccelerationStructureCreateFlagsKHR {
    VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR = (1<<0),
  }
}

flag_bits! {
  /// [VkFramebufferCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebufferCreateFlagBits.html)
  VkFramebufferCreateFlagBits = VkFramebufferCreateFlags {}
}

flag_bits! {
  /// [VkDeviceDiagnosticsConfigFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceDiagnosticsConfigFlagBitsNV.html)
  VkDeviceDiagnosticsConfigFlagBitsNV = VkDeviceDiagnosticsConfigFlagsNV {
    VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_DEBUG_INFO_BIT_NV = (1<<0),
    VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_RESOURCE_TRACKING_BIT_NV = (1<<1),
    VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_AUTOMATIC_CHECKPOINTS_BIT_NV = (1<<2),
  }
}

flag_bits! {
  /// [VkPipelineCreationFeedbackFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCreationFeedbackFlagBitsEXT.html)
  VkPipelineCreationFeedbackFlagBitsEXT = VkPipelineCreationFeedbackFlagsEXT {
    VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT_EXT = (1<<0),
    VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT_EXT = (1<<1),
    VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT_EXT = (1<<2),
  }
}

flag_bits! {
  /// [VkPerformanceCounterDescriptionFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterDescriptionFlagBitsKHR.html)
  VkPerformanceCounterDescriptionFlagBitsKHR = VkPerformanceCounterDescriptionFlagsKHR {
    VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR = (1<<0),
    VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR = (1<<1),
  }
}
/// Backwards-compatible alias containing a typo
pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_KHR: VkPerformanceCounterDescriptionFlagBitsKHR =
  VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR;
/// Backwards-compatible alias containing a typo
pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_KHR: VkPerformanceCounterDescriptionFlagBitsKHR =
  VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR;

flag_bits! {
  /// [VkAcquireProfilingLockFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAcquireProfilingLockFlagBitsKHR.html)
  ///
  /// currently reserved for future use.
  VkAcquireProfilingLockFlagBitsKHR = VkAcquireProfilingLockFlagsKHR {}
}

flag_bits! {
  /// [VkShaderCorePropertiesFlagBitsAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderCorePropertiesFlagBitsAMD.html)
  ///
  /// currently reserved for future use.
  VkShaderCorePropertiesFlagBitsAMD = VkShaderCorePropertiesFlagsAMD {}
}

flag_bits! {
  /// [VkShaderModuleCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderModuleCreateFlagBits.html)
  ///
  /// currently reserved for future use.
  VkShaderModuleCreateFlagBits = VkShaderModuleCreateFlags {}
}

flag_bits! {
  /// [VkPipelineCompilerControlFlagBitsAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCompilerControlFlagBitsAMD.html)
  VkPipelineCompilerControlFlagBitsAMD = VkPipelineCompilerControlFlagsAMD {}
}

flag_bits! {
  /// [VkToolPurposeFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkToolPurposeFlagBitsEXT.html)
  VkToolPurposeFlagBitsEXT {
    VK_TOOL_PURPOSE_VALIDATION_BIT_EXT = (1<<0),
    VK_TOOL_PURPOSE_PROFILING_BIT_EXT = (1<<1),
    VK_TOOL_PURPOSE_TRACING_BIT_EXT = (1<<2),
    VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT_EXT = (1<<3),
    VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT_EXT = (1<<4),
  }
}

flag_bits! {
  /// [VkPipelineCacheCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCacheCreateFlagBits.html)
  VkPipelineCacheCreateFlagBits = VkPipelineCacheCreateFlags {
    /// Provided by VK_EXT_pipeline_creation_cache_control
    VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT_EXT = (1<<0),
  }
}

flag_bits! {
  /// [VkDescriptorUpdateTemplateCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateCreateFlags.html)
  ///
  /// currently reserved for future use.
  VkDescriptorUpdateTemplateCreateFlags {}
}
flag_bits! {
  /// [VkDebugUtilsMessengerCallbackDataFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessengerCallbackDataFlagsEXT.html)
  ///
  /// currently reserved for future use.
  VkDebugUtilsMessengerCallbackDataFlagsEXT {}
}
flag_bits! {
  /// [VkAndroidSurfaceCreateFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAndroidSurfaceCreateFlagsKHR.html)
  ///
  /// currently reserved for future use.
  VkAndroidSurfaceCreateFlagsKHR {}
}
flag_bits! {
  /// [VkBufferViewCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferViewCreateFlags.html)
  ///
  /// currently reserved for future use.
  VkBufferViewCreateFlags {}
}
flag_bits! {
  /// [VkCompositeAlphaFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCompositeAlphaFlagsKHR.html)
  ///
  /// currently reserved for future use.
  VkCompositeAlphaFlagsKHR {}
}
flag_bits! {
  /// [VkConditionalRenderingFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkConditionalRenderingFlagsEXT.html)
  ///
  /// currently reserved for future use.
  VkConditionalRenderingFlagsEXT {}
}
flag_bits! {
  /// [VkDebugUtilsMessageSeverityFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessageSeverityFlagsEXT.html)
  ///
  /// currently reserved for future use.
  VkDebugUtilsMessageSeverityFlagsEXT {}
}
flag_bits! {
  /// [VkDebugUtilsMessengerCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessengerCreateFlagsEXT.html)
  ///
  /// currently reserved for future use.
  VkDebugUtilsMessengerCreateFlagsEXT {}
}
flag_bits! {
  /// [VkDescriptorBindingFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorBindingFlags.html)
  ///
  /// currently reserved for future use.
  VkDescriptorBindingFlags {}
}
flag_bits! {
  /// [VkDeviceGroupPresentModeFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupPresentModeFlagsKHR.html)
  ///
  /// currently reserved for future use.
  VkDeviceGroupPresentModeFlagsKHR {}
}
flag_bits! {
  /// [VkDeviceMemoryReportFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceMemoryReportFlagsEXT.html)
  ///
  /// currently reserved for future use.
  VkDeviceMemoryReportFlagsEXT {}
}
flag_bits! {
  /// [VkDirectFBSurfaceCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDirectFBSurfaceCreateFlagsEXT.html)
  ///
  /// currently reserved for future use.
  VkDirectFBSurfaceCreateFlagsEXT {}
}
flag_bits! {
  /// [VkDisplayKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayKHR.html)
  ///
  /// currently reserved for future use.
  VkDisplayKHR {}
}
flag_bits! {
  /// [VkDisplayModeCreateFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayModeCreateFlagsKHR.html)
  ///
  /// currently reserved for future use.
  VkDisplayModeCreateFlagsKHR {}
}
flag_bits! {
  /// [VkDisplayModeKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayModeKHR.html)
  ///
  /// currently reserved for future use.
  VkDisplayModeKHR {}
}
flag_bits! {
  /// [VkDisplayPlaneAlphaFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlaneAlphaFlagsKHR.html)
  ///
  /// currently reserved for future use.
  VkDisplayPlaneAlphaFlagsKHR {}
}
flag_bits! {
  /// [VkDisplaySurfaceCreateFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplaySurfaceCreateFlagsKHR.html)
  ///
  /// currently reserved for future use.
  VkDisplaySurfaceCreateFlagsKHR {}
}
flag_bits! {
  /// [VkEventCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkEventCreateFlags.html)
  ///
  /// currently reserved for future use.
  VkEventCreateFlags {}
}
flag_bits! {
  /// [VkExternalFenceFeatureFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceFeatureFlags.html)
  ///
  /// currently reserved for future use.
  VkExternalFenceFeatureFlags {}
}
flag_bits! {
  /// [VkExternalFenceHandleTypeFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceHandleTypeFlags.html)
  ///
  /// currently reserved for future use.
  VkExternalFenceHandleTypeFlags {}
}
flag_bits! {
  /// [VkExternalMemoryFeatureFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryFeatureFlags.html)
  ///
  /// currently reserved for future use.
  VkExternalMemoryFeatureFlags {}
}
flag_bits! {
  /// [VkExternalMemoryFeatureFlagsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryFeatureFlagsNV.html)
  ///
  /// currently reserved for future use.
  VkExternalMemoryFeatureFlagsNV {}
}
flag_bits! {
  /// [VkExternalMemoryHandleTypeFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryHandleTypeFlags.html)
  ///
  /// currently reserved for future use.
  VkExternalMemoryHandleTypeFlags {}
}
flag_bits! {
  /// [VkExternalMemoryHandleTypeFlagsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryHandleTypeFlagsNV.html)
  ///
  /// currently reserved for future use.
  VkExternalMemoryHandleTypeFlagsNV {}
}
flag_bits! {
  /// [VkExternalSemaphoreFeatureFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreFeatureFlags.html)
  ///
  /// currently reserved for future use.
  VkExternalSemaphoreFeatureFlags {}
}
flag_bits! {
  /// [VkExternalSemaphoreHandleTypeFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreHandleTypeFlags.html)
  ///
  /// currently reserved for future use.
  VkExternalSemaphoreHandleTypeFlags {}
}
flag_bits! {
  /// [VkFenceImportFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceImportFlags.html)
  ///
  /// currently reserved for future use.
  VkFenceImportFlags {}
}
flag_bits! {
  /// [VkHeadlessSurfaceCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkHeadlessSurfaceCreateFlagsEXT.html)
  ///
  /// currently reserved for future use.
  VkHeadlessSurfaceCreateFlagsEXT {}
}
flag_bits! {
  /// [VkImagePipeSurfaceCreateFlagsFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImagePipeSurfaceCreateFlagsFUCHSIA.html)
  ///
  /// currently reserved for future use.
  VkImagePipeSurfaceCreateFlagsFUCHSIA {}
}
flag_bits! {
  /// [VkIOSSurfaceCreateFlagsMVK](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIOSSurfaceCreateFlagsMVK.html)
  ///
  /// currently reserved for future use.
  VkIOSSurfaceCreateFlagsMVK {}
}
flag_bits! {
  /// [VkMacOSSurfaceCreateFlagsMVK](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMacOSSurfaceCreateFlagsMVK.html)
  ///
  /// currently reserved for future use.
  VkMacOSSurfaceCreateFlagsMVK {}
}
flag_bits! {
  /// [VkMemoryAllocateFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryAllocateFlags.html)
  ///
  /// currently reserved for future use.
  VkMemoryAllocateFlags {}
}
flag_bits! {
  /// [VkMetalSurfaceCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMetalSurfaceCreateFlagsEXT.html)
  ///
  /// currently reserved for future use.
  VkMetalSurfaceCreateFlagsEXT {}
}
flag_bits! {
  /// [VkPipelineColorBlendStateCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineColorBlendStateCreateFlags.html)
  ///
  /// currently reserved for future use.
  VkPipelineColorBlendStateCreateFlags {}
}
flag_bits! {
  /// [VkPipelineCoverageModulationStateCreateFlagsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCoverageModulationStateCreateFlagsNV.html)
  ///
  /// currently reserved for future use.
  VkPipelineCoverageModulationStateCreateFlagsNV {}
}
flag_bits! {
  /// [VkPipelineCoverageReductionStateCreateFlagsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCoverageReductionStateCreateFlagsNV.html)
  ///
  /// currently reserved for future use.
  VkPipelineCoverageReductionStateCreateFlagsNV {}
}
flag_bits! {
  /// [VkPipelineCoverageToColorStateCreateFlagsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCoverageToColorStateCreateFlagsNV.html)
  ///
  /// currently reserved for future use.
  VkPipelineCoverageToColorStateCreateFlagsNV {}
}
flag_bits! {
  /// [VkPipelineDepthStencilStateCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDepthStencilStateCreateFlags.html)
  ///
  /// currently reserved for future use.
  VkPipelineDepthStencilStateCreateFlags {}
}
flag_bits! {
  /// [VkPipelineDiscardRectangleStateCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDiscardRectangleStateCreateFlagsEXT.html)
  ///
  /// currently reserved for future use.
  VkPipelineDiscardRectangleStateCreateFlagsEXT {}
}
flag_bits! {
  /// [VkPipelineDynamicStateCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDynamicStateCreateFlags.html)
  ///
  /// currently reserved for future use.
  VkPipelineDynamicStateCreateFlags {}
}
flag_bits! {
  /// [VkPipelineInputAssemblyStateCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineInputAssemblyStateCreateFlags.html)
  ///
  /// currently reserved for future use.
  VkPipelineInputAssemblyStateCreateFlags {}
}
flag_bits! {
  /// [VkPipelineLayoutCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineLayoutCreateFlags.html)
  ///
  /// currently reserved for future use.
  VkPipelineLayoutCreateFlags {}
}
flag_bits! {
  /// [VkPipelineMultisampleStateCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineMultisampleStateCreateFlags.html)
  ///
  /// currently reserved for future use.
  VkPipelineMultisampleStateCreateFlags {}
}
flag_bits! {
  /// [VkPipelineRasterizationConservativeStateCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationConservativeStateCreateFlagsEXT.html)
  ///
  /// currently reserved for future use.
  VkPipelineRasterizationConservativeStateCreateFlagsEXT {}
}
flag_bits! {
  /// [VkPipelineRasterizationDepthClipStateCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationDepthClipStateCreateFlagsEXT.html)
  ///
  /// currently reserved for future use.
  VkPipelineRasterizationDepthClipStateCreateFlagsEXT {}
}
flag_bits! {
  /// [VkPipelineRasterizationStateCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationStateCreateFlags.html)
  ///
  /// currently reserved for future use.
  VkPipelineRasterizationStateCreateFlags {}
}
flag_bits! {
  /// [VkPipelineRasterizationStateStreamCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationStateStreamCreateFlagsEXT.html)
  ///
  /// currently reserved for future use.
  VkPipelineRasterizationStateStreamCreateFlagsEXT {}
}
flag_bits! {
  /// [VkPipelineTessellationStateCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineTessellationStateCreateFlags.html)
  ///
  /// currently reserved for future use.
  VkPipelineTessellationStateCreateFlags {}
}
flag_bits! {
  /// [VkPipelineVertexInputStateCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineVertexInputStateCreateFlags.html)
  ///
  /// currently reserved for future use.
  VkPipelineVertexInputStateCreateFlags {}
}
flag_bits! {
  /// [VkPipelineViewportStateCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportStateCreateFlags.html)
  ///
  /// currently reserved for future use.
  VkPipelineViewportStateCreateFlags {}
}
flag_bits! {
  /// [VkPipelineViewportSwizzleStateCreateFlagsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportSwizzleStateCreateFlagsNV.html)
  ///
  /// currently reserved for future use.
  VkPipelineViewportSwizzleStateCreateFlagsNV {}
}
flag_bits! {
  /// [VkQueryPoolCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolCreateFlags.html)
  ///
  /// currently reserved for future use.
  VkQueryPoolCreateFlags {}
}
flag_bits! {
  /// [VkResolveModeFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkResolveModeFlags.html)
  ///
  /// currently reserved for future use.
  VkResolveModeFlags {}
}
flag_bits! {
  /// [VkSemaphoreImportFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreImportFlags.html)
  ///
  /// currently reserved for future use.
  VkSemaphoreImportFlags {}
}
flag_bits! {
  /// [VkStreamDescriptorSurfaceCreateFlagsGGP](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStreamDescriptorSurfaceCreateFlagsGGP.html)
  ///
  /// currently reserved for future use.
  VkStreamDescriptorSurfaceCreateFlagsGGP {}
}
flag_bits! {
  /// [VkSurfaceCounterFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCounterFlagsEXT.html)
  ///
  /// currently reserved for future use.
  VkSurfaceCounterFlagsEXT {}
}
flag_bits! {
  /// [VkSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceKHR.html)
  ///
  /// currently reserved for future use.
  VkSurfaceKHR {}
}
flag_bits! {
  /// [VkSurfaceTransformFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceTransformFlagsKHR.html)
  ///
  /// currently reserved for future use.
  VkSurfaceTransformFlagsKHR {}
}
flag_bits! {
  /// [VkSwapchainCreateFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainCreateFlagsKHR.html)
  ///
  /// currently reserved for future use.
  VkSwapchainCreateFlagsKHR {}
}
flag_bits! {
  /// [VkSwapchainImageUsageFlagsANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainImageUsageFlagsANDROID.html)
  ///
  /// currently reserved for future use.
  VkSwapchainImageUsageFlagsANDROID {}
}
flag_bits! {
  /// [VkToolPurposeFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkToolPurposeFlagsEXT.html)
  ///
  /// currently reserved for future use.
  VkToolPurposeFlagsEXT {}
}
flag_bits! {
  /// [VkValidationCacheCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationCacheCreateFlagsEXT.html)
  ///
  /// currently reserved for future use.
  VkValidationCacheCreateFlagsEXT {}
}
flag_bits! {
  /// [VkViSurfaceCreateFlagsNN](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViSurfaceCreateFlagsNN.html)
  ///
  /// currently reserved for future use.
  VkViSurfaceCreateFlagsNN {}
}
flag_bits! {
  /// [VkWaylandSurfaceCreateFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWaylandSurfaceCreateFlagsKHR.html)
  ///
  /// currently reserved for future use.
  VkWaylandSurfaceCreateFlagsKHR {}
}
flag_bits! {
  /// [VkWin32SurfaceCreateFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWin32SurfaceCreateFlagsKHR.html)
  ///
  /// currently reserved for future use.
  VkWin32SurfaceCreateFlagsKHR {}
}
flag_bits! {
  /// [VkXcbSurfaceCreateFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXcbSurfaceCreateFlagsKHR.html)
  ///
  /// currently reserved for future use.
  VkXcbSurfaceCreateFlagsKHR {}
}
flag_bits! {
  /// [VkXlibSurfaceCreateFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXlibSurfaceCreateFlagsKHR.html)
  ///
  /// currently reserved for future use.
  VkXlibSurfaceCreateFlagsKHR {}
}
flag_bits! {
  /// [VkXlibSurfaceCreateFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXlibSurfaceCreateFlagsKHR.html)
  ///
  /// currently reserved for future use.
  VkCommandPoolTrimFlags {}
}
flag_bits! {
  /// [VkXlibSurfaceCreateFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXlibSurfaceCreateFlagsKHR.html)
  ///
  /// currently reserved for future use.
  VkDescriptorPoolResetFlags {}
}

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
