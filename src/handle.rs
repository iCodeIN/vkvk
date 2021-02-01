use super::*;

macro_rules! define_handle {
  ($(#[$m:meta])* $handle:ident) => {
    #[derive(PartialEq, Eq, Hash)]
    #[repr(transparent)]
    $(#[$m])*
    pub struct $handle(*mut c_void);
    impl Default for $handle {
      fn default() -> Self {
        Self::null()
      }
    }
    impl $handle {
      pub const fn null() -> Self {
        Self(core::ptr::null_mut())
      }
      pub fn is_null(&self) -> bool {
        self.0.is_null()
      }
    }
  };
}
define_handle!(
  /// Handle to an Instance.
  ///
  /// * Parent: none
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_INSTANCE`]
  ///
  /// [VkInstance](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkInstance.html)
  VkInstance
);
define_handle!(
  /// Handle to a Physical Device.
  ///
  /// * Parent: [`VkInstance`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_PHYSICAL_DEVICE`]
  ///
  /// [VkPhysicalDevice](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevice.html)
  VkPhysicalDevice
);
define_handle!(
  /// Handle to a Device.
  ///
  /// * Parent: [`VkPhysicalDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_DEVICE`]
  ///
  /// [VkDevice](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDevice.html)
  VkDevice
);
define_handle!(
  /// Handle to a Queue.
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_QUEUE`]
  ///
  /// [VkQueue](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueue.html)
  VkQueue
);

macro_rules! define_non_dispatchable_handle {
  ($(#[$m: meta])*$handle:ident) => {
    #[repr(transparent)]
    $(#[$m])*
    pub struct $handle(u64);
  };
}

define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_DEVICE_MEMORY`]
  ///
  /// [VkDeviceMemory](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceMemory.html)
  VkDeviceMemory
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_COMMAND_POOL`]
  ///
  /// [VkCommandPool](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandPool.html)
  VkCommandPool
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_BUFFER`]
  ///
  /// [VkBuffer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBuffer.html)
  VkBuffer
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_BUFFER_VIEW`]
  ///
  /// [VkBufferView](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferView.html)
  VkBufferView
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_IMAGE`]
  ///
  /// [VkImage](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImage.html)
  VkImage
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_IMAGE_VIEW`]
  ///
  /// [VkImageView](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageView.html)
  VkImageView
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_SHADER_MODULE`]
  ///
  /// [VkShaderModule](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderModule.html)
  VkShaderModule
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_PIPELINE`]
  ///
  /// [VkPipeline](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipeline.html)
  VkPipeline
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_PIPELINE_LAYOUT`]
  ///
  /// [VkPipelineLayout](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineLayout.html)
  VkPipelineLayout
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_SAMPLER`]
  ///
  /// [VkSampler](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSampler.html)
  VkSampler
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDescriptorPool`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_DESCRIPTOR_SET`]
  ///
  /// [VkDescriptorSet](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSet.html)
  VkDescriptorSet
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT`]
  ///
  /// [VkDescriptorSetLayout](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayout.html)
  VkDescriptorSetLayout
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_DESCRIPTOR_POOL`]
  ///
  /// [VkDescriptorPool](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorPool.html)
  VkDescriptorPool
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_FENCE`]
  ///
  /// [VkFence](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFence.html)
  VkFence
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_SEMAPHORE`]
  ///
  /// [VkSemaphore](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphore.html)
  VkSemaphore
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_EVENT`]
  ///
  /// [VkEvent](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkEvent.html)
  VkEvent
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_QUERY_POOL`]
  ///
  /// [VkQueryPool](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPool.html)
  VkQueryPool
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_FRAMEBUFFER`]
  ///
  /// [VkFramebuffer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebuffer.html)
  VkFramebuffer
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_RENDER_PASS`]
  ///
  /// [VkRenderPass](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPass.html)
  VkRenderPass
);
define_non_dispatchable_handle!(
  ///
  /// * Parent: [`VkDevice`]
  /// * ObjectTypeEnum: [`VK_OBJECT_TYPE_PIPELINE_CACHE`]
  ///
  /// [VkPipelineCache](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCache.html)
  VkPipelineCache
);
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

pub type VkDescriptorUpdateTemplateKHR = VkDescriptorUpdateTemplate;
pub type VkSamplerYcbcrConversionKHR = VkSamplerYcbcrConversion;
