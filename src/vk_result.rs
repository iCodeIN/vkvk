use super::*;

vk_enumeration!(
  /// API result codes.
  ///
  /// * Successful codes (non-negative values)
  /// * Error codes (negative values)
  ///
  /// [VkResult](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkResult.html)
  VkResult {
    /// Command completed successfully
    VK_SUCCESS = 0,
    /// A fence or query has not yet completed
    VK_NOT_READY = 1,
    /// A wait operation has not completed in the specified time
    VK_TIMEOUT = 2,
    /// An event is signaled
    VK_EVENT_SET = 3,
    /// An event is unsignaled
    VK_EVENT_RESET = 4,
    /// A return array was too small for the result
    VK_INCOMPLETE = 5,
    /// A host memory allocation has failed
    VK_ERROR_OUT_OF_HOST_MEMORY = -1,
    /// A device memory allocation has failed
    VK_ERROR_OUT_OF_DEVICE_MEMORY = -2,
    /// Initialization of a object has failed
    VK_ERROR_INITIALIZATION_FAILED = -3,
    /// The logical device has been lost.
    ///
    /// [Spec 5.2.3: Lost Device](https://renderdoc.org/vkspec_chunked/chap6.html#devsandqueues-lost-device)
    VK_ERROR_DEVICE_LOST = -4,
    /// Mapping of a memory object has failed
    VK_ERROR_MEMORY_MAP_FAILED = -5,
    /// Layer specified does not exist
    VK_ERROR_LAYER_NOT_PRESENT = -6,
    /// Extension specified does not exist
    VK_ERROR_EXTENSION_NOT_PRESENT = -7,
    /// Requested feature is not available on this device
    VK_ERROR_FEATURE_NOT_PRESENT = -8,
    /// Unable to find a Vulkan driver
    VK_ERROR_INCOMPATIBLE_DRIVER = -9,
    /// Too many objects of the type have already been created
    VK_ERROR_TOO_MANY_OBJECTS = -10,
    /// Requested format is not supported on this device
    VK_ERROR_FORMAT_NOT_SUPPORTED = -11,
    /// A requested pool allocation has failed due to fragmentation of the pool's memory
    VK_ERROR_FRAGMENTED_POOL = -12,
    /// An unknown error has occurred, due to an implementation or application bug
    VK_ERROR_UNKNOWN = -13,
    // Provided by `VK_VERSION_1_1`
    VK_ERROR_OUT_OF_POOL_MEMORY = -1000069000,
    /// Provided by `VK_VERSION_1_1`
    VK_ERROR_INVALID_EXTERNAL_HANDLE = -1000072003,
    /// Provided by `VK_VERSION_1_2`
    VK_ERROR_FRAGMENTATION = -1000161000,
    /// Provided by `VK_VERSION_1_2`
    VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS = -1000257000,
    /// Provided by `VK_KHR_surface`
    VK_ERROR_SURFACE_LOST_KHR = -1000000000,
    /// Provided by `VK_KHR_surface`
    VK_ERROR_NATIVE_WINDOW_IN_USE_KHR = -1000000001,
    /// Provided by `VK_KHR_swapchain`
    VK_SUBOPTIMAL_KHR = 1000001003,
    /// Provided by `VK_KHR_swapchain`
    VK_ERROR_OUT_OF_DATE_KHR = -1000001004,
    /// Provided by `VK_KHR_display_swapchain`
    VK_ERROR_INCOMPATIBLE_DISPLAY_KHR = -1000003001,
    /// Provided by `VK_EXT_debug_report`
    VK_ERROR_VALIDATION_FAILED_EXT = -1000011001,
    /// Provided by `VK_NV_glsl_shader`
    VK_ERROR_INVALID_SHADER_NV = -1000012000,
    /// Provided by `VK_EXT_image_drm_format_modifier`
    VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT = -1000158000,
    /// Provided by `VK_EXT_global_priority`
    VK_ERROR_NOT_PERMITTED_EXT = -1000174001,
    /// Provided by `VK_EXT_full_screen_exclusive`
    VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT = -1000255000,
    /// Provided by `VK_KHR_deferred_host_operations`
    VK_THREAD_IDLE_KHR = 1000268000,
    /// Provided by `VK_KHR_deferred_host_operations`
    VK_THREAD_DONE_KHR = 1000268001,
    /// Provided by `VK_KHR_deferred_host_operations`
    VK_OPERATION_DEFERRED_KHR = 1000268002,
    /// Provided by `VK_KHR_deferred_host_operations`
    VK_OPERATION_NOT_DEFERRED_KHR = 1000268003,
    /// Provided by `VK_EXT_pipeline_creation_cache_control`
    VK_PIPELINE_COMPILE_REQUIRED_EXT = 1000297000,
  }
);

/// Provided by `VK_KHR_maintenance1`
pub const VK_ERROR_OUT_OF_POOL_MEMORY_KHR: VkResult = VK_ERROR_OUT_OF_POOL_MEMORY;
/// Provided by `VK_KHR_external_memory`
pub const VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR: VkResult = VK_ERROR_INVALID_EXTERNAL_HANDLE;
/// Provided by `VK_EXT_descriptor_indexing`
pub const VK_ERROR_FRAGMENTATION_EXT: VkResult = VK_ERROR_FRAGMENTATION;
/// Provided by `VK_EXT_buffer_device_address`
pub const VK_ERROR_INVALID_DEVICE_ADDRESS_EXT: VkResult = VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS;
/// Provided by `VK_KHR_buffer_device_address`
pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR: VkResult = VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS;
pub const VK_ERROR_PIPELINE_COMPILE_REQUIRED_EXT: VkResult = VK_PIPELINE_COMPILE_REQUIRED_EXT;
