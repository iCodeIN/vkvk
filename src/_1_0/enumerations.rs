//! Vulkan types where you're supposed to use one among a list of possible
//! variants.
//!
//! The difference from a Rust `enum` is that illegal bit patterns are *not*
//! actually prohibited from existing.

use crate::prelude::*;

mod vk_bool;
pub use vk_bool::*;

mod vk_result;
pub use vk_result::*;

mod vk_structure_type;
pub use vk_structure_type::*;

mod vk_format;
pub use vk_format::*;

vk_enumeration! {
  /// [VkObjectType](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkObjectType.html)
  ///
  /// Enums to track objects of various types - also see objtypeenum attributes on type tags.
  VkObjectType {
    VK_OBJECT_TYPE_UNKNOWN = 0,
    VK_OBJECT_TYPE_INSTANCE = 1,
    VK_OBJECT_TYPE_PHYSICAL_DEVICE = 2,
    VK_OBJECT_TYPE_DEVICE = 3,
    VK_OBJECT_TYPE_QUEUE = 4,
    VK_OBJECT_TYPE_SEMAPHORE = 5,
    VK_OBJECT_TYPE_COMMAND_BUFFER = 6,
    VK_OBJECT_TYPE_FENCE = 7,
    VK_OBJECT_TYPE_DEVICE_MEMORY = 8,
    VK_OBJECT_TYPE_BUFFER = 9,
    VK_OBJECT_TYPE_IMAGE = 10,
    VK_OBJECT_TYPE_EVENT = 11,
    VK_OBJECT_TYPE_QUERY_POOL = 12,
    VK_OBJECT_TYPE_BUFFER_VIEW = 13,
    VK_OBJECT_TYPE_IMAGE_VIEW = 14,
    VK_OBJECT_TYPE_SHADER_MODULE = 15,
    VK_OBJECT_TYPE_PIPELINE_CACHE = 16,
    VK_OBJECT_TYPE_PIPELINE_LAYOUT = 17,
    VK_OBJECT_TYPE_RENDER_PASS = 18,
    VK_OBJECT_TYPE_PIPELINE = 19,
    VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT = 20,
    VK_OBJECT_TYPE_SAMPLER = 21,
    VK_OBJECT_TYPE_DESCRIPTOR_POOL = 22,
    VK_OBJECT_TYPE_DESCRIPTOR_SET = 23,
    VK_OBJECT_TYPE_FRAMEBUFFER = 24,
    VK_OBJECT_TYPE_COMMAND_POOL = 25,
    /// Provided by `VK_VERSION_1_1`
    VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION = 1000156000,
    /// Provided by `VK_VERSION_1_1`
    VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE = 1000085000,
    /// Provided by `VK_KHR_surface`
    VK_OBJECT_TYPE_SURFACE_KHR = 1000000000,
    /// Provided by `VK_KHR_swapchain`
    VK_OBJECT_TYPE_SWAPCHAIN_KHR = 1000001000,
    /// Provided by `VK_KHR_display`
    VK_OBJECT_TYPE_DISPLAY_KHR = 1000002000,
    /// Provided by `VK_KHR_display`
    VK_OBJECT_TYPE_DISPLAY_MODE_KHR = 1000002001,
    /// Provided by `VK_EXT_debug_report`
    VK_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT = 1000011000,
    /// Provided by `VK_EXT_debug_utils`
    VK_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT = 1000128000,
    /// Provided by `VK_KHR_acceleration_structure`
    VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR = 1000150000,
    /// Provided by `VK_EXT_validation_cache`
    VK_OBJECT_TYPE_VALIDATION_CACHE_EXT = 1000160000,
    /// Provided by `VK_NV_ray_tracing`
    VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV = 1000165000,
    /// Provided by `VK_INTEL_performance_query`
    VK_OBJECT_TYPE_PERFORMANCE_CONFIGURATION_INTEL = 1000210000,
    /// Provided by `VK_KHR_deferred_host_operations`
    VK_OBJECT_TYPE_DEFERRED_OPERATION_KHR = 1000268000,
    /// Provided by `VK_NV_device_generated_commands`
    VK_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NV = 1000277000,
    /// Provided by `VK_EXT_private_data`
    VK_OBJECT_TYPE_PRIVATE_DATA_SLOT_EXT = 1000295000,
  }
}
/// Provided by `VK_KHR_descriptor_update_template`
pub const VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR: VkObjectType = VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE;
/// Provided by `VK_KHR_sampler_ycbcr_conversion`
pub const VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR: VkObjectType = VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION;

