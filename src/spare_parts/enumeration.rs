use super::*;

enumeration! {
  /// [VkAttachmentLoadOp](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentLoadOp.html)
  VkAttachmentLoadOp {
    VK_ATTACHMENT_LOAD_OP_LOAD = 0,
    VK_ATTACHMENT_LOAD_OP_CLEAR = 1,
    VK_ATTACHMENT_LOAD_OP_DONT_CARE = 2,
  }
}

enumeration! {
  /// [VkAttachmentStoreOp](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentStoreOp.html)
  VkAttachmentStoreOp {
    VK_ATTACHMENT_STORE_OP_STORE = 0,
    VK_ATTACHMENT_STORE_OP_DONT_CARE = 1,
  }
}

enumeration! {
  /// [VkImageViewType](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewType.html)
  VkImageViewType {
    VK_IMAGE_VIEW_TYPE_1D = 0,
    VK_IMAGE_VIEW_TYPE_2D = 1,
    VK_IMAGE_VIEW_TYPE_3D = 2,
    VK_IMAGE_VIEW_TYPE_CUBE = 3,
    VK_IMAGE_VIEW_TYPE_1D_ARRAY = 4,
    VK_IMAGE_VIEW_TYPE_2D_ARRAY = 5,
    VK_IMAGE_VIEW_TYPE_CUBE_ARRAY = 6,
  }
}

enumeration! {
  /// [VkCommandBufferLevel](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferLevel.html)
  VkCommandBufferLevel {
    VK_COMMAND_BUFFER_LEVEL_PRIMARY = 0,
    VK_COMMAND_BUFFER_LEVEL_SECONDARY = 1,
  }
}

enumeration! {
  /// [VkComponentSwizzle](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkComponentSwizzle.html)
  VkComponentSwizzle {
    VK_COMPONENT_SWIZZLE_IDENTITY = 0,
    VK_COMPONENT_SWIZZLE_ZERO = 1,
    VK_COMPONENT_SWIZZLE_ONE = 2,
    VK_COMPONENT_SWIZZLE_R = 3,
    VK_COMPONENT_SWIZZLE_G = 4,
    VK_COMPONENT_SWIZZLE_B = 5,
    VK_COMPONENT_SWIZZLE_A = 6,
  }
}

enumeration! {
  /// [VkDescriptorType](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorType.html)
  VkDescriptorType {
    VK_DESCRIPTOR_TYPE_SAMPLER = 0,
    VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER = 1,
    VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE = 2,
    VK_DESCRIPTOR_TYPE_STORAGE_IMAGE = 3,
    VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER = 4,
    VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER = 5,
    VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER = 6,
    VK_DESCRIPTOR_TYPE_STORAGE_BUFFER = 7,
    VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC = 8,
    VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC = 9,
    VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT = 10,
  }
}

enumeration! {
  /// [VkQueryType](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryType.html)
  VkQueryType {
    VK_QUERY_TYPE_OCCLUSION = 0,
    /// Optional
    VK_QUERY_TYPE_PIPELINE_STATISTICS = 1,
    VK_QUERY_TYPE_TIMESTAMP = 2,
  }
}

enumeration! {
  /// [VkBorderColor](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBorderColor.html)
  VkBorderColor {
    VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK = 0,
    VK_BORDER_COLOR_INT_TRANSPARENT_BLACK = 1,
    VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK = 2,
    VK_BORDER_COLOR_INT_OPAQUE_BLACK = 3,
    VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE = 4,
    VK_BORDER_COLOR_INT_OPAQUE_WHITE = 5,
  }
}

enumeration! {
  /// [VkPipelineBindPoint](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineBindPoint.html)
  VkPipelineBindPoint {
    VK_PIPELINE_BIND_POINT_GRAPHICS = 0,
    VK_PIPELINE_BIND_POINT_COMPUTE = 1,
  }
}

enumeration! {
  /// [VkPrimitiveTopology](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPrimitiveTopology.html)
  VkPrimitiveTopology {
    VK_PRIMITIVE_TOPOLOGY_POINT_LIST = 0,
    VK_PRIMITIVE_TOPOLOGY_LINE_LIST = 1,
    VK_PRIMITIVE_TOPOLOGY_LINE_STRIP = 2,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST = 3,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP = 4,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN = 5,
    VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY = 6,
    VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY = 7,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY = 8,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY = 9,
    VK_PRIMITIVE_TOPOLOGY_PATCH_LIST = 10,
  }
}

enumeration! {
  /// [VkSharingMode](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSharingMode.html)
  VkSharingMode {
    VK_SHARING_MODE_EXCLUSIVE = 0,
    VK_SHARING_MODE_CONCURRENT = 1,
  }
}

