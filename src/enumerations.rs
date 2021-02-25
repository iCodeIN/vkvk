//! Vulkan types where you're supposed to use one among a list of possible
//! variants.
//!
//! The difference from a Rust `enum` is that illegal bit patterns are *not*
//! actually prohibited from existing.

use super::*;

mod vk_bool;
pub use vk_bool::*;

mod vk_result;
pub use vk_result::*;

mod vk_structure_type;
pub use vk_structure_type::*;

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
