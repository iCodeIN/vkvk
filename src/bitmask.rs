use super::*;

macro_rules! bitmask {
  ($(#[$m:meta])* $mask:ident) => {
    #[derive(PartialEq, Eq, Hash)]
    #[repr(transparent)]
    $(#[$m])*
    pub struct $mask(pub u32);
    impl Copy for $mask {}
    impl Clone for $mask {
      fn clone(&self) -> Self {
        *self
      }
    }
    impl Default for $mask {
      fn default() -> Self {
        Self(0)
      }
    }
    impl core::ops::BitAnd for $mask {
      type Output = Self;
      fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
      }
    }
    impl core::ops::BitAndAssign for $mask {
      fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
      }
    }
    impl core::ops::BitOr for $mask {
      type Output = Self;
      fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
      }
    }
    impl core::ops::BitOrAssign for $mask {
      fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
      }
    }
    impl core::ops::BitXor for $mask {
      type Output = Self;
      fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
      }
    }
    impl core::ops::BitXorAssign for $mask {
      fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
      }
    }
    impl core::ops::Not for $mask {
      type Output = Self;
      fn not(self) -> Self::Output {
        Self(!self.0)
      }
    }
  };
}

bitmask!(VkAccelerationStructureCreateFlagsKHR);
bitmask!(VkAccessFlags);
bitmask!(VkAcquireProfilingLockFlagsKHR);
bitmask!(VkAttachmentDescriptionFlags);
bitmask!(VkBufferCreateFlags);
bitmask!(VkBufferUsageFlags);
bitmask!(VkBufferViewCreateFlags);
bitmask!(VkBuildAccelerationStructureFlagsKHR);
bitmask!(VkColorComponentFlags);
bitmask!(VkCommandBufferResetFlags);
bitmask!(VkCommandBufferUsageFlags);
bitmask!(VkCommandPoolCreateFlags);
bitmask!(VkCommandPoolResetFlags);
bitmask!(VkCullModeFlags);
bitmask!(VkDependencyFlags);
bitmask!(VkDescriptorPoolCreateFlags);
bitmask!(VkDescriptorPoolResetFlags);
bitmask!(VkDescriptorSetLayoutCreateFlags);
bitmask!(VkDescriptorUpdateTemplateCreateFlags);
bitmask!(VkDeviceCreateFlags);
bitmask!(VkDeviceDiagnosticsConfigFlagsNV);
bitmask!(VkDeviceQueueCreateFlags);
bitmask!(VkEventCreateFlags);
bitmask!(VkFenceCreateFlags);
bitmask!(VkFormatFeatureFlags);
bitmask!(VkFramebufferCreateFlags);
bitmask!(VkGeometryFlagsKHR);
bitmask!(VkGeometryInstanceFlagsKHR);
bitmask!(VkImageAspectFlags);
bitmask!(VkImageCreateFlags);
bitmask!(VkImageUsageFlags);
bitmask!(VkImageViewCreateFlags);
bitmask!(VkIndirectCommandsLayoutUsageFlagsNV);
bitmask!(VkIndirectStateFlagsNV);
bitmask!(VkInstanceCreateFlags);
bitmask!(VkMemoryHeapFlags);
bitmask!(VkMemoryMapFlags);
bitmask!(VkMemoryPropertyFlags);
bitmask!(VkPerformanceCounterDescriptionFlagsKHR);
bitmask!(VkPipelineCacheCreateFlags);
bitmask!(VkPipelineColorBlendStateCreateFlags);
bitmask!(VkPipelineCompilerControlFlagsAMD);
bitmask!(VkPipelineCreateFlags);
bitmask!(VkPipelineCreationFeedbackFlagsEXT);
bitmask!(VkPipelineDepthStencilStateCreateFlags);
bitmask!(VkPipelineDynamicStateCreateFlags);
bitmask!(VkPipelineInputAssemblyStateCreateFlags);
bitmask!(VkPipelineLayoutCreateFlags);
bitmask!(VkPipelineMultisampleStateCreateFlags);
bitmask!(VkPipelineRasterizationStateCreateFlags);
bitmask!(VkPipelineShaderStageCreateFlags);
bitmask!(VkPipelineStageFlags);
bitmask!(VkPipelineTessellationStateCreateFlags);
bitmask!(VkPipelineVertexInputStateCreateFlags);
bitmask!(VkPipelineViewportStateCreateFlags);
bitmask!(VkPrivateDataSlotCreateFlagsEXT);
bitmask!(VkQueryControlFlags);
bitmask!(VkQueryPipelineStatisticFlags);
bitmask!(VkQueryPoolCreateFlags);
bitmask!(VkQueryResultFlags);
bitmask!(VkQueueFlags);
bitmask!(VkRenderPassCreateFlags);
bitmask!(VkSampleCountFlags);
bitmask!(VkSamplerCreateFlags);
bitmask!(VkSemaphoreCreateFlags);
bitmask!(VkSemaphoreWaitFlags);
bitmask!(VkShaderCorePropertiesFlagsAMD);
bitmask!(VkShaderModuleCreateFlags);
bitmask!(VkShaderStageFlags);
bitmask!(VkSparseImageFormatFlags);
bitmask!(VkSparseMemoryBindFlags);
bitmask!(VkStencilFaceFlags);
bitmask!(VkSubgroupFeatureFlags);
bitmask!(VkSubpassDescriptionFlags);