enumeration! {
  /// [VkIndexType](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndexType.html)
  VkIndexType {
    VK_INDEX_TYPE_UINT16 = 0,
    VK_INDEX_TYPE_UINT32 = 1,
  }
}

enumeration! {
  /// [VkFilter](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFilter.html)
  VkFilter {
    VK_FILTER_NEAREST = 0,
    VK_FILTER_LINEAR = 1,
  }
}

enumeration! {
  /// [VkSamplerMipmapMode](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerMipmapMode.html)
  VkSamplerMipmapMode {
    /// Choose nearest mip level
    VK_SAMPLER_MIPMAP_MODE_NEAREST = 0,
    /// Linear filter between mip levels
    VK_SAMPLER_MIPMAP_MODE_LINEAR = 1,
  }
}

enumeration! {
  /// [VkSamplerAddressMode](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerAddressMode.html)
  VkSamplerAddressMode {
    VK_SAMPLER_ADDRESS_MODE_REPEAT = 0,
    VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT = 1,
    VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE = 2,
    VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER = 3,
  }
}

enumeration! {
  /// [VkCompareOp](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCompareOp.html)
  VkCompareOp {
    VK_COMPARE_OP_NEVER = 0,
    VK_COMPARE_OP_LESS = 1,
    VK_COMPARE_OP_EQUAL = 2,
    VK_COMPARE_OP_LESS_OR_EQUAL = 3,
    VK_COMPARE_OP_GREATER = 4,
    VK_COMPARE_OP_NOT_EQUAL = 5,
    VK_COMPARE_OP_GREATER_OR_EQUAL = 6,
    VK_COMPARE_OP_ALWAYS = 7,
  }
}

enumeration! {
  /// [VkPolygonMode](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPolygonMode.html)
  VkPolygonMode {
    VK_POLYGON_MODE_FILL = 0,
    VK_POLYGON_MODE_LINE = 1,
    VK_POLYGON_MODE_POINT = 2,
  }
}

enumeration! {
  /// [VkFrontFace](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFrontFace.html)
  VkFrontFace {
    VK_FRONT_FACE_COUNTER_CLOCKWISE = 0,
    VK_FRONT_FACE_CLOCKWISE = 1,
  }
}

enumeration! {
  /// [VkBlendFactor](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBlendFactor.html)
  VkBlendFactor {
    VK_BLEND_FACTOR_ZERO = 0,
    VK_BLEND_FACTOR_ONE = 1,
    VK_BLEND_FACTOR_SRC_COLOR = 2,
    VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR = 3,
    VK_BLEND_FACTOR_DST_COLOR = 4,
    VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR = 5,
    VK_BLEND_FACTOR_SRC_ALPHA = 6,
    VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA = 7,
    VK_BLEND_FACTOR_DST_ALPHA = 8,
    VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA = 9,
    VK_BLEND_FACTOR_CONSTANT_COLOR = 10,
    VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR = 11,
    VK_BLEND_FACTOR_CONSTANT_ALPHA = 12,
    VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA = 13,
    VK_BLEND_FACTOR_SRC_ALPHA_SATURATE = 14,
    VK_BLEND_FACTOR_SRC1_COLOR = 15,
    VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR = 16,
    VK_BLEND_FACTOR_SRC1_ALPHA = 17,
    VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA = 18,
  }
}

enumeration! {
  /// [VkBlendOp](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBlendOp.html)
  VkBlendOp {
    VK_BLEND_OP_ADD = 0,
    VK_BLEND_OP_SUBTRACT = 1,
    VK_BLEND_OP_REVERSE_SUBTRACT = 2,
    VK_BLEND_OP_MIN = 3,
    VK_BLEND_OP_MAX = 4,
  }
}

enumeration! {
  /// [VkStencilOp](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStencilOp.html)
  VkStencilOp {
    VK_STENCIL_OP_KEEP = 0,
    VK_STENCIL_OP_ZERO = 1,
    VK_STENCIL_OP_REPLACE = 2,
    VK_STENCIL_OP_INCREMENT_AND_CLAMP = 3,
    VK_STENCIL_OP_DECREMENT_AND_CLAMP = 4,
    VK_STENCIL_OP_INVERT = 5,
    VK_STENCIL_OP_INCREMENT_AND_WRAP = 6,
    VK_STENCIL_OP_DECREMENT_AND_WRAP = 7,
  }
}