vk_enumeration! {
  /// [VkVendorId](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVendorId.html)
  VkVendorId {
    /// Vivante vendor ID
    VK_VENDOR_ID_VIV = 0x10001,
    /// VeriSilicon vendor ID
    VK_VENDOR_ID_VSI = 0x10002,
    /// Kazan Software Renderer
    VK_VENDOR_ID_KAZAN = 0x10003,
    /// Codeplay Software Ltd. vendor ID
    VK_VENDOR_ID_CODEPLAY = 0x10004,
    /// Mesa vendor ID
    VK_VENDOR_ID_MESA = 0x10005,
    /// PoCL vendor ID
    VK_VENDOR_ID_POCL = 0x10006,
  }
}

vk_enumeration! {
  /// [VkImageLayout](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageLayout.html)
  VkImageLayout {
    /// Implicit layout an image is when its contents are undefined due to various reasons (e.g. right after creation)
    VK_IMAGE_LAYOUT_UNDEFINED = 0,
    /// General layout when image can be used for any kind of access
    VK_IMAGE_LAYOUT_GENERAL = 1,
    /// Optimal layout when image is only used for color attachment read/write
    VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL = 2,
    /// Optimal layout when image is only used for depth/stencil attachment read/write
    VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL = 3,
    /// Optimal layout when image is used for read only depth/stencil attachment and shader access
    VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL = 4,
    /// Optimal layout when image is used for read only shader access
    VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL = 5,
    /// Optimal layout when image is used only as source of transfer operations
    VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL = 6,
    /// Optimal layout when image is used only as destination of transfer operations
    VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL = 7,
    /// Initial layout used when the data is populated by the CPU
    VK_IMAGE_LAYOUT_PREINITIALIZED = 8,
    /// Provided by `VK_VERSION_1_1`
    VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL = 1000117000,
    /// Provided by `VK_VERSION_1_1`
    VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL = 1000117001,
    /// Provided by `VK_VERSION_1_2`
    VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL = 1000241000,
    /// Provided by `VK_VERSION_1_2`
    VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL = 1000241001,
    /// Provided by `VK_VERSION_1_2`
    VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL = 1000241002,
    /// Provided by `VK_VERSION_1_2`
    VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL = 1000241003,
    /// Provided by `VK_KHR_swapchain`
    VK_IMAGE_LAYOUT_PRESENT_SRC_KHR = 1000001002,
    /// Provided by `VK_KHR_shared_presentable_image`
    VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR = 1000111000,
    /// Provided by `VK_NV_shading_rate_image`
    VK_IMAGE_LAYOUT_SHADING_RATE_OPTIMAL_NV = 1000164003,
    /// Provided by `VK_EXT_fragment_density_map`
    VK_IMAGE_LAYOUT_FRAGMENT_DENSITY_MAP_OPTIMAL_EXT = 1000218000,
    /// Provided by `VK_KHR_synchronization2`
    VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL_KHR = 1000314000,
    /// Provided by `VK_KHR_synchronization2`
    VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL_KHR = 1000314001,
  }
}

