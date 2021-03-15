use super::*;

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
  /// [VkPipelineDiscardRectangleStateCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDiscardRectangleStateCreateFlagsEXT.html)
  ///
  /// currently reserved for future use.
  VkPipelineDiscardRectangleStateCreateFlagsEXT {}
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
  /// [VkPipelineRasterizationStateStreamCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationStateStreamCreateFlagsEXT.html)
  ///
  /// currently reserved for future use.
  VkPipelineRasterizationStateStreamCreateFlagsEXT {}
}
flag_bits! {
  /// [VkPipelineViewportSwizzleStateCreateFlagsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportSwizzleStateCreateFlagsNV.html)
  ///
  /// currently reserved for future use.
  VkPipelineViewportSwizzleStateCreateFlagsNV {}
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