enumeration! {
  /// [VkLogicOp](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkLogicOp.html)
  VkLogicOp {
    VK_LOGIC_OP_CLEAR = 0,
    VK_LOGIC_OP_AND = 1,
    VK_LOGIC_OP_AND_REVERSE = 2,
    VK_LOGIC_OP_COPY = 3,
    VK_LOGIC_OP_AND_INVERTED = 4,
    VK_LOGIC_OP_NO_OP = 5,
    VK_LOGIC_OP_XOR = 6,
    VK_LOGIC_OP_OR = 7,
    VK_LOGIC_OP_NOR = 8,
    VK_LOGIC_OP_EQUIVALENT = 9,
    VK_LOGIC_OP_INVERT = 10,
    VK_LOGIC_OP_OR_REVERSE = 11,
    VK_LOGIC_OP_COPY_INVERTED = 12,
    VK_LOGIC_OP_OR_INVERTED = 13,
    VK_LOGIC_OP_NAND = 14,
    VK_LOGIC_OP_SET = 15,
  }
}

enumeration! {
  /// [VkVertexInputRate](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputRate.html)
  VkVertexInputRate {
    VK_VERTEX_INPUT_RATE_VERTEX = 0,
    VK_VERTEX_INPUT_RATE_INSTANCE = 1,
  }
}

enumeration! {
  /// [VkSubpassContents](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassContents.html)
  VkSubpassContents {
    VK_SUBPASS_CONTENTS_INLINE = 0,
    VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS = 1,
  }
}

enumeration! {
  /// [VkDynamicState](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDynamicState.html)
  VkDynamicState {
    VK_DYNAMIC_STATE_VIEWPORT = 0,
    VK_DYNAMIC_STATE_SCISSOR = 1,
    VK_DYNAMIC_STATE_LINE_WIDTH = 2,
    VK_DYNAMIC_STATE_DEPTH_BIAS = 3,
    VK_DYNAMIC_STATE_BLEND_CONSTANTS = 4,
    VK_DYNAMIC_STATE_DEPTH_BOUNDS = 5,
    VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK = 6,
    VK_DYNAMIC_STATE_STENCIL_WRITE_MASK = 7,
    VK_DYNAMIC_STATE_STENCIL_REFERENCE = 8,
  }
}

enumeration! {
  /// [VkDescriptorUpdateTemplateType](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateType.html)
  VkDescriptorUpdateTemplateType {
    /// Create descriptor update template for descriptor set updates
    VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET = 0,
  }
}

enumeration! {
  /// [VkSemaphoreType](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreType.html)
  VkSemaphoreType {
    VK_SEMAPHORE_TYPE_BINARY = 0,
    VK_SEMAPHORE_TYPE_TIMELINE = 1,
  }
}

enumeration! {
  /// [VkPresentModeKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentModeKHR.html)
  VkPresentModeKHR {
    VK_PRESENT_MODE_IMMEDIATE_KHR = 0,
    VK_PRESENT_MODE_MAILBOX_KHR = 1,
    VK_PRESENT_MODE_FIFO_KHR = 2,
    VK_PRESENT_MODE_FIFO_RELAXED_KHR = 3,
  }
}

enumeration! {
  /// [VkColorSpaceKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkColorSpaceKHR.html)
  VkColorSpaceKHR {
    VK_COLOR_SPACE_SRGB_NONLINEAR_KHR = 0,
  }
}
/// Backwards-compatible alias containing a typo
pub const VK_COLORSPACE_SRGB_NONLINEAR_KHR: VkColorSpaceKHR = VK_COLOR_SPACE_SRGB_NONLINEAR_KHR;

enumeration! {
  /// [VkTimeDomainEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTimeDomainEXT.html)
  VkTimeDomainEXT {
    VK_TIME_DOMAIN_DEVICE_EXT = 0,
    VK_TIME_DOMAIN_CLOCK_MONOTONIC_EXT = 1,
    VK_TIME_DOMAIN_CLOCK_MONOTONIC_RAW_EXT = 2,
    VK_TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER_EXT = 3,
  }
}

enumeration! {
  /// [VkDebugReportObjectTypeEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugReportObjectTypeEXT.html)
  VkDebugReportObjectTypeEXT {
    VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT = 0,
    VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT = 1,
    VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT = 2,
    VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT = 3,
    VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT = 4,
    VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT = 5,
    VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT = 6,
    VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT = 7,
    VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT = 8,
    VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT = 9,
    VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT = 10,
    VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT = 11,
    VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT = 12,
    VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT = 13,
    VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT = 14,
    VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT = 15,
    VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT = 16,
    VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT = 17,
    VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT = 18,
    VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT = 19,
    VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT = 20,
    VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT = 21,
    VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT = 22,
    VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT = 23,
    VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT = 24,
    VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT = 25,
    VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT = 26,
    VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT = 27,
    VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT = 28,
    VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT = 29,
    VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT = 30,
    VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT = 33,
  }
}
/// Backwards-compatible alias containing a typo
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT: VkDebugReportObjectTypeEXT = VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT;
/// Backwards-compatible alias containing a typo
pub const VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT: VkDebugReportObjectTypeEXT = VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT;