pub type VkAccelerationStructureCreateFlagBitsKHR = VkAccelerationStructureCreateFlagsKHR;
pub type VkAccessFlagBits = VkAccessFlags;
pub type VkAcquireProfilingLockFlagBitsKHR = VkAcquireProfilingLockFlagsKHR;
pub type VkAttachmentDescriptionFlagBits = VkAttachmentDescriptionFlags;
pub type VkBufferCreateFlagBits = VkBufferCreateFlags;
pub type VkBufferUsageFlagBits = VkBufferUsageFlags;
pub type VkBuildAccelerationStructureFlagBitsKHR = VkBuildAccelerationStructureFlagsKHR;
pub type VkBuildAccelerationStructureFlagsNV = VkBuildAccelerationStructureFlagsKHR;
pub type VkColorComponentFlagBits = VkColorComponentFlags;
pub type VkCommandBufferResetFlagBits = VkCommandBufferResetFlags;
pub type VkCommandBufferUsageFlagBits = VkCommandBufferUsageFlags;
pub type VkCommandPoolCreateFlagBits = VkCommandPoolCreateFlags;
pub type VkCommandPoolResetFlagBits = VkCommandPoolResetFlags;
pub type VkCullModeFlagBits = VkCullModeFlags;
pub type VkDependencyFlagBits = VkDependencyFlags;
pub type VkDescriptorPoolCreateFlagBits = VkDescriptorPoolCreateFlags;
pub type VkDescriptorSetLayoutCreateFlagBits = VkDescriptorSetLayoutCreateFlags;
pub type VkDescriptorUpdateTemplateCreateFlagsKHR = VkDescriptorUpdateTemplateCreateFlags;
pub type VkDeviceDiagnosticsConfigFlagBitsNV = VkDeviceDiagnosticsConfigFlagsNV;
pub type VkDeviceQueueCreateFlagBits = VkDeviceQueueCreateFlags;
pub type VkFenceCreateFlagBits = VkFenceCreateFlags;
pub type VkFormatFeatureFlagBits = VkFormatFeatureFlags;
pub type VkFramebufferCreateFlagBits = VkFramebufferCreateFlags;
pub type VkGeometryFlagBitsKHR = VkGeometryFlagsKHR;
pub type VkGeometryFlagsNV = VkGeometryFlagsKHR;
pub type VkGeometryInstanceFlagBitsKHR = VkGeometryInstanceFlagsKHR;
pub type VkGeometryInstanceFlagsNV = VkGeometryInstanceFlagsKHR;
pub type VkImageAspectFlagBits = VkImageAspectFlags;
pub type VkImageCreateFlagBits = VkImageCreateFlags;
pub type VkImageUsageFlagBits = VkImageUsageFlags;
pub type VkImageViewCreateFlagBits = VkImageViewCreateFlags;
pub type VkIndirectCommandsLayoutUsageFlagBitsNV = VkIndirectCommandsLayoutUsageFlagsNV;
pub type VkIndirectStateFlagBitsNV = VkIndirectStateFlagsNV;
pub type VkMemoryHeapFlagBits = VkMemoryHeapFlags;
pub type VkMemoryPropertyFlagBits = VkMemoryPropertyFlags;
pub type VkPerformanceCounterDescriptionFlagBitsKHR = VkPerformanceCounterDescriptionFlagsKHR;
pub type VkPipelineCacheCreateFlagBits = VkPipelineCacheCreateFlags;
pub type VkPipelineCompilerControlFlagBitsAMD = VkPipelineCompilerControlFlagsAMD;
pub type VkPipelineCreateFlagBits = VkPipelineCreateFlags;
pub type VkPipelineCreationFeedbackFlagBitsEXT = VkPipelineCreationFeedbackFlagsEXT;
pub type VkPipelineShaderStageCreateFlagBits = VkPipelineShaderStageCreateFlags;
pub type VkPipelineStageFlagBits = VkPipelineStageFlags;
pub type VkPrivateDataSlotCreateFlagBitsEXT = VkPrivateDataSlotCreateFlagsEXT;
pub type VkQueryControlFlagBits = VkQueryControlFlags;
pub type VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlags;
pub type VkQueryResultFlagBits = VkQueryResultFlags;
pub type VkQueueFlagBits = VkQueueFlags;
pub type VkRenderPassCreateFlagBits = VkRenderPassCreateFlags;
pub type VkSampleCountFlagBits = VkSampleCountFlags;
pub type VkSamplerCreateFlagBits = VkSamplerCreateFlags;
pub type VkSemaphoreWaitFlagBits = VkSemaphoreWaitFlags;
pub type VkSemaphoreWaitFlagsKHR = VkSemaphoreWaitFlags;
pub type VkShaderCorePropertiesFlagBitsAMD = VkShaderCorePropertiesFlagsAMD;
pub type VkShaderModuleCreateFlagBits = VkShaderModuleCreateFlags;
pub type VkShaderStageFlagBits = VkShaderStageFlags;
pub type VkSparseImageFormatFlagBits = VkSparseImageFormatFlags;
pub type VkSparseMemoryBindFlagBits = VkSparseMemoryBindFlags;
pub type VkStencilFaceFlagBits = VkStencilFaceFlags;
pub type VkSubgroupFeatureFlagBits = VkSubgroupFeatureFlags;
pub type VkSubpassDescriptionFlagBits = VkSubpassDescriptionFlags;