/// Provided by `VK_KHR_maintenance2`
pub const VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR: VkImageLayout = VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL;
/// Provided by `VK_KHR_maintenance2`
pub const VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR: VkImageLayout = VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL;
/// Provided by `VK_KHR_fragment_shading_rate`
pub const VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR: VkImageLayout = VK_IMAGE_LAYOUT_SHADING_RATE_OPTIMAL_NV;
/// Provided by `VK_KHR_separate_depth_stencil_layouts`
pub const VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL_KHR: VkImageLayout = VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL;
/// Provided by `VK_KHR_separate_depth_stencil_layouts`
pub const VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL_KHR: VkImageLayout = VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL;
/// Provided by `VK_KHR_separate_depth_stencil_layouts`
pub const VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL_KHR: VkImageLayout = VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL;
/// Provided by `VK_KHR_separate_depth_stencil_layouts`
pub const VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL_KHR: VkImageLayout = VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL;

vk_enumeration! {
  /// [VkPipelineCacheHeaderVersion](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCacheHeaderVersion.html)
  VkPipelineCacheHeaderVersion {
    VK_PIPELINE_CACHE_HEADER_VERSION_ONE = 1,
  }
}

vk_enumeration! {
  /// [VkImageTiling](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageTiling.html)
  VkImageTiling {
    VK_IMAGE_TILING_OPTIMAL = 0,
    VK_IMAGE_TILING_LINEAR = 1,
    /// Provided by `VK_EXT_image_drm_format_modifier`
    VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT = 1000158000,
  }
}

vk_enumeration! {
  /// [VkImageType](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageType.html)
  VkImageType {
    VK_IMAGE_TYPE_1D = 0,
    VK_IMAGE_TYPE_2D = 1,
    VK_IMAGE_TYPE_3D = 2,
  }
}

vk_enumeration! {
  /// [VkInternalAllocationType](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkInternalAllocationType.html)
  VkInternalAllocationType {
    VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE = 0,
  }
}

vk_enumeration! {
  /// [VkPhysicalDeviceType](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceType.html)
  VkPhysicalDeviceType {
    VK_PHYSICAL_DEVICE_TYPE_OTHER = 0,
    VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU = 1,
    VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU = 2,
    VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU = 3,
    VK_PHYSICAL_DEVICE_TYPE_CPU = 4,
  }
}

vk_enumeration! {
  /// [VkSystemAllocationScope](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSystemAllocationScope.html)
  VkSystemAllocationScope {
    VK_SYSTEM_ALLOCATION_SCOPE_COMMAND = 0,
    VK_SYSTEM_ALLOCATION_SCOPE_OBJECT = 1,
    VK_SYSTEM_ALLOCATION_SCOPE_CACHE = 2,
    VK_SYSTEM_ALLOCATION_SCOPE_DEVICE = 3,
    VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE = 4,
  }
}

vk_enumeration! {
  /// [VkQueryType](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryType.html)
  VkQueryType {
    VK_QUERY_TYPE_OCCLUSION = 0,
    /// Optional
    VK_QUERY_TYPE_PIPELINE_STATISTICS = 1,
    VK_QUERY_TYPE_TIMESTAMP = 2,
    //// Provided by `VK_EXT_transform_feedback`
    VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT = 1000028004,
    //// Provided by `VK_KHR_performance_query`
    VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR = 1000116000,
    //// Provided by `VK_KHR_acceleration_structure`
    VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR = 1000150000,
    //// Provided by `VK_KHR_acceleration_structure`
    VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR = 1000150001,
    //// Provided by `VK_NV_ray_tracing`
    VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV = 1000165000,
    //// Provided by `VK_INTEL_performance_query`
    VK_QUERY_TYPE_PERFORMANCE_QUERY_INTEL = 1000210000,
  }
}

vk_enumeration! {
  /// [VkSharingMode](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSharingMode.html)
  VkSharingMode {
    VK_SHARING_MODE_EXCLUSIVE = 0,
    VK_SHARING_MODE_CONCURRENT = 1,
  }
}