enumeration! {
  /// [VkDeviceMemoryReportEventTypeEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceMemoryReportEventTypeEXT.html)
  VkDeviceMemoryReportEventTypeEXT {
    /// specifies this event corresponds to the allocation of an internal device memory object or a VkDeviceMemory.
    VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT = 0,
    /// specifies this event corresponds to the deallocation of an internally-allocated device memory object or a VkDeviceMemory.
    VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT = 1,
    /// specifies this event corresponds to the import of an external memory object.
    VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT = 2,
    /// specifies this event is the release of an imported external memory object.
    VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT = 3,
    /// specifies this event is the release of an imported external memory object.
    VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT = 4,
  }
}

enumeration! {
  /// [VkRasterizationOrderAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRasterizationOrderAMD.html)
  VkRasterizationOrderAMD {
    VK_RASTERIZATION_ORDER_STRICT_AMD = 0,
    VK_RASTERIZATION_ORDER_RELAXED_AMD = 1,
  }
}

enumeration! {
  /// [VkValidationCheckEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationCheckEXT.html)
  VkValidationCheckEXT {
    VK_VALIDATION_CHECK_ALL_EXT = 0,
    VK_VALIDATION_CHECK_SHADERS_EXT = 1,
  }
}

enumeration! {
  /// [VkValidationFeatureEnableEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationFeatureEnableEXT.html)
  VkValidationFeatureEnableEXT {
    VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT = 0,
    VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT = 1,
    VK_VALIDATION_FEATURE_ENABLE_BEST_PRACTICES_EXT = 2,
    VK_VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF_EXT = 3,
    VK_VALIDATION_FEATURE_ENABLE_SYNCHRONIZATION_VALIDATION_EXT = 4,
  }
}

enumeration! {
  /// [VkValidationFeatureDisableEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationFeatureDisableEXT.html)
  VkValidationFeatureDisableEXT {
    VK_VALIDATION_FEATURE_DISABLE_ALL_EXT = 0,
    VK_VALIDATION_FEATURE_DISABLE_SHADERS_EXT = 1,
    VK_VALIDATION_FEATURE_DISABLE_THREAD_SAFETY_EXT = 2,
    VK_VALIDATION_FEATURE_DISABLE_API_PARAMETERS_EXT = 3,
    VK_VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES_EXT = 4,
    VK_VALIDATION_FEATURE_DISABLE_CORE_CHECKS_EXT = 5,
    VK_VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES_EXT = 6,
  }
}

enumeration! {
  /// [VkIndirectCommandsTokenTypeNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectCommandsTokenTypeNV.html)
  VkIndirectCommandsTokenTypeNV {
    VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV = 0,
    VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV = 1,
    VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NV = 2,
    VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV = 3,
    VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV = 4,
    VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NV = 5,
    VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NV = 6,
    VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS_NV = 7,
  }
}

enumeration! {
  /// [VkDisplayPowerStateEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPowerStateEXT.html)
  VkDisplayPowerStateEXT {
    VK_DISPLAY_POWER_STATE_OFF_EXT = 0,
    VK_DISPLAY_POWER_STATE_SUSPEND_EXT = 1,
    VK_DISPLAY_POWER_STATE_ON_EXT = 2,
  }
}

enumeration! {
  /// [VkDeviceEventTypeEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceEventTypeEXT.html)
  VkDeviceEventTypeEXT {
    VK_DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT = 0,
  }
}

enumeration! {
  /// [VkDisplayEventTypeEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayEventTypeEXT.html)
  VkDisplayEventTypeEXT {
    VK_DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT = 0,
  }
}

enumeration! {
  /// [VkViewportCoordinateSwizzleNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViewportCoordinateSwizzleNV.html)
  VkViewportCoordinateSwizzleNV {
    VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV = 0,
    VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV = 1,
    VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV = 2,
    VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV = 3,
    VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV = 4,
    VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV = 5,
    VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV = 6,
    VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV = 7,
  }
}

enumeration! {
  /// [VkDiscardRectangleModeEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDiscardRectangleModeEXT.html)
  VkDiscardRectangleModeEXT {
    VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT = 0,
    VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT = 1,
  }
}

enumeration! {
  /// [VkPointClippingBehavior](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPointClippingBehavior.html)
  VkPointClippingBehavior {
    VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES = 0,
    VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY = 1,
  }
}

enumeration! {
  /// [VkSamplerReductionMode](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerReductionMode.html)
  VkSamplerReductionMode {
    VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE = 0,
    VK_SAMPLER_REDUCTION_MODE_MIN = 1,
    VK_SAMPLER_REDUCTION_MODE_MAX = 2,
  }
}

enumeration! {
  /// [VkTessellationDomainOrigin](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTessellationDomainOrigin.html)
  VkTessellationDomainOrigin {
    VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT = 0,
    VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT = 1,
  }
}

