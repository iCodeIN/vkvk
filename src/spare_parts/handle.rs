use super::*;

define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NV`]
  ///
  /// [VkIndirectCommandsLayoutNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectCommandsLayoutNV.html)
  VkIndirectCommandsLayoutNV
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE`]
  ///
  /// [VkDescriptorUpdateTemplate](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplate.html)
  VkDescriptorUpdateTemplate
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION`]
  ///
  /// [VkSamplerYcbcrConversion](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrConversion.html)
  VkSamplerYcbcrConversion
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_VALIDATION_CACHE_EXT`]
  ///
  /// [VkValidationCacheEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationCacheEXT.html)
  VkValidationCacheEXT
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR`]
  ///
  /// [VkAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureKHR.html)
  VkAccelerationStructureKHR
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV`]
  ///
  /// [VkAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureNV.html)
  VkAccelerationStructureNV
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_PERFORMANCE_CONFIGURATION_INTEL`]
  ///
  /// [VkPerformanceConfigurationINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceConfigurationINTEL.html)
  VkPerformanceConfigurationINTEL
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_DEFERRED_OPERATION_KHR`]
  ///
  /// [VkDeferredOperationKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeferredOperationKHR.html)
  VkDeferredOperationKHR
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_PRIVATE_DATA_SLOT_EXT`]
  ///
  /// [VkPrivateDataSlotEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPrivateDataSlotEXT.html)
  VkPrivateDataSlotEXT
);

define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkPhysicalDevice`]
  /// * ObjTypeEnum: [`VK_OBJECT_TYPE_DISPLAY_KHR`]
  /// [VkDisplayKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayKHR.html)
  VkDisplayKHR
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDisplayKHR`]
  /// * ObjTypeEnum: [`VK_OBJECT_TYPE_DISPLAY_MODE_KHR`]
  /// [VkDisplayModeKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayModeKHR.html)
  VkDisplayModeKHR
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkInstance`]
  /// * ObjTypeEnum: [`VK_OBJECT_TYPE_SURFACE_KHR`]
  /// [VkSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceKHR.html)
  VkSurfaceKHR
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkSurfaceKHR`]
  /// * ObjTypeEnum: [`VK_OBJECT_TYPE_SWAPCHAIN_KHR`]
  /// [VkSwapchainKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainKHR.html)
  VkSwapchainKHR
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkInstance`]
  /// * ObjTypeEnum: [`VK_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT`]
  /// [VkDebugReportCallbackEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugReportCallbackEXT.html)
  VkDebugReportCallbackEXT
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkInstance`]
  /// * ObjTypeEnum: [`VK_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT`]
  /// [VkDebugUtilsMessengerEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessengerEXT.html)
  VkDebugUtilsMessengerEXT
);

pub type VkDescriptorUpdateTemplateKHR = VkDescriptorUpdateTemplate;
pub type VkSamplerYcbcrConversionKHR = VkSamplerYcbcrConversion;