vk_enumeration! {
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

vk_enumeration! {
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

vk_enumeration! {
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

vk_enumeration! {
  /// [VkBlendOp](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBlendOp.html)
  VkBlendOp {
    VK_BLEND_OP_ADD = 0,
    VK_BLEND_OP_SUBTRACT = 1,
    VK_BLEND_OP_REVERSE_SUBTRACT = 2,
    VK_BLEND_OP_MIN = 3,
    VK_BLEND_OP_MAX = 4,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_ZERO_EXT = 1000148000,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_SRC_EXT = 1000148001,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_DST_EXT = 1000148002,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_SRC_OVER_EXT = 1000148003,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_DST_OVER_EXT = 1000148004,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_SRC_IN_EXT = 1000148005,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_DST_IN_EXT = 1000148006,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_SRC_OUT_EXT = 1000148007,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_DST_OUT_EXT = 1000148008,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_SRC_ATOP_EXT = 1000148009,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_DST_ATOP_EXT = 1000148010,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_XOR_EXT = 1000148011,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_MULTIPLY_EXT = 1000148012,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_SCREEN_EXT = 1000148013,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_OVERLAY_EXT = 1000148014,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_DARKEN_EXT = 1000148015,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_LIGHTEN_EXT = 1000148016,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_COLORDODGE_EXT = 1000148017,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_COLORBURN_EXT = 1000148018,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_HARDLIGHT_EXT = 1000148019,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_SOFTLIGHT_EXT = 1000148020,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_DIFFERENCE_EXT = 1000148021,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_EXCLUSION_EXT = 1000148022,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_INVERT_EXT = 1000148023,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_INVERT_RGB_EXT = 1000148024,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_LINEARDODGE_EXT = 1000148025,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_LINEARBURN_EXT = 1000148026,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_VIVIDLIGHT_EXT = 1000148027,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_LINEARLIGHT_EXT = 1000148028,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_PINLIGHT_EXT = 1000148029,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_HARDMIX_EXT = 1000148030,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_HSL_HUE_EXT = 1000148031,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_HSL_SATURATION_EXT = 1000148032,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_HSL_COLOR_EXT = 1000148033,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_HSL_LUMINOSITY_EXT = 1000148034,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_PLUS_EXT = 1000148035,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_PLUS_CLAMPED_EXT = 1000148036,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_PLUS_CLAMPED_ALPHA_EXT = 1000148037,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_PLUS_DARKER_EXT = 1000148038,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_MINUS_EXT = 1000148039,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_MINUS_CLAMPED_EXT = 1000148040,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_CONTRAST_EXT = 1000148041,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_INVERT_OVG_EXT = 1000148042,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_RED_EXT = 1000148043,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_GREEN_EXT = 1000148044,
    /// Provided by `VK_EXT_blend_operation_advanced`
    VK_BLEND_OP_BLUE_EXT = 1000148045,
  }
}

vk_enumeration! {
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

vk_enumeration! {
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
    /// Provided by `VK_NV_clip_space_w_scaling`
    VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV = 1000087000,
    /// Provided by `VK_EXT_discard_rectangles`
    VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT = 1000099000,
    /// Provided by `VK_EXT_sample_locations`
    VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT = 1000143000,
    /// Provided by `VK_KHR_ray_tracing_pipeline`
    VK_DYNAMIC_STATE_RAY_TRACING_PIPELINE_STACK_SIZE_KHR = 1000347000,
    /// Provided by `VK_NV_shading_rate_image`
    VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV = 1000164004,
    /// Provided by `VK_NV_shading_rate_image`
    VK_DYNAMIC_STATE_VIEWPORT_COARSE_SAMPLE_ORDER_NV = 1000164006,
    /// Provided by `VK_NV_scissor_exclusive`
    VK_DYNAMIC_STATE_EXCLUSIVE_SCISSOR_NV = 1000205001,
    /// Provided by `VK_KHR_fragment_shading_rate`
    VK_DYNAMIC_STATE_FRAGMENT_SHADING_RATE_KHR = 1000226000,
    /// Provided by `VK_EXT_line_rasterization`
    VK_DYNAMIC_STATE_LINE_STIPPLE_EXT = 1000259000,
    /// Provided by `VK_EXT_extended_dynamic_state`
    VK_DYNAMIC_STATE_CULL_MODE_EXT = 1000267000,
    /// Provided by `VK_EXT_extended_dynamic_state`
    VK_DYNAMIC_STATE_FRONT_FACE_EXT = 1000267001,
    /// Provided by `VK_EXT_extended_dynamic_state`
    VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY_EXT = 1000267002,
    /// Provided by `VK_EXT_extended_dynamic_state`
    VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT_EXT = 1000267003,
    /// Provided by `VK_EXT_extended_dynamic_state`
    VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT_EXT = 1000267004,
    /// Provided by `VK_EXT_extended_dynamic_state`
    VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT = 1000267005,
    /// Provided by `VK_EXT_extended_dynamic_state`
    VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE_EXT = 1000267006,
    /// Provided by `VK_EXT_extended_dynamic_state`
    VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE_EXT = 1000267007,
    /// Provided by `VK_EXT_extended_dynamic_state`
    VK_DYNAMIC_STATE_DEPTH_COMPARE_OP_EXT = 1000267008,
    /// Provided by `VK_EXT_extended_dynamic_state`
    VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE_EXT = 1000267009,
    /// Provided by `VK_EXT_extended_dynamic_state`
    VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE_EXT = 1000267010,
    /// Provided by `VK_EXT_extended_dynamic_state`
    VK_DYNAMIC_STATE_STENCIL_OP_EXT = 1000267011,
  }
}

vk_enumeration! {
  /// [VkFrontFace](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFrontFace.html)
  VkFrontFace {
    VK_FRONT_FACE_COUNTER_CLOCKWISE = 0,
    VK_FRONT_FACE_CLOCKWISE = 1,
  }
}

vk_enumeration! {
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

vk_enumeration! {
  /// [VkPolygonMode](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPolygonMode.html)
  VkPolygonMode {
    VK_POLYGON_MODE_FILL = 0,
    VK_POLYGON_MODE_LINE = 1,
    VK_POLYGON_MODE_POINT = 2,
    /// Provided by `VK_NV_fill_rectangle`
    VK_POLYGON_MODE_FILL_RECTANGLE_NV = 1000153000,
  }
}

vk_enumeration! {
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

vk_enumeration! {
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

vk_enumeration! {
  /// [VkVertexInputRate](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputRate.html)
  VkVertexInputRate {
    VK_VERTEX_INPUT_RATE_VERTEX = 0,
    VK_VERTEX_INPUT_RATE_INSTANCE = 1,
  }
}

vk_enumeration! {
  /// [VkBorderColor](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBorderColor.html)
  VkBorderColor {
    VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK = 0,
    VK_BORDER_COLOR_INT_TRANSPARENT_BLACK = 1,
    VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK = 2,
    VK_BORDER_COLOR_INT_OPAQUE_BLACK = 3,
    VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE = 4,
    VK_BORDER_COLOR_INT_OPAQUE_WHITE = 5,
    /// Provided by `VK_EXT_custom_border_color`
    VK_BORDER_COLOR_FLOAT_CUSTOM_EXT = 1000287003,
    /// Provided by `VK_EXT_custom_border_color`
    VK_BORDER_COLOR_INT_CUSTOM_EXT = 1000287004,
  }
}

vk_enumeration! {
  /// [VkFilter](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFilter.html)
  VkFilter {
    VK_FILTER_NEAREST = 0,
    VK_FILTER_LINEAR = 1,
    /// Provided by `VK_IMG_filter_cubic`
      VK_FILTER_CUBIC_IMG = 1000015000,
  }
}
/// Provided by `VK_EXT_filter_cubic`
pub const VK_FILTER_CUBIC_EXT: VkFilter = VK_FILTER_CUBIC_IMG;

vk_enumeration! {
  /// [VkSamplerAddressMode](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerAddressMode.html)
  VkSamplerAddressMode {
    VK_SAMPLER_ADDRESS_MODE_REPEAT = 0,
    VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT = 1,
    VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE = 2,
    VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER = 3,
    // Provided by `VK_VERSION_1_2`, `VK_KHR_sampler_mirror_clamp_to_edge`
    VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE = 4,
  }
}
pub const VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE_KHR: VkSamplerAddressMode = VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE;

vk_enumeration! {
  /// [VkSamplerMipmapMode](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerMipmapMode.html)
  VkSamplerMipmapMode {
    /// Choose nearest mip level
    VK_SAMPLER_MIPMAP_MODE_NEAREST = 0,
    /// Linear filter between mip levels
    VK_SAMPLER_MIPMAP_MODE_LINEAR = 1,
  }
}

vk_enumeration! {
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
    /// Provided by `VK_EXT_inline_uniform_block`
    VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK_EXT = 1000138000,
    /// Provided by `VK_KHR_acceleration_structure`
    VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR = 1000150000,
    /// Provided by `VK_NV_ray_tracing`
    VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV = 1000165000,
    /// Provided by `VK_VALVE_mutable_descriptor_type`
    VK_DESCRIPTOR_TYPE_MUTABLE_VALVE = 1000351000,
  }
}

vk_enumeration! {
  /// [VkAttachmentLoadOp](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentLoadOp.html)
  VkAttachmentLoadOp {
    VK_ATTACHMENT_LOAD_OP_LOAD = 0,
    VK_ATTACHMENT_LOAD_OP_CLEAR = 1,
    VK_ATTACHMENT_LOAD_OP_DONT_CARE = 2,
  }
}

vk_enumeration! {
  /// [VkAttachmentStoreOp](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentStoreOp.html)
  VkAttachmentStoreOp {
    VK_ATTACHMENT_STORE_OP_STORE = 0,
    VK_ATTACHMENT_STORE_OP_DONT_CARE = 1,
    /// Provided by `VK_QCOM_render_pass_store_ops`
    VK_ATTACHMENT_STORE_OP_NONE_QCOM = 1000301000,
  }
}

vk_enumeration! {
  /// [VkPipelineBindPoint](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineBindPoint.html)
  VkPipelineBindPoint {
    VK_PIPELINE_BIND_POINT_GRAPHICS = 0,
    VK_PIPELINE_BIND_POINT_COMPUTE = 1,
    /// Provided by `VK_KHR_ray_tracing_pipeline`
    VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR = 1000165000,
  }
}
// Provided by `VK_NV_ray_tracing`
pub const VK_PIPELINE_BIND_POINT_RAY_TRACING_NV: VkPipelineBindPoint = VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR;

vk_enumeration! {
  /// [VkCommandBufferLevel](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferLevel.html)
  VkCommandBufferLevel {
    VK_COMMAND_BUFFER_LEVEL_PRIMARY = 0,
    VK_COMMAND_BUFFER_LEVEL_SECONDARY = 1,
  }
}

vk_enumeration! {
  /// [VkIndexType](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndexType.html)
  VkIndexType {
    VK_INDEX_TYPE_UINT16 = 0,
    VK_INDEX_TYPE_UINT32 = 1,
    /// Provided by `VK_KHR_acceleration_structure`
    VK_INDEX_TYPE_NONE_KHR = 1000165000,
    /// Provided by `VK_EXT_index_type_uint8`
    VK_INDEX_TYPE_UINT8_EXT = 1000265000,
  }
}
/// Provided by `VK_NV_ray_tracing`
pub const VK_INDEX_TYPE_NONE_NV: VkIndexType = VK_INDEX_TYPE_NONE_KHR;

vk_enumeration! {
  /// [VkSubpassContents](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassContents.html)
  VkSubpassContents {
    VK_SUBPASS_CONTENTS_INLINE = 0,
    VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS = 1,
  }
}