enumeration! {
  /// [VkSamplerYcbcrModelConversion](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrModelConversion.html)
  VkSamplerYcbcrModelConversion {
    VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY = 0,
    /// just range expansion
    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY = 1,
    /// aka HD YUV
    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709 = 2,
    /// aka SD YUV
    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601 = 3,
    /// aka UHD YUV
    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020 = 4,
  }
}

enumeration! {
  /// [VkSamplerYcbcrRange](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrRange.html)
  VkSamplerYcbcrRange {
    /// Luma 0..1 maps to 0..255, chroma -0.5..0.5 to 1..255 (clamped)
    VK_SAMPLER_YCBCR_RANGE_ITU_FULL = 0,
    /// Luma 0..1 maps to 16..235, chroma -0.5..0.5 to 16..240
    VK_SAMPLER_YCBCR_RANGE_ITU_NARROW = 1,
  }
}

enumeration! {
  /// [VkChromaLocation](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkChromaLocation.html)
  VkChromaLocation {
    VK_CHROMA_LOCATION_COSITED_EVEN = 0,
    VK_CHROMA_LOCATION_MIDPOINT = 1,
  }
}

enumeration! {
  /// [VkBlendOverlapEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBlendOverlapEXT.html)
  VkBlendOverlapEXT {
    VK_BLEND_OVERLAP_UNCORRELATED_EXT = 0,
    VK_BLEND_OVERLAP_DISJOINT_EXT = 1,
    VK_BLEND_OVERLAP_CONJOINT_EXT = 2,
  }
}

enumeration! {
  /// [VkCoverageModulationModeNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCoverageModulationModeNV.html)
  VkCoverageModulationModeNV {
    VK_COVERAGE_MODULATION_MODE_NONE_NV = 0,
    VK_COVERAGE_MODULATION_MODE_RGB_NV = 1,
    VK_COVERAGE_MODULATION_MODE_ALPHA_NV = 2,
    VK_COVERAGE_MODULATION_MODE_RGBA_NV = 3,
  }
}

enumeration! {
  /// [VkCoverageReductionModeNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCoverageReductionModeNV.html)
  VkCoverageReductionModeNV {
    VK_COVERAGE_REDUCTION_MODE_MERGE_NV = 0,
    VK_COVERAGE_REDUCTION_MODE_TRUNCATE_NV = 1,
  }
}

enumeration! {
  /// [VkValidationCacheHeaderVersionEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationCacheHeaderVersionEXT.html)
  VkValidationCacheHeaderVersionEXT {
    VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT = 1,
  }
}

enumeration! {
  /// [VkShaderInfoTypeAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderInfoTypeAMD.html)
  VkShaderInfoTypeAMD {
    VK_SHADER_INFO_TYPE_STATISTICS_AMD = 0,
    VK_SHADER_INFO_TYPE_BINARY_AMD = 1,
    VK_SHADER_INFO_TYPE_DISASSEMBLY_AMD = 2,
  }
}

enumeration! {
  /// [VkQueueGlobalPriorityEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueGlobalPriorityEXT.html)
  VkQueueGlobalPriorityEXT {
    VK_QUEUE_GLOBAL_PRIORITY_LOW_EXT = 128,
    VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_EXT = 256,
    VK_QUEUE_GLOBAL_PRIORITY_HIGH_EXT = 512,
    VK_QUEUE_GLOBAL_PRIORITY_REALTIME_EXT = 1024,
  }
}

enumeration! {
  /// [VkConservativeRasterizationModeEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkConservativeRasterizationModeEXT.html)
  VkConservativeRasterizationModeEXT {
    VK_CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT = 0,
    VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT = 1,
    VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT = 2,
  }
}

enumeration! {
  /// [VkDriverId](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDriverId.html)
  VkDriverId {
    /// Advanced Micro Devices, Inc.
    VK_DRIVER_ID_AMD_PROPRIETARY = 1,
    /// Advanced Micro Devices, Inc.
    VK_DRIVER_ID_AMD_OPEN_SOURCE = 2,
    /// Mesa open source project
    VK_DRIVER_ID_MESA_RADV = 3,
    /// NVIDIA Corporation
    VK_DRIVER_ID_NVIDIA_PROPRIETARY = 4,
    /// Intel Corporation
    VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS = 5,
    /// Intel Corporation
    VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA = 6,
    /// Imagination Technologies
    VK_DRIVER_ID_IMAGINATION_PROPRIETARY = 7,
    /// Qualcomm Technologies, Inc.
    VK_DRIVER_ID_QUALCOMM_PROPRIETARY = 8,
    /// Arm Limited
    VK_DRIVER_ID_ARM_PROPRIETARY = 9,
    /// Google LLC
    VK_DRIVER_ID_GOOGLE_SWIFTSHADER = 10,
    /// Google LLC
    VK_DRIVER_ID_GGP_PROPRIETARY = 11,
    /// Broadcom Inc.
    VK_DRIVER_ID_BROADCOM_PROPRIETARY = 12,
    /// Mesa
    VK_DRIVER_ID_MESA_LLVMPIPE = 13,
    /// MoltenVK
    VK_DRIVER_ID_MOLTENVK = 14,
  }
}

enumeration! {
  /// [VkShadingRatePaletteEntryNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShadingRatePaletteEntryNV.html)
  VkShadingRatePaletteEntryNV {
    VK_SHADING_RATE_PALETTE_ENTRY_NO_INVOCATIONS_NV = 0,
    VK_SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL_NV = 1,
    VK_SHADING_RATE_PALETTE_ENTRY_8_INVOCATIONS_PER_PIXEL_NV = 2,
    VK_SHADING_RATE_PALETTE_ENTRY_4_INVOCATIONS_PER_PIXEL_NV = 3,
    VK_SHADING_RATE_PALETTE_ENTRY_2_INVOCATIONS_PER_PIXEL_NV = 4,
    VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_PIXEL_NV = 5,
    VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X1_PIXELS_NV = 6,
    VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_1X2_PIXELS_NV = 7,
    VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X2_PIXELS_NV = 8,
    VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X2_PIXELS_NV = 9,
    VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X4_PIXELS_NV = 10,
    VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X4_PIXELS_NV = 11,
  }
}

enumeration! {
  /// [VkCoarseSampleOrderTypeNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCoarseSampleOrderTypeNV.html)
  VkCoarseSampleOrderTypeNV {
    VK_COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV = 0,
    VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV = 1,
    VK_COARSE_SAMPLE_ORDER_TYPE_PIXEL_MAJOR_NV = 2,
    VK_COARSE_SAMPLE_ORDER_TYPE_SAMPLE_MAJOR_NV = 3,
  }
}

enumeration! {
  /// [VkCopyAccelerationStructureModeKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyAccelerationStructureModeKHR.html)
  VkCopyAccelerationStructureModeKHR {
    VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR = 0,
    VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR = 1,
    VK_COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE_KHR = 2,
    VK_COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE_KHR = 3,
  }
}

enumeration! {
  /// [VkBuildAccelerationStructureModeKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBuildAccelerationStructureModeKHR.html)
  VkBuildAccelerationStructureModeKHR {
    VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR = 0,
    VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR = 1,
  }
}

enumeration! {
  /// [VkAccelerationStructureTypeKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureTypeKHR.html)
  VkAccelerationStructureTypeKHR {
    VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR = 0,
    VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR = 1,
    VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR = 2,
  }
}

enumeration! {
  /// [VkGeometryTypeKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryTypeKHR.html)
  VkGeometryTypeKHR {
    VK_GEOMETRY_TYPE_TRIANGLES_KHR = 0,
    VK_GEOMETRY_TYPE_AABBS_KHR = 1,
    VK_GEOMETRY_TYPE_INSTANCES_KHR = 2,
  }
}

enumeration! {
  /// [VkAccelerationStructureMemoryRequirementsTypeNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMemoryRequirementsTypeNV.html)
  VkAccelerationStructureMemoryRequirementsTypeNV {
    VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV = 0,
    VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV = 1,
    VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV = 2,
  }
}

enumeration! {
  /// [VkAccelerationStructureBuildTypeKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureBuildTypeKHR.html)
  VkAccelerationStructureBuildTypeKHR {
    VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_KHR = 0,
    VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR = 1,
    VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE_KHR = 2,
  }
}

enumeration! {
  /// [VkRayTracingShaderGroupTypeKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingShaderGroupTypeKHR.html)
  VkRayTracingShaderGroupTypeKHR {
    VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR = 0,
    VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR = 1,
    VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR = 2,
  }
}

enumeration! {
  /// [VkAccelerationStructureCompatibilityKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureCompatibilityKHR.html)
  VkAccelerationStructureCompatibilityKHR {
    VK_ACCELERATION_STRUCTURE_COMPATIBILITY_COMPATIBLE_KHR = 0,
    VK_ACCELERATION_STRUCTURE_COMPATIBILITY_INCOMPATIBLE_KHR = 1,
  }
}

enumeration! {
  /// [VkShaderGroupShaderKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderGroupShaderKHR.html)
  VkShaderGroupShaderKHR {
    VK_SHADER_GROUP_SHADER_GENERAL_KHR = 0,
    VK_SHADER_GROUP_SHADER_CLOSEST_HIT_KHR = 1,
    VK_SHADER_GROUP_SHADER_ANY_HIT_KHR = 2,
    VK_SHADER_GROUP_SHADER_INTERSECTION_KHR = 3,
  }
}

enumeration! {
  /// [VkMemoryOverallocationBehaviorAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryOverallocationBehaviorAMD.html)
  VkMemoryOverallocationBehaviorAMD {
    VK_MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT_AMD = 0,
    VK_MEMORY_OVERALLOCATION_BEHAVIOR_ALLOWED_AMD = 1,
    VK_MEMORY_OVERALLOCATION_BEHAVIOR_DISALLOWED_AMD = 2,
  }
}

enumeration! {
  /// [VkScopeNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkScopeNV.html)
  VkScopeNV {
    VK_SCOPE_DEVICE_NV = 1,
    VK_SCOPE_WORKGROUP_NV = 2,
    VK_SCOPE_SUBGROUP_NV = 3,
    VK_SCOPE_QUEUE_FAMILY_NV = 5,
  }
}

enumeration! {
  /// [VkComponentTypeNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkComponentTypeNV.html)
  VkComponentTypeNV {
    VK_COMPONENT_TYPE_FLOAT16_NV = 0,
    VK_COMPONENT_TYPE_FLOAT32_NV = 1,
    VK_COMPONENT_TYPE_FLOAT64_NV = 2,
    VK_COMPONENT_TYPE_SINT8_NV = 3,
    VK_COMPONENT_TYPE_SINT16_NV = 4,
    VK_COMPONENT_TYPE_SINT32_NV = 5,
    VK_COMPONENT_TYPE_SINT64_NV = 6,
    VK_COMPONENT_TYPE_UINT8_NV = 7,
    VK_COMPONENT_TYPE_UINT16_NV = 8,
    VK_COMPONENT_TYPE_UINT32_NV = 9,
    VK_COMPONENT_TYPE_UINT64_NV = 10,
  }
}

enumeration! {
  /// [VkFullScreenExclusiveEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFullScreenExclusiveEXT.html)
  VkFullScreenExclusiveEXT {
    VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT = 0,
    VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT = 1,
    VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT = 2,
    VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT = 3,
  }
}

enumeration! {
  /// [VkPerformanceCounterScopeKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterScopeKHR.html)
  VkPerformanceCounterScopeKHR {
    VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR = 0,
    VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR = 1,
    VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR = 2,
  }
}
pub const VK_QUERY_SCOPE_COMMAND_BUFFER_KHR: VkPerformanceCounterScopeKHR = VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR;
pub const VK_QUERY_SCOPE_RENDER_PASS_KHR: VkPerformanceCounterScopeKHR = VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR;
pub const VK_QUERY_SCOPE_COMMAND_KHR: VkPerformanceCounterScopeKHR = VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR;

enumeration! {
  /// [VkPerformanceCounterUnitKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterUnitKHR.html)
  VkPerformanceCounterUnitKHR {
    VK_PERFORMANCE_COUNTER_UNIT_GENERIC_KHR = 0,
    VK_PERFORMANCE_COUNTER_UNIT_PERCENTAGE_KHR = 1,
    VK_PERFORMANCE_COUNTER_UNIT_NANOSECONDS_KHR = 2,
    VK_PERFORMANCE_COUNTER_UNIT_BYTES_KHR = 3,
    VK_PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND_KHR = 4,
    VK_PERFORMANCE_COUNTER_UNIT_KELVIN_KHR = 5,
    VK_PERFORMANCE_COUNTER_UNIT_WATTS_KHR = 6,
    VK_PERFORMANCE_COUNTER_UNIT_VOLTS_KHR = 7,
    VK_PERFORMANCE_COUNTER_UNIT_AMPS_KHR = 8,
    VK_PERFORMANCE_COUNTER_UNIT_HERTZ_KHR = 9,
    VK_PERFORMANCE_COUNTER_UNIT_CYCLES_KHR = 10,
  }
}

enumeration! {
  /// [VkPerformanceCounterStorageKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterStorageKHR.html)
  VkPerformanceCounterStorageKHR {
    VK_PERFORMANCE_COUNTER_STORAGE_INT32_KHR = 0,
    VK_PERFORMANCE_COUNTER_STORAGE_INT64_KHR = 1,
    VK_PERFORMANCE_COUNTER_STORAGE_UINT32_KHR = 2,
    VK_PERFORMANCE_COUNTER_STORAGE_UINT64_KHR = 3,
    VK_PERFORMANCE_COUNTER_STORAGE_FLOAT32_KHR = 4,
    VK_PERFORMANCE_COUNTER_STORAGE_FLOAT64_KHR = 5,
  }
}

enumeration! {
  /// [VkPerformanceConfigurationTypeINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceConfigurationTypeINTEL.html)
  VkPerformanceConfigurationTypeINTEL {
    VK_PERFORMANCE_CONFIGURATION_TYPE_COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL = 0,
  }
}

enumeration! {
  /// [VkQueryPoolSamplingModeINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolSamplingModeINTEL.html)
  VkQueryPoolSamplingModeINTEL {
    VK_QUERY_POOL_SAMPLING_MODE_MANUAL_INTEL = 0,
  }
}

enumeration! {
  /// [VkPerformanceOverrideTypeINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceOverrideTypeINTEL.html)
  VkPerformanceOverrideTypeINTEL {
    VK_PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE_INTEL = 0,
    VK_PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES_INTEL = 1,
  }
}

enumeration! {
  /// [VkPerformanceParameterTypeINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceParameterTypeINTEL.html)
  VkPerformanceParameterTypeINTEL {
    VK_PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED_INTEL = 0,
    VK_PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS_INTEL = 1,
  }
}

enumeration! {
  /// [VkPerformanceValueTypeINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceValueTypeINTEL.html)
  VkPerformanceValueTypeINTEL {
    VK_PERFORMANCE_VALUE_TYPE_UINT32_INTEL = 0,
    VK_PERFORMANCE_VALUE_TYPE_UINT64_INTEL = 1,
    VK_PERFORMANCE_VALUE_TYPE_FLOAT_INTEL = 2,
    VK_PERFORMANCE_VALUE_TYPE_BOOL_INTEL = 3,
    VK_PERFORMANCE_VALUE_TYPE_STRING_INTEL = 4,
  }
}

enumeration! {
  /// [VkShaderFloatControlsIndependence](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderFloatControlsIndependence.html)
  VkShaderFloatControlsIndependence {
    VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY = 0,
    VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL = 1,
    VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE = 2,
  }
}

enumeration! {
  /// [VkPipelineExecutableStatisticFormatKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineExecutableStatisticFormatKHR.html)
  VkPipelineExecutableStatisticFormatKHR {
    VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR = 0,
    VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR = 1,
    VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR = 2,
    VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR = 3,
  }
}

enumeration! {
  /// [VkLineRasterizationModeEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkLineRasterizationModeEXT.html)
  VkLineRasterizationModeEXT {
    VK_LINE_RASTERIZATION_MODE_DEFAULT_EXT = 0,
    VK_LINE_RASTERIZATION_MODE_RECTANGULAR_EXT = 1,
    VK_LINE_RASTERIZATION_MODE_BRESENHAM_EXT = 2,
    VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH_EXT = 3,
  }
}

enumeration! {
  /// [VkFragmentShadingRateCombinerOpKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFragmentShadingRateCombinerOpKHR.html)
  VkFragmentShadingRateCombinerOpKHR {
    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR = 0,
    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR = 1,
    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MIN_KHR = 2,
    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MAX_KHR = 3,
    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR = 4,
  }
}

enumeration! {
  /// [VkFragmentShadingRateNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFragmentShadingRateNV.html)
  VkFragmentShadingRateNV {
    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV = 0,
    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS_NV = 1,
    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS_NV = 4,
    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS_NV = 5,
    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS_NV = 6,
    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS_NV = 9,
    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS_NV = 10,
    VK_FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV = 11,
    VK_FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV = 12,
    VK_FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV = 13,
    VK_FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV = 14,
    VK_FRAGMENT_SHADING_RATE_NO_INVOCATIONS_NV = 15,
  }
}

enumeration! {
  /// [VkFragmentShadingRateTypeNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFragmentShadingRateTypeNV.html)
  VkFragmentShadingRateTypeNV {
    VK_FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE_NV = 0,
    VK_FRAGMENT_SHADING_RATE_TYPE_ENUMS_NV = 1,
  }
}

pub type VkAccelerationStructureTypeNV = VkAccelerationStructureTypeKHR;
pub type VkChromaLocationKHR = VkChromaLocation;
pub type VkCopyAccelerationStructureModeNV = VkCopyAccelerationStructureModeKHR;
pub type VkDescriptorUpdateTemplateTypeKHR = VkDescriptorUpdateTemplateType;
pub type VkDriverIdKHR = VkDriverId;
pub type VkGeometryTypeNV = VkGeometryTypeKHR;
pub type VkPointClippingBehaviorKHR = VkPointClippingBehavior;
pub type VkRayTracingShaderGroupTypeNV = VkRayTracingShaderGroupTypeKHR;
pub type VkSamplerReductionModeEXT = VkSamplerReductionMode;
pub type VkSamplerYcbcrModelConversionKHR = VkSamplerYcbcrModelConversion;
pub type VkSamplerYcbcrRangeKHR = VkSamplerYcbcrRange;
pub type VkSemaphoreTypeKHR = VkSemaphoreType;
pub type VkShaderFloatControlsIndependenceKHR = VkShaderFloatControlsIndependence;
pub type VkTessellationDomainOriginKHR = VkTessellationDomainOrigin;
