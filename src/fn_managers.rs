#![allow(non_snake_case)]

use crate::prelude::*;

use core::mem::transmute as t;

/// non-null-function
type NNF = unsafe extern "system" fn();

/// Holds functions from [vkGetInstanceProcAddr][vkGIPA] that **do not** require
/// a [`VkInstance`] to look up.
///
/// These functions are used to create your `VkInstance` in the first place.
///
/// [vkGIPA]: https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetInstanceProcAddr.html
#[derive(Clone, Copy)]
pub struct PreInstanceFns {
  vkGetInstanceProcAddr_p: vkGetInstanceProcAddr_t,
  vkEnumerateInstanceExtensionProperties_p: vkEnumerateInstanceExtensionProperties_t,
  vkEnumerateInstanceLayerProperties_p: vkEnumerateInstanceLayerProperties_t,
  vkCreateInstance_p: vkCreateInstance_t,
  // (1.1) vkEnumerateInstanceVersion
}

impl PreInstanceFns {
  pub unsafe fn load_from(vkGetInstanceProcAddr_p: vkGetInstanceProcAddr_t) -> Result<Self, &'static str> {
    let vkCreateInstance_p = t::<NNF, _>(vkGetInstanceProcAddr_p(VkInstance::null(), b"vkCreateInstance\0".as_ptr()).ok_or("vkCreateInstance")?);
    let vkEnumerateInstanceExtensionProperties_p = t::<NNF, _>(
      vkGetInstanceProcAddr_p(VkInstance::null(), b"vkEnumerateInstanceExtensionProperties\0".as_ptr())
        .ok_or("vkEnumerateInstanceExtensionProperties")?,
    );
    let vkEnumerateInstanceLayerProperties_p = t::<NNF, _>(
      vkGetInstanceProcAddr_p(VkInstance::null(), b"vkEnumerateInstanceLayerProperties\0".as_ptr()).ok_or("vkEnumerateInstanceLayerProperties")?,
    );
    Ok(Self {
      vkGetInstanceProcAddr_p,
      vkEnumerateInstanceExtensionProperties_p,
      vkEnumerateInstanceLayerProperties_p,
      vkCreateInstance_p,
      // (1.1) vkEnumerateInstanceVersion_p
    })
  }

  /// [vkCreateInstance](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateInstance.html)
  pub unsafe fn CreateInstance(
    &self, pCreateInfo: &VkInstanceCreateInfo, pAllocator: Option<&VkAllocationCallbacks>, pInstance: &mut VkInstance,
  ) -> VkResult {
    (self.vkCreateInstance_p)(pCreateInfo, pAllocator, pInstance)
  }

  /// [vkEnumerateInstanceExtensionProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceExtensionProperties.html)
  pub unsafe fn EnumerateInstanceExtensionProperties(
    &self, pLayerName: *const char, pPropertyCount: &mut uint32_t, pProperties: *mut VkExtensionProperties,
  ) -> VkResult {
    (self.vkEnumerateInstanceExtensionProperties_p)(pLayerName, pPropertyCount, pProperties)
  }

  /// [vkEnumerateInstanceLayerProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceLayerProperties.html)
  pub unsafe fn EnumerateInstanceLayerProperties(&self, pPropertyCount: &mut uint32_t, pProperties: *mut VkLayerProperties) -> VkResult {
    (self.vkEnumerateInstanceLayerProperties_p)(pPropertyCount, pProperties)
  }

  /// [vkGetInstanceProcAddr](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetInstanceProcAddr.html)
  pub unsafe fn GetInstanceProcAddr(&self, instance: VkInstance, pName: *const char) -> PFN_vkVoidFunction {
    (self.vkGetInstanceProcAddr_p)(instance, pName)
  }
}

/// Holds functions from [vkGetInstanceProcAddr][vkGIPA] that require a
/// [`VkInstance`] to look up.
///
/// [vkGIPA]: https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetInstanceProcAddr.html
#[derive(Clone, Copy)]
pub struct InstanceFns {
  pre_instance_fns: PreInstanceFns,
  vkDestroyInstance_p: vkDestroyInstance_t,
  vkEnumeratePhysicalDevices_p: vkEnumeratePhysicalDevices_t,
  vkGetPhysicalDeviceFeatures_p: vkGetPhysicalDeviceFeatures_t,
  vkGetPhysicalDeviceFormatProperties_p: vkGetPhysicalDeviceFormatProperties_t,
  vkGetPhysicalDeviceImageFormatProperties_p: vkGetPhysicalDeviceImageFormatProperties_t,
  vkGetPhysicalDeviceProperties_p: vkGetPhysicalDeviceProperties_t,
  vkGetPhysicalDeviceQueueFamilyProperties_p: vkGetPhysicalDeviceQueueFamilyProperties_t,
  vkGetPhysicalDeviceMemoryProperties_p: vkGetPhysicalDeviceMemoryProperties_t,
  vkGetDeviceProcAddr_p: vkGetDeviceProcAddr_t,
  vkCreateDevice_p: vkCreateDevice_t,
  vkDestroyDevice_p: vkDestroyDevice_t,
  vkEnumerateDeviceExtensionProperties_p: vkEnumerateDeviceExtensionProperties_t,
  vkEnumerateDeviceLayerProperties_p: vkEnumerateDeviceLayerProperties_t,
  vkQueueSubmit_p: vkQueueSubmit_t,
  vkQueueWaitIdle_p: vkQueueWaitIdle_t,
  vkDeviceWaitIdle_p: vkDeviceWaitIdle_t,
  vkGetPhysicalDeviceSparseImageFormatProperties_p: vkGetPhysicalDeviceSparseImageFormatProperties_t,
  vkQueueBindSparse_p: vkQueueBindSparse_t,
}

impl core::ops::Deref for InstanceFns {
  type Target = PreInstanceFns;
  fn deref(&self) -> &PreInstanceFns {
    &self.pre_instance_fns
  }
}

impl InstanceFns {
  pub unsafe fn load_from(pif: PreInstanceFns, instance: VkInstance) -> Result<Self, &'static str> {
    let vkDestroyInstance_p = t::<NNF, _>(pif.GetInstanceProcAddr(instance, b"vkDestroyInstance\0".as_ptr()).ok_or("vkDestroyInstance")?);
    let vkEnumeratePhysicalDevices_p =
      t::<NNF, _>(pif.GetInstanceProcAddr(instance, b"vkEnumeratePhysicalDevices\0".as_ptr()).ok_or("vkEnumeratePhysicalDevices")?);
    let vkGetPhysicalDeviceFeatures_p =
      t::<NNF, _>(pif.GetInstanceProcAddr(instance, b"vkGetPhysicalDeviceFeatures\0".as_ptr()).ok_or("vkGetPhysicalDeviceFeatures")?);
    let vkGetPhysicalDeviceFormatProperties_p =
      t::<NNF, _>(pif.GetInstanceProcAddr(instance, b"vkGetPhysicalDeviceFormatProperties\0".as_ptr()).ok_or("vkGetPhysicalDeviceFormatProperties")?);
    let vkGetPhysicalDeviceImageFormatProperties_p = t::<NNF, _>(
      pif.GetInstanceProcAddr(instance, b"vkGetPhysicalDeviceImageFormatProperties\0".as_ptr()).ok_or("vkGetPhysicalDeviceImageFormatProperties")?,
    );
    let vkGetPhysicalDeviceProperties_p =
      t::<NNF, _>(pif.GetInstanceProcAddr(instance, b"vkGetPhysicalDeviceProperties\0".as_ptr()).ok_or("vkGetPhysicalDeviceProperties")?);
    let vkGetPhysicalDeviceQueueFamilyProperties_p = t::<NNF, _>(
      pif.GetInstanceProcAddr(instance, b"vkGetPhysicalDeviceQueueFamilyProperties\0".as_ptr()).ok_or("vkGetPhysicalDeviceQueueFamilyProperties")?,
    );
    let vkGetPhysicalDeviceMemoryProperties_p =
      t::<NNF, _>(pif.GetInstanceProcAddr(instance, b"vkGetPhysicalDeviceMemoryProperties\0".as_ptr()).ok_or("vkGetPhysicalDeviceMemoryProperties")?);
    let vkGetDeviceProcAddr_p = t::<NNF, _>(pif.GetInstanceProcAddr(instance, b"vkGetDeviceProcAddr\0".as_ptr()).ok_or("vkGetDeviceProcAddr")?);
    let vkCreateDevice_p = t::<NNF, _>(pif.GetInstanceProcAddr(instance, b"vkCreateDevice\0".as_ptr()).ok_or("vkCreateDevice")?);
    let vkDestroyDevice_p = t::<NNF, _>(pif.GetInstanceProcAddr(instance, b"vkDestroyDevice\0".as_ptr()).ok_or("vkDestroyDevice")?);
    let vkEnumerateDeviceExtensionProperties_p = t::<NNF, _>(
      pif.GetInstanceProcAddr(instance, b"vkEnumerateDeviceExtensionProperties\0".as_ptr()).ok_or("vkEnumerateDeviceExtensionProperties")?,
    );
    let vkEnumerateDeviceLayerProperties_p =
      t::<NNF, _>(pif.GetInstanceProcAddr(instance, b"vkEnumerateDeviceLayerProperties\0".as_ptr()).ok_or("vkEnumerateDeviceLayerProperties")?);
    let vkQueueSubmit_p = t::<NNF, _>(pif.GetInstanceProcAddr(instance, b"vkQueueSubmit\0".as_ptr()).ok_or("vkQueueSubmit")?);
    let vkQueueWaitIdle_p = t::<NNF, _>(pif.GetInstanceProcAddr(instance, b"vkQueueWaitIdle\0".as_ptr()).ok_or("vkQueueWaitIdle")?);
    let vkDeviceWaitIdle_p = t::<NNF, _>(pif.GetInstanceProcAddr(instance, b"vkDeviceWaitIdle\0".as_ptr()).ok_or("vkDeviceWaitIdle")?);
    let vkGetPhysicalDeviceSparseImageFormatProperties_p = t::<NNF, _>(
      pif
        .GetInstanceProcAddr(instance, b"vkGetPhysicalDeviceSparseImageFormatProperties\0".as_ptr())
        .ok_or("vkGetPhysicalDeviceSparseImageFormatProperties")?,
    );
    let vkQueueBindSparse_p = t::<NNF, _>(pif.GetInstanceProcAddr(instance, b"vkQueueBindSparse\0".as_ptr()).ok_or("vkQueueBindSparse")?);
    Ok(Self {
      pre_instance_fns: pif,
      vkDestroyInstance_p,
      vkEnumeratePhysicalDevices_p,
      vkGetPhysicalDeviceFeatures_p,
      vkGetPhysicalDeviceFormatProperties_p,
      vkGetPhysicalDeviceImageFormatProperties_p,
      vkGetPhysicalDeviceProperties_p,
      vkGetPhysicalDeviceQueueFamilyProperties_p,
      vkGetPhysicalDeviceMemoryProperties_p,
      vkGetDeviceProcAddr_p,
      vkCreateDevice_p,
      vkDestroyDevice_p,
      vkEnumerateDeviceExtensionProperties_p,
      vkEnumerateDeviceLayerProperties_p,
      vkQueueSubmit_p,
      vkQueueWaitIdle_p,
      vkDeviceWaitIdle_p,
      vkGetPhysicalDeviceSparseImageFormatProperties_p,
      vkQueueBindSparse_p,
    })
  }

  /// [vkDestroyInstance](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyInstance.html)
  pub unsafe fn DestroyInstance(&self, instance: VkInstance, pAllocator: Option<&VkAllocationCallbacks>) {
    (self.vkDestroyInstance_p)(instance, pAllocator)
  }

  /// [vkEnumeratePhysicalDevices](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDevices.html)
  pub unsafe fn EnumeratePhysicalDevices(
    &self, instance: VkInstance, pPhysicalDeviceCount: &mut uint32_t, pPhysicalDevices: *mut VkPhysicalDevice,
  ) -> VkResult {
    (self.vkEnumeratePhysicalDevices_p)(instance, pPhysicalDeviceCount, pPhysicalDevices)
  }

  /// [vkGetPhysicalDeviceFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFeatures.html)
  pub unsafe fn GetPhysicalDeviceFeatures(&self, physicalDevice: VkPhysicalDevice, pFeatures: &mut VkPhysicalDeviceFeatures) {
    (self.vkGetPhysicalDeviceFeatures_p)(physicalDevice, pFeatures)
  }

  /// [vkGetPhysicalDeviceFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFormatProperties.html)
  pub unsafe fn GetPhysicalDeviceFormatProperties(
    &self, physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: &mut VkFormatProperties,
  ) {
    (self.vkGetPhysicalDeviceFormatProperties_p)(physicalDevice, format, pFormatProperties)
  }

  /// [vkGetPhysicalDeviceImageFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties.html)
  pub unsafe fn GetPhysicalDeviceImageFormatProperties(
    &self, physicalDevice: VkPhysicalDevice, format: VkFormat, type_: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags,
    flags: VkImageCreateFlags, pImageFormatProperties: &mut VkImageFormatProperties,
  ) -> VkResult {
    (self.vkGetPhysicalDeviceImageFormatProperties_p)(physicalDevice, format, type_, tiling, usage, flags, pImageFormatProperties)
  }

  /// [vkGetPhysicalDeviceProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceProperties.html)
  pub unsafe fn GetPhysicalDeviceProperties(&self, physicalDevice: VkPhysicalDevice, pProperties: &mut VkPhysicalDeviceProperties) {
    (self.vkGetPhysicalDeviceProperties_p)(physicalDevice, pProperties)
  }

  /// [vkGetPhysicalDeviceQueueFamilyProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties.html)
  pub unsafe fn GetPhysicalDeviceQueueFamilyProperties(
    &self, physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: &mut uint32_t, pQueueFamilyProperties: *mut VkQueueFamilyProperties,
  ) {
    (self.vkGetPhysicalDeviceQueueFamilyProperties_p)(physicalDevice, pQueueFamilyPropertyCount, pQueueFamilyProperties)
  }

  /// [vkGetPhysicalDeviceMemoryProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMemoryProperties.html)
  pub unsafe fn GetPhysicalDeviceMemoryProperties(&self, physicalDevice: VkPhysicalDevice, pMemoryProperties: &mut VkPhysicalDeviceMemoryProperties) {
    (self.vkGetPhysicalDeviceMemoryProperties_p)(physicalDevice, pMemoryProperties)
  }

  /// [vkGetDeviceProcAddr](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceProcAddr.html)
  pub unsafe fn GetDeviceProcAddr(&self, device: VkDevice, pName: *const char) -> PFN_vkVoidFunction {
    (self.vkGetDeviceProcAddr_p)(device, pName)
  }

  /// [vkCreateDevice](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDevice.html)
  pub unsafe fn CreateDevice(
    &self, physicalDevice: VkPhysicalDevice, pCreateInfo: &VkDeviceCreateInfo, pAllocator: Option<&VkAllocationCallbacks>, pDevice: &mut VkDevice,
  ) -> VkResult {
    (self.vkCreateDevice_p)(physicalDevice, pCreateInfo, pAllocator, pDevice)
  }

  /// [vkDestroyDevice](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDevice.html)
  pub unsafe fn DestroyDevice(&self, device: VkDevice, pAllocator: Option<&VkAllocationCallbacks>) {
    (self.vkDestroyDevice_p)(device, pAllocator)
  }

  /// [vkEnumerateDeviceExtensionProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateDeviceExtensionProperties.html)
  pub unsafe fn EnumerateDeviceExtensionProperties(
    &self, physicalDevice: VkPhysicalDevice, pLayerName: *const char, pPropertyCount: &mut uint32_t, pProperties: *mut VkExtensionProperties,
  ) -> VkResult {
    (self.vkEnumerateDeviceExtensionProperties_p)(physicalDevice, pLayerName, pPropertyCount, pProperties)
  }

  /// [vkEnumerateDeviceLayerProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateDeviceLayerProperties.html)
  pub unsafe fn EnumerateDeviceLayerProperties(
    &self, physicalDevice: VkPhysicalDevice, pPropertyCount: &mut uint32_t, pProperties: *mut VkLayerProperties,
  ) -> VkResult {
    (self.vkEnumerateDeviceLayerProperties_p)(physicalDevice, pPropertyCount, pProperties)
  }

  /// [vkQueueSubmit](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueSubmit.html)
  pub unsafe fn QueueSubmit(&self, queue: VkQueue, submitCount: uint32_t, pSubmits: *const VkSubmitInfo, fence: VkFence) -> VkResult {
    (self.vkQueueSubmit_p)(queue, submitCount, pSubmits, fence)
  }

  /// [vkQueueWaitIdle](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueWaitIdle.html)
  pub unsafe fn QueueWaitIdle(&self, queue: VkQueue) -> VkResult {
    (self.vkQueueWaitIdle_p)(queue)
  }

  /// [vkDeviceWaitIdle](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDeviceWaitIdle.html)
  pub unsafe fn DeviceWaitIdle(&self, device: VkDevice) -> VkResult {
    (self.vkDeviceWaitIdle_p)(device)
  }

  /// [vkGetPhysicalDeviceSparseImageFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties.html)
  pub unsafe fn GetPhysicalDeviceSparseImageFormatProperties(
    &self, physicalDevice: VkPhysicalDevice, format: VkFormat, type_: VkImageType, samples: VkSampleCountFlagBits, usage: VkImageUsageFlags,
    tiling: VkImageTiling, pPropertyCount: &mut uint32_t, pProperties: *mut VkSparseImageFormatProperties,
  ) {
    (self.vkGetPhysicalDeviceSparseImageFormatProperties_p)(physicalDevice, format, type_, samples, usage, tiling, pPropertyCount, pProperties)
  }

  /// [vkQueueBindSparse](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueBindSparse.html)
  pub unsafe fn QueueBindSparse(&self, queue: VkQueue, bindInfoCount: uint32_t, pBindInfo: *const VkBindSparseInfo, fence: VkFence) -> VkResult {
    (self.vkQueueBindSparse_p)(queue, bindInfoCount, pBindInfo, fence)
  }
}

/// Holds functions that come from a [`vkGetDeviceProcAddr_t`] value.
#[derive(Clone, Copy)]
pub struct DeviceFns {
  vkGetDeviceQueue_p: vkGetDeviceQueue_t,
  vkAllocateMemory_p: vkAllocateMemory_t,
  vkFreeMemory_p: vkFreeMemory_t,
  vkMapMemory_p: vkMapMemory_t,
  vkUnmapMemory_p: vkUnmapMemory_t,
  vkFlushMappedMemoryRanges_p: vkFlushMappedMemoryRanges_t,
  vkInvalidateMappedMemoryRanges_p: vkInvalidateMappedMemoryRanges_t,
  vkGetDeviceMemoryCommitment_p: vkGetDeviceMemoryCommitment_t,
  vkBindBufferMemory_p: vkBindBufferMemory_t,
  vkBindImageMemory_p: vkBindImageMemory_t,
  vkGetBufferMemoryRequirements_p: vkGetBufferMemoryRequirements_t,
  vkGetImageMemoryRequirements_p: vkGetImageMemoryRequirements_t,
  vkGetImageSparseMemoryRequirements_p: vkGetImageSparseMemoryRequirements_t,
  vkCreateFence_p: vkCreateFence_t,
  vkDestroyFence_p: vkDestroyFence_t,
  vkResetFences_p: vkResetFences_t,
  vkGetFenceStatus_p: vkGetFenceStatus_t,
  vkWaitForFences_p: vkWaitForFences_t,
  vkCreateSemaphore_p: vkCreateSemaphore_t,
  vkDestroySemaphore_p: vkDestroySemaphore_t,
  vkCreateEvent_p: vkCreateEvent_t,
  vkDestroyEvent_p: vkDestroyEvent_t,
  vkGetEventStatus_p: vkGetEventStatus_t,
  vkSetEvent_p: vkSetEvent_t,
  vkResetEvent_p: vkResetEvent_t,
  vkCreateQueryPool_p: vkCreateQueryPool_t,
  vkDestroyQueryPool_p: vkDestroyQueryPool_t,
  vkGetQueryPoolResults_p: vkGetQueryPoolResults_t,
  vkCreateBuffer_p: vkCreateBuffer_t,
  vkDestroyBuffer_p: vkDestroyBuffer_t,
  vkCreateBufferView_p: vkCreateBufferView_t,
  vkDestroyBufferView_p: vkDestroyBufferView_t,
  vkCreateImage_p: vkCreateImage_t,
  vkDestroyImage_p: vkDestroyImage_t,
  vkGetImageSubresourceLayout_p: vkGetImageSubresourceLayout_t,
  vkCreateImageView_p: vkCreateImageView_t,
  vkDestroyImageView_p: vkDestroyImageView_t,
  vkCreateShaderModule_p: vkCreateShaderModule_t,
  vkDestroyShaderModule_p: vkDestroyShaderModule_t,
  vkCreatePipelineCache_p: vkCreatePipelineCache_t,
  vkDestroyPipelineCache_p: vkDestroyPipelineCache_t,
  vkGetPipelineCacheData_p: vkGetPipelineCacheData_t,
  vkMergePipelineCaches_p: vkMergePipelineCaches_t,
  vkCreateGraphicsPipelines_p: vkCreateGraphicsPipelines_t,
  vkCreateComputePipelines_p: vkCreateComputePipelines_t,
  vkDestroyPipeline_p: vkDestroyPipeline_t,
  vkCreatePipelineLayout_p: vkCreatePipelineLayout_t,
  vkDestroyPipelineLayout_p: vkDestroyPipelineLayout_t,
  vkCreateSampler_p: vkCreateSampler_t,
  vkDestroySampler_p: vkDestroySampler_t,
  vkCreateDescriptorSetLayout_p: vkCreateDescriptorSetLayout_t,
  vkDestroyDescriptorSetLayout_p: vkDestroyDescriptorSetLayout_t,
  vkCreateDescriptorPool_p: vkCreateDescriptorPool_t,
  vkDestroyDescriptorPool_p: vkDestroyDescriptorPool_t,
  vkResetDescriptorPool_p: vkResetDescriptorPool_t,
  vkAllocateDescriptorSets_p: vkAllocateDescriptorSets_t,
  vkFreeDescriptorSets_p: vkFreeDescriptorSets_t,
  vkUpdateDescriptorSets_p: vkUpdateDescriptorSets_t,
  vkCreateFramebuffer_p: vkCreateFramebuffer_t,
  vkDestroyFramebuffer_p: vkDestroyFramebuffer_t,
  vkCreateRenderPass_p: vkCreateRenderPass_t,
  vkDestroyRenderPass_p: vkDestroyRenderPass_t,
  vkGetRenderAreaGranularity_p: vkGetRenderAreaGranularity_t,
  vkCreateCommandPool_p: vkCreateCommandPool_t,
  vkDestroyCommandPool_p: vkDestroyCommandPool_t,
  vkResetCommandPool_p: vkResetCommandPool_t,
  vkAllocateCommandBuffers_p: vkAllocateCommandBuffers_t,
  vkFreeCommandBuffers_p: vkFreeCommandBuffers_t,
  vkBeginCommandBuffer_p: vkBeginCommandBuffer_t,
  vkEndCommandBuffer_p: vkEndCommandBuffer_t,
  vkResetCommandBuffer_p: vkResetCommandBuffer_t,
  vkCmdBindPipeline_p: vkCmdBindPipeline_t,
  vkCmdSetViewport_p: vkCmdSetViewport_t,
  vkCmdSetScissor_p: vkCmdSetScissor_t,
  vkCmdSetLineWidth_p: vkCmdSetLineWidth_t,
  vkCmdSetDepthBias_p: vkCmdSetDepthBias_t,
  vkCmdSetBlendConstants_p: vkCmdSetBlendConstants_t,
  vkCmdSetDepthBounds_p: vkCmdSetDepthBounds_t,
  vkCmdSetStencilCompareMask_p: vkCmdSetStencilCompareMask_t,
  vkCmdSetStencilWriteMask_p: vkCmdSetStencilWriteMask_t,
  vkCmdSetStencilReference_p: vkCmdSetStencilReference_t,
  vkCmdBindDescriptorSets_p: vkCmdBindDescriptorSets_t,
  vkCmdBindIndexBuffer_p: vkCmdBindIndexBuffer_t,
  vkCmdBindVertexBuffers_p: vkCmdBindVertexBuffers_t,
  vkCmdDraw_p: vkCmdDraw_t,
  vkCmdDrawIndexed_p: vkCmdDrawIndexed_t,
  vkCmdDrawIndirect_p: vkCmdDrawIndirect_t,
  vkCmdDrawIndexedIndirect_p: vkCmdDrawIndexedIndirect_t,
  vkCmdDispatch_p: vkCmdDispatch_t,
  vkCmdDispatchIndirect_p: vkCmdDispatchIndirect_t,
  vkCmdCopyBuffer_p: vkCmdCopyBuffer_t,
  vkCmdCopyImage_p: vkCmdCopyImage_t,
  vkCmdBlitImage_p: vkCmdBlitImage_t,
  vkCmdCopyBufferToImage_p: vkCmdCopyBufferToImage_t,
  vkCmdCopyImageToBuffer_p: vkCmdCopyImageToBuffer_t,
  vkCmdUpdateBuffer_p: vkCmdUpdateBuffer_t,
  vkCmdFillBuffer_p: vkCmdFillBuffer_t,
  vkCmdClearColorImage_p: vkCmdClearColorImage_t,
  vkCmdClearDepthStencilImage_p: vkCmdClearDepthStencilImage_t,
  vkCmdClearAttachments_p: vkCmdClearAttachments_t,
  vkCmdResolveImage_p: vkCmdResolveImage_t,
  vkCmdSetEvent_p: vkCmdSetEvent_t,
  vkCmdResetEvent_p: vkCmdResetEvent_t,
  vkCmdWaitEvents_p: vkCmdWaitEvents_t,
  vkCmdPipelineBarrier_p: vkCmdPipelineBarrier_t,
  vkCmdBeginQuery_p: vkCmdBeginQuery_t,
  vkCmdEndQuery_p: vkCmdEndQuery_t,
  vkCmdResetQueryPool_p: vkCmdResetQueryPool_t,
  vkCmdWriteTimestamp_p: vkCmdWriteTimestamp_t,
  vkCmdCopyQueryPoolResults_p: vkCmdCopyQueryPoolResults_t,
  vkCmdPushConstants_p: vkCmdPushConstants_t,
  vkCmdBeginRenderPass_p: vkCmdBeginRenderPass_t,
  vkCmdNextSubpass_p: vkCmdNextSubpass_t,
  vkCmdEndRenderPass_p: vkCmdEndRenderPass_t,
  vkCmdExecuteCommands_p: vkCmdExecuteCommands_t,
}

impl DeviceFns {
  pub unsafe fn load_from(in_fns: InstanceFns, device: VkDevice) -> Result<Self, &'static str> {
    let vkGetDeviceQueue_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkGetDeviceQueue\0".as_ptr()).ok_or("vkGetDeviceQueue")?);
    let vkAllocateMemory_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkAllocateMemory\0".as_ptr()).ok_or("vkAllocateMemory")?);
    let vkFreeMemory_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkFreeMemory\0".as_ptr()).ok_or("vkFreeMemory")?);
    let vkMapMemory_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkMapMemory\0".as_ptr()).ok_or("vkMapMemory")?);
    let vkUnmapMemory_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkUnmapMemory\0".as_ptr()).ok_or("vkUnmapMemory")?);
    let vkFlushMappedMemoryRanges_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkFlushMappedMemoryRanges\0".as_ptr()).ok_or("vkFlushMappedMemoryRanges")?);
    let vkInvalidateMappedMemoryRanges_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkInvalidateMappedMemoryRanges\0".as_ptr()).ok_or("vkInvalidateMappedMemoryRanges")?);
    let vkGetDeviceMemoryCommitment_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkGetDeviceMemoryCommitment\0".as_ptr()).ok_or("vkGetDeviceMemoryCommitment")?);
    let vkBindBufferMemory_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkBindBufferMemory\0".as_ptr()).ok_or("vkBindBufferMemory")?);
    let vkBindImageMemory_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkBindImageMemory\0".as_ptr()).ok_or("vkBindImageMemory")?);
    let vkGetBufferMemoryRequirements_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkGetBufferMemoryRequirements\0".as_ptr()).ok_or("vkGetBufferMemoryRequirements")?);
    let vkGetImageMemoryRequirements_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkGetImageMemoryRequirements\0".as_ptr()).ok_or("vkGetImageMemoryRequirements")?);
    let vkGetImageSparseMemoryRequirements_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkGetImageSparseMemoryRequirements\0".as_ptr()).ok_or("vkGetImageSparseMemoryRequirements")?);
    let vkCreateFence_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCreateFence\0".as_ptr()).ok_or("vkCreateFence")?);
    let vkDestroyFence_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkDestroyFence\0".as_ptr()).ok_or("vkDestroyFence")?);
    let vkResetFences_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkResetFences\0".as_ptr()).ok_or("vkResetFences")?);
    let vkGetFenceStatus_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkGetFenceStatus\0".as_ptr()).ok_or("vkGetFenceStatus")?);
    let vkWaitForFences_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkWaitForFences\0".as_ptr()).ok_or("vkWaitForFences")?);
    let vkCreateSemaphore_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCreateSemaphore\0".as_ptr()).ok_or("vkCreateSemaphore")?);
    let vkDestroySemaphore_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkDestroySemaphore\0".as_ptr()).ok_or("vkDestroySemaphore")?);
    let vkCreateEvent_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCreateEvent\0".as_ptr()).ok_or("vkCreateEvent")?);
    let vkDestroyEvent_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkDestroyEvent\0".as_ptr()).ok_or("vkDestroyEvent")?);
    let vkGetEventStatus_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkGetEventStatus\0".as_ptr()).ok_or("vkGetEventStatus")?);
    let vkSetEvent_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkSetEvent\0".as_ptr()).ok_or("vkSetEvent")?);
    let vkResetEvent_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkResetEvent\0".as_ptr()).ok_or("vkResetEvent")?);
    let vkCreateQueryPool_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCreateQueryPool\0".as_ptr()).ok_or("vkCreateQueryPool")?);
    let vkDestroyQueryPool_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkDestroyQueryPool\0".as_ptr()).ok_or("vkDestroyQueryPool")?);
    let vkGetQueryPoolResults_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkGetQueryPoolResults\0".as_ptr()).ok_or("vkGetQueryPoolResults")?);
    let vkCreateBuffer_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCreateBuffer\0".as_ptr()).ok_or("vkCreateBuffer")?);
    let vkDestroyBuffer_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkDestroyBuffer\0".as_ptr()).ok_or("vkDestroyBuffer")?);
    let vkCreateBufferView_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCreateBufferView\0".as_ptr()).ok_or("vkCreateBufferView")?);
    let vkDestroyBufferView_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkDestroyBufferView\0".as_ptr()).ok_or("vkDestroyBufferView")?);
    let vkCreateImage_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCreateImage\0".as_ptr()).ok_or("vkCreateImage")?);
    let vkDestroyImage_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkDestroyImage\0".as_ptr()).ok_or("vkDestroyImage")?);
    let vkGetImageSubresourceLayout_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkGetImageSubresourceLayout\0".as_ptr()).ok_or("vkGetImageSubresourceLayout")?);
    let vkCreateImageView_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCreateImageView\0".as_ptr()).ok_or("vkCreateImageView")?);
    let vkDestroyImageView_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkDestroyImageView\0".as_ptr()).ok_or("vkDestroyImageView")?);
    let vkCreateShaderModule_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCreateShaderModule\0".as_ptr()).ok_or("vkCreateShaderModule")?);
    let vkDestroyShaderModule_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkDestroyShaderModule\0".as_ptr()).ok_or("vkDestroyShaderModule")?);
    let vkCreatePipelineCache_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCreatePipelineCache\0".as_ptr()).ok_or("vkCreatePipelineCache")?);
    let vkDestroyPipelineCache_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkDestroyPipelineCache\0".as_ptr()).ok_or("vkDestroyPipelineCache")?);
    let vkGetPipelineCacheData_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkGetPipelineCacheData\0".as_ptr()).ok_or("vkGetPipelineCacheData")?);
    let vkMergePipelineCaches_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkMergePipelineCaches\0".as_ptr()).ok_or("vkMergePipelineCaches")?);
    let vkCreateGraphicsPipelines_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCreateGraphicsPipelines\0".as_ptr()).ok_or("vkCreateGraphicsPipelines")?);
    let vkCreateComputePipelines_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCreateComputePipelines\0".as_ptr()).ok_or("vkCreateComputePipelines")?);
    let vkDestroyPipeline_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkDestroyPipeline\0".as_ptr()).ok_or("vkDestroyPipeline")?);
    let vkCreatePipelineLayout_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCreatePipelineLayout\0".as_ptr()).ok_or("vkCreatePipelineLayout")?);
    let vkDestroyPipelineLayout_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkDestroyPipelineLayout\0".as_ptr()).ok_or("vkDestroyPipelineLayout")?);
    let vkCreateSampler_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCreatesampler\0".as_ptr()).ok_or("vkCreateSampler")?);
    let vkDestroySampler_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkDestroysampler\0".as_ptr()).ok_or("vkDestroySampler")?);
    let vkCreateDescriptorSetLayout_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCreateDescriptorSetLayout\0".as_ptr()).ok_or("vkCreateDescriptorSetLayout")?);
    let vkDestroyDescriptorSetLayout_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkDestroyDescriptorSetLayout\0".as_ptr()).ok_or("vkDestroyDescriptorSetLayout")?);
    let vkCreateDescriptorPool_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCreateDescriptorPool\0".as_ptr()).ok_or("vkCreateDescriptorPool")?);
    let vkDestroyDescriptorPool_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkDestroyDescriptorPool\0".as_ptr()).ok_or("vkDestroyDescriptorPool")?);
    let vkResetDescriptorPool_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkResetDescriptorPool\0".as_ptr()).ok_or("vkResetDescriptorPool")?);
    let vkAllocateDescriptorSets_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkAllocateDescriptorSets\0".as_ptr()).ok_or("vkAllocateDescriptorSets")?);
    let vkFreeDescriptorSets_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkFreeDescriptorSets\0".as_ptr()).ok_or("vkFreeDescriptorSets")?);
    let vkUpdateDescriptorSets_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkUpdateDescriptorSets\0".as_ptr()).ok_or("vkUpdateDescriptorSets")?);
    let vkCreateFramebuffer_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCreateFramebuffer\0".as_ptr()).ok_or("vkCreateFramebuffer")?);
    let vkDestroyFramebuffer_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkDestroyFramebuffer\0".as_ptr()).ok_or("vkDestroyFramebuffer")?);
    let vkCreateRenderPass_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCreateRenderPass\0".as_ptr()).ok_or("vkCreateRenderPass")?);
    let vkDestroyRenderPass_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkDestroyRenderPass\0".as_ptr()).ok_or("vkDestroyRenderPass")?);
    let vkGetRenderAreaGranularity_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkGetRenderAreaGranularity\0".as_ptr()).ok_or("vkGetRenderAreaGranularity")?);
    let vkCreateCommandPool_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCreateCommandPool\0".as_ptr()).ok_or("vkCreateCommandPool")?);
    let vkDestroyCommandPool_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkDestroyCommandPool\0".as_ptr()).ok_or("vkDestroyCommandPool")?);
    let vkResetCommandPool_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkResetCommandPool\0".as_ptr()).ok_or("vkResetCommandPool")?);
    let vkAllocateCommandBuffers_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkAllocateCommandBuffers\0".as_ptr()).ok_or("vkAllocateCommandBuffers")?);
    let vkFreeCommandBuffers_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkFreeCommandBuffers\0".as_ptr()).ok_or("vkFreeCommandBuffers")?);
    let vkBeginCommandBuffer_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkBeginCommandBuffer\0".as_ptr()).ok_or("vkBeginCommandBuffer")?);
    let vkEndCommandBuffer_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkEndCommandBuffer\0".as_ptr()).ok_or("vkEndCommandBuffer")?);
    let vkResetCommandBuffer_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkResetCommandBuffer\0".as_ptr()).ok_or("vkResetCommandBuffer")?);
    let vkCmdBindPipeline_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdBindPipeline\0".as_ptr()).ok_or("vkCmdBindPipeline")?);
    let vkCmdSetViewport_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdSetViewport\0".as_ptr()).ok_or("vkCmdSetViewport")?);
    let vkCmdSetScissor_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdSetScissor\0".as_ptr()).ok_or("vkCmdSetScissor")?);
    let vkCmdSetLineWidth_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdSetLineWidth\0".as_ptr()).ok_or("vkCmdSetLineWidth")?);
    let vkCmdSetDepthBias_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdSetDepthBias\0".as_ptr()).ok_or("vkCmdSetDepthBias")?);
    let vkCmdSetBlendConstants_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdSetBlendConstants\0".as_ptr()).ok_or("vkCmdSetBlendConstants")?);
    let vkCmdSetDepthBounds_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdSetDepthBounds\0".as_ptr()).ok_or("vkCmdSetDepthBounds")?);
    let vkCmdSetStencilCompareMask_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdSetStencilCompareMask\0".as_ptr()).ok_or("vkCmdSetStencilCompareMask")?);
    let vkCmdSetStencilWriteMask_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdSetStencilWriteMask\0".as_ptr()).ok_or("vkCmdSetStencilWriteMask")?);
    let vkCmdSetStencilReference_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdSetStencilReference\0".as_ptr()).ok_or("vkCmdSetStencilReference")?);
    let vkCmdBindDescriptorSets_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdBindDescriptorSets\0".as_ptr()).ok_or("vkCmdBindDescriptorSets")?);
    let vkCmdBindIndexBuffer_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdBindIndexBuffer\0".as_ptr()).ok_or("vkCmdBindIndexBuffer")?);
    let vkCmdBindVertexBuffers_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdBindVertexBuffers\0".as_ptr()).ok_or("vkCmdBindVertexBuffers")?);
    let vkCmdDraw_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdDraw\0".as_ptr()).ok_or("vkCmdDraw")?);
    let vkCmdDrawIndexed_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdDrawIndexed\0".as_ptr()).ok_or("vkCmdDrawIndexed")?);
    let vkCmdDrawIndirect_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdDrawIndirect\0".as_ptr()).ok_or("vkCmdDrawIndirect")?);
    let vkCmdDrawIndexedIndirect_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdDrawIndexedIndirect\0".as_ptr()).ok_or("vkCmdDrawIndexedIndirect")?);
    let vkCmdDispatch_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdDispatch\0".as_ptr()).ok_or("vkCmdDispatch")?);
    let vkCmdDispatchIndirect_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdDispatchIndirect\0".as_ptr()).ok_or("vkCmdDispatchIndirect")?);
    let vkCmdCopyBuffer_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdCopyBuffer\0".as_ptr()).ok_or("vkCmdCopyBuffer")?);
    let vkCmdCopyImage_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdCopyImage\0".as_ptr()).ok_or("vkCmdCopyImage")?);
    let vkCmdBlitImage_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdBlitImage\0".as_ptr()).ok_or("vkCmdBlitImage")?);
    let vkCmdCopyBufferToImage_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdCopyBufferToImage\0".as_ptr()).ok_or("vkCmdCopyBufferToImage")?);
    let vkCmdCopyImageToBuffer_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdCopyImageToBuffer\0".as_ptr()).ok_or("vkCmdCopyImageToBuffer")?);
    let vkCmdUpdateBuffer_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdUpdateBuffer\0".as_ptr()).ok_or("vkCmdUpdateBuffer")?);
    let vkCmdFillBuffer_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdFillBuffer\0".as_ptr()).ok_or("vkCmdFillBuffer")?);
    let vkCmdClearColorImage_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdClearColorImage\0".as_ptr()).ok_or("vkCmdClearColorImage")?);
    let vkCmdClearDepthStencilImage_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdClearDepthStencilImage\0".as_ptr()).ok_or("vkCmdClearDepthStencilImage")?);
    let vkCmdClearAttachments_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdClearAttachments\0".as_ptr()).ok_or("vkCmdClearAttachments")?);
    let vkCmdResolveImage_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdResolveImage\0".as_ptr()).ok_or("vkCmdResolveImage")?);
    let vkCmdSetEvent_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdSetEvent\0".as_ptr()).ok_or("vkCmdSetEvent")?);
    let vkCmdResetEvent_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdResetEvent\0".as_ptr()).ok_or("vkCmdResetEvent")?);
    let vkCmdWaitEvents_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdWaitEvents\0".as_ptr()).ok_or("vkCmdWaitEvents")?);
    let vkCmdPipelineBarrier_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdPipelineBarrier\0".as_ptr()).ok_or("vkCmdPipelineBarrier")?);
    let vkCmdBeginQuery_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdBeginQuery\0".as_ptr()).ok_or("vkCmdBeginQuery")?);
    let vkCmdEndQuery_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdEndQuery\0".as_ptr()).ok_or("vkCmdEndQuery")?);
    let vkCmdResetQueryPool_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdResetQueryPool\0".as_ptr()).ok_or("vkCmdResetQueryPool")?);
    let vkCmdWriteTimestamp_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdWriteTimestamp\0".as_ptr()).ok_or("vkCmdWriteTimestamp")?);
    let vkCmdCopyQueryPoolResults_p =
      t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdCopyQueryPoolResults\0".as_ptr()).ok_or("vkCmdCopyQueryPoolResults")?);
    let vkCmdPushConstants_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdPushConstants\0".as_ptr()).ok_or("vkCmdPushConstants")?);
    let vkCmdBeginRenderPass_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdBeginRenderPass\0".as_ptr()).ok_or("vkCmdBeginRenderPass")?);
    let vkCmdNextSubpass_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdNextSubpass\0".as_ptr()).ok_or("vkCmdNextSubpass")?);
    let vkCmdEndRenderPass_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdEndRenderPass\0".as_ptr()).ok_or("vkCmdEndRenderPass")?);
    let vkCmdExecuteCommands_p = t::<NNF, _>(in_fns.GetDeviceProcAddr(device, b"vkCmdExecuteCommands\0".as_ptr()).ok_or("vkCmdExecuteCommands")?);
    Ok(Self {
      vkGetDeviceQueue_p,
      vkAllocateMemory_p,
      vkFreeMemory_p,
      vkMapMemory_p,
      vkUnmapMemory_p,
      vkFlushMappedMemoryRanges_p,
      vkInvalidateMappedMemoryRanges_p,
      vkGetDeviceMemoryCommitment_p,
      vkBindBufferMemory_p,
      vkBindImageMemory_p,
      vkGetBufferMemoryRequirements_p,
      vkGetImageMemoryRequirements_p,
      vkGetImageSparseMemoryRequirements_p,
      vkCreateFence_p,
      vkDestroyFence_p,
      vkResetFences_p,
      vkGetFenceStatus_p,
      vkWaitForFences_p,
      vkCreateSemaphore_p,
      vkDestroySemaphore_p,
      vkCreateEvent_p,
      vkDestroyEvent_p,
      vkGetEventStatus_p,
      vkSetEvent_p,
      vkResetEvent_p,
      vkCreateQueryPool_p,
      vkDestroyQueryPool_p,
      vkGetQueryPoolResults_p,
      vkCreateBuffer_p,
      vkDestroyBuffer_p,
      vkCreateBufferView_p,
      vkDestroyBufferView_p,
      vkCreateImage_p,
      vkDestroyImage_p,
      vkGetImageSubresourceLayout_p,
      vkCreateImageView_p,
      vkDestroyImageView_p,
      vkCreateShaderModule_p,
      vkDestroyShaderModule_p,
      vkCreatePipelineCache_p,
      vkDestroyPipelineCache_p,
      vkGetPipelineCacheData_p,
      vkMergePipelineCaches_p,
      vkCreateGraphicsPipelines_p,
      vkCreateComputePipelines_p,
      vkDestroyPipeline_p,
      vkCreatePipelineLayout_p,
      vkDestroyPipelineLayout_p,
      vkCreateSampler_p,
      vkDestroySampler_p,
      vkCreateDescriptorSetLayout_p,
      vkDestroyDescriptorSetLayout_p,
      vkCreateDescriptorPool_p,
      vkDestroyDescriptorPool_p,
      vkResetDescriptorPool_p,
      vkAllocateDescriptorSets_p,
      vkFreeDescriptorSets_p,
      vkUpdateDescriptorSets_p,
      vkCreateFramebuffer_p,
      vkDestroyFramebuffer_p,
      vkCreateRenderPass_p,
      vkDestroyRenderPass_p,
      vkGetRenderAreaGranularity_p,
      vkCreateCommandPool_p,
      vkDestroyCommandPool_p,
      vkResetCommandPool_p,
      vkAllocateCommandBuffers_p,
      vkFreeCommandBuffers_p,
      vkBeginCommandBuffer_p,
      vkEndCommandBuffer_p,
      vkResetCommandBuffer_p,
      vkCmdBindPipeline_p,
      vkCmdSetViewport_p,
      vkCmdSetScissor_p,
      vkCmdSetLineWidth_p,
      vkCmdSetDepthBias_p,
      vkCmdSetBlendConstants_p,
      vkCmdSetDepthBounds_p,
      vkCmdSetStencilCompareMask_p,
      vkCmdSetStencilWriteMask_p,
      vkCmdSetStencilReference_p,
      vkCmdBindDescriptorSets_p,
      vkCmdBindIndexBuffer_p,
      vkCmdBindVertexBuffers_p,
      vkCmdDraw_p,
      vkCmdDrawIndexed_p,
      vkCmdDrawIndirect_p,
      vkCmdDrawIndexedIndirect_p,
      vkCmdDispatch_p,
      vkCmdDispatchIndirect_p,
      vkCmdCopyBuffer_p,
      vkCmdCopyImage_p,
      vkCmdBlitImage_p,
      vkCmdCopyBufferToImage_p,
      vkCmdCopyImageToBuffer_p,
      vkCmdUpdateBuffer_p,
      vkCmdFillBuffer_p,
      vkCmdClearColorImage_p,
      vkCmdClearDepthStencilImage_p,
      vkCmdClearAttachments_p,
      vkCmdResolveImage_p,
      vkCmdSetEvent_p,
      vkCmdResetEvent_p,
      vkCmdWaitEvents_p,
      vkCmdPipelineBarrier_p,
      vkCmdBeginQuery_p,
      vkCmdEndQuery_p,
      vkCmdResetQueryPool_p,
      vkCmdWriteTimestamp_p,
      vkCmdCopyQueryPoolResults_p,
      vkCmdPushConstants_p,
      vkCmdBeginRenderPass_p,
      vkCmdNextSubpass_p,
      vkCmdEndRenderPass_p,
      vkCmdExecuteCommands_p,
    })
  }

  /// [vkGetDeviceQueue](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceQueue.html)
  pub unsafe fn GetDeviceQueue(&self, device: VkDevice, queueFamilyIndex: uint32_t, queueIndex: uint32_t, pQueue: *mut VkQueue) {
    (self.vkGetDeviceQueue_p)(device, queueFamilyIndex, queueIndex, pQueue)
  }

  /// [vkAllocateMemory](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAllocateMemory.html)
  pub unsafe fn AllocateMemory(
    &self, device: VkDevice, pAllocateInfo: &VkMemoryAllocateInfo, pAllocator: Option<&VkAllocationCallbacks>, pMemory: &mut VkDeviceMemory,
  ) -> VkResult {
    (self.vkAllocateMemory_p)(device, pAllocateInfo, pAllocator, pMemory)
  }

  /// [vkFreeMemory](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkFreeMemory.html)
  pub unsafe fn FreeMemory(&self, device: VkDevice, memory: VkDeviceMemory, pAllocator: Option<&VkAllocationCallbacks>) {
    (self.vkFreeMemory_p)(device, memory, pAllocator)
  }

  /// [vkMapMemory](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkMapMemory.html)
  pub unsafe fn MapMemory(
    &self, device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, ppData: *mut c_void,
  ) -> VkResult {
    (self.vkMapMemory_p)(device, memory, offset, size, flags, ppData)
  }

  /// [vkUnmapMemory](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUnmapMemory.html)
  pub unsafe fn UnmapMemory(&self, device: VkDevice, memory: VkDeviceMemory) {
    (self.vkUnmapMemory_p)(device, memory)
  }

  /// [vkFlushMappedMemoryRanges](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkFlushMappedMemoryRanges.html)
  pub unsafe fn FlushMappedMemoryRanges(&self, device: VkDevice, memoryRangeCount: uint32_t, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult {
    (self.vkFlushMappedMemoryRanges_p)(device, memoryRangeCount, pMemoryRanges)
  }

  /// [vkInvalidateMappedMemoryRanges](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkInvalidateMappedMemoryRanges.html)
  pub unsafe fn InvalidateMappedMemoryRanges(
    &self, device: VkDevice, memoryRangeCount: uint32_t, pMemoryRanges: *const VkMappedMemoryRange,
  ) -> VkResult {
    (self.vkInvalidateMappedMemoryRanges_p)(device, memoryRangeCount, pMemoryRanges)
  }

  /// [vkGetDeviceMemoryCommitment](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceMemoryCommitment.html)
  pub unsafe fn GetDeviceMemoryCommitment(&self, device: VkDevice, memory: VkDeviceMemory, pCommittedMemoryInBytes: &mut VkDeviceSize) {
    (self.vkGetDeviceMemoryCommitment_p)(device, memory, pCommittedMemoryInBytes)
  }

  /// [vkBindBufferMemory](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindBufferMemory.html)
  pub unsafe fn BindBufferMemory(&self, device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult {
    (self.vkBindBufferMemory_p)(device, buffer, memory, memoryOffset)
  }

  /// [vkBindImageMemory](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindImageMemory.html)
  pub unsafe fn BindImageMemory(&self, device: VkDevice, image: VkImage, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult {
    (self.vkBindImageMemory_p)(device, image, memory, memoryOffset)
  }

  /// [vkGetBufferMemoryRequirements](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferMemoryRequirements.html)
  pub unsafe fn GetBufferMemoryRequirements(&self, device: VkDevice, buffer: VkBuffer, pMemoryRequirements: &mut VkMemoryRequirements) {
    (self.vkGetBufferMemoryRequirements_p)(device, buffer, pMemoryRequirements)
  }

  /// [vkGetImageMemoryRequirements](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageMemoryRequirements.html)
  pub unsafe fn GetImageMemoryRequirements(&self, device: VkDevice, image: VkImage, pMemoryRequirements: &mut VkMemoryRequirements) {
    (self.vkGetImageMemoryRequirements_p)(device, image, pMemoryRequirements)
  }

  /// [vkGetImageSparseMemoryRequirements](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageSparseMemoryRequirements.html)
  pub unsafe fn GetImageSparseMemoryRequirements(
    &self, device: VkDevice, image: VkImage, pSparseMemoryRequirementCount: &mut uint32_t,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements,
  ) {
    (self.vkGetImageSparseMemoryRequirements_p)(device, image, pSparseMemoryRequirementCount, pSparseMemoryRequirements)
  }

  /// [vkCreateFence](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateFence.html)
  pub unsafe fn CreateFence(
    &self, device: VkDevice, pCreateInfo: &VkFenceCreateInfo, pAllocator: Option<&VkAllocationCallbacks>, pFence: &mut VkFence,
  ) -> VkResult {
    (self.vkCreateFence_p)(device, pCreateInfo, pAllocator, pFence)
  }

  /// [vkDestroyFence](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyFence.html)
  pub unsafe fn DestroyFence(&self, device: VkDevice, fence: VkFence, pAllocator: Option<&VkAllocationCallbacks>) {
    (self.vkDestroyFence_p)(device, fence, pAllocator)
  }

  /// [vkResetFences](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetFences.html)
  pub unsafe fn ResetFences(&self, device: VkDevice, fenceCount: uint32_t, pFences: *const VkFence) -> VkResult {
    (self.vkResetFences_p)(device, fenceCount, pFences)
  }

  /// [vkGetFenceStatus](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetFenceStatus.html)
  pub unsafe fn GetFenceStatus(&self, device: VkDevice, fence: VkFence) -> VkResult {
    (self.vkGetFenceStatus_p)(device, fence)
  }

  /// [vkWaitForFences](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWaitForFences.html)
  pub unsafe fn WaitForFences(
    &self, device: VkDevice, fenceCount: uint32_t, pFences: *const VkFence, waitAll: VkBool32, timeout: uint64_t,
  ) -> VkResult {
    (self.vkWaitForFences_p)(device, fenceCount, pFences, waitAll, timeout)
  }

  /// [vkCreateSemaphore](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSemaphore.html)
  pub unsafe fn CreateSemaphore(
    &self, device: VkDevice, pCreateInfo: &VkSemaphoreCreateInfo, pAllocator: Option<&VkAllocationCallbacks>, pSemaphore: &mut VkSemaphore,
  ) -> VkResult {
    (self.vkCreateSemaphore_p)(device, pCreateInfo, pAllocator, pSemaphore)
  }

  /// [vkDestroySemaphore](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySemaphore.html)
  pub unsafe fn DestroySemaphore(&self, device: VkDevice, semaphore: VkSemaphore, pAllocator: Option<&VkAllocationCallbacks>) {
    (self.vkDestroySemaphore_p)(device, semaphore, pAllocator)
  }

  /// [vkCreateEvent](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateEvent.html)
  pub unsafe fn CreateEvent(
    &self, device: VkDevice, pCreateInfo: &VkEventCreateInfo, pAllocator: Option<&VkAllocationCallbacks>, pEvent: &mut VkEvent,
  ) -> VkResult {
    (self.vkCreateEvent_p)(device, pCreateInfo, pAllocator, pEvent)
  }

  /// [vkDestroyEvent](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyEvent.html)
  pub unsafe fn DestroyEvent(&self, device: VkDevice, event: VkEvent, pAllocator: Option<&VkAllocationCallbacks>) {
    (self.vkDestroyEvent_p)(device, event, pAllocator)
  }

  /// [vkGetEventStatus](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetEventStatus.html)
  pub unsafe fn GetEventStatus(&self, device: VkDevice, event: VkEvent) -> VkResult {
    (self.vkGetEventStatus_p)(device, event)
  }

  /// [vkSetEvent](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetEvent.html)
  pub unsafe fn SetEvent(&self, device: VkDevice, event: VkEvent) -> VkResult {
    (self.vkSetEvent_p)(device, event)
  }

  /// [vkResetEvent](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetEvent.html)
  pub unsafe fn ResetEvent(&self, device: VkDevice, event: VkEvent) -> VkResult {
    (self.vkResetEvent_p)(device, event)
  }

  /// [vkCreateQueryPool](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateQueryPool.html)
  pub unsafe fn CreateQueryPool(
    &self, device: VkDevice, pCreateInfo: &VkQueryPoolCreateInfo, pAllocator: Option<&VkAllocationCallbacks>, pQueryPool: &mut VkQueryPool,
  ) -> VkResult {
    (self.vkCreateQueryPool_p)(device, pCreateInfo, pAllocator, pQueryPool)
  }

  /// [vkDestroyQueryPool](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyQueryPool.html)
  pub unsafe fn DestroyQueryPool(&self, device: VkDevice, queryPool: VkQueryPool, pAllocator: Option<&VkAllocationCallbacks>) {
    (self.vkDestroyQueryPool_p)(device, queryPool, pAllocator)
  }

  /// [vkGetQueryPoolResults](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetQueryPoolResults.html)
  pub unsafe fn GetQueryPoolResults(
    &self, device: VkDevice, queryPool: VkQueryPool, firstQuery: uint32_t, queryCount: uint32_t, dataSize: size_t, pData: *mut c_void,
    stride: VkDeviceSize, flags: VkQueryResultFlags,
  ) -> VkResult {
    (self.vkGetQueryPoolResults_p)(device, queryPool, firstQuery, queryCount, dataSize, pData, stride, flags)
  }

  /// [vkCreateBuffer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateBuffer.html)
  pub unsafe fn CreateBuffer(
    &self, device: VkDevice, pCreateInfo: &VkBufferCreateInfo, pAllocator: Option<&VkAllocationCallbacks>, pBuffer: &mut VkBuffer,
  ) -> VkResult {
    (self.vkCreateBuffer_p)(device, pCreateInfo, pAllocator, pBuffer)
  }

  /// [vkDestroyBuffer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyBuffer.html)
  pub unsafe fn DestroyBuffer(&self, device: VkDevice, buffer: VkBuffer, pAllocator: Option<&VkAllocationCallbacks>) {
    (self.vkDestroyBuffer_p)(device, buffer, pAllocator)
  }

  /// [vkCreateBufferView](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateBufferView.html)
  pub unsafe fn CreateBufferView(
    &self, device: VkDevice, pCreateInfo: &VkBufferViewCreateInfo, pAllocator: Option<&VkAllocationCallbacks>, pView: &mut VkBufferView,
  ) -> VkResult {
    (self.vkCreateBufferView_p)(device, pCreateInfo, pAllocator, pView)
  }

  /// [vkDestroyBufferView](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyBufferView.html)
  pub unsafe fn DestroyBufferView(&self, device: VkDevice, bufferView: VkBufferView, pAllocator: Option<&VkAllocationCallbacks>) {
    (self.vkDestroyBufferView_p)(device, bufferView, pAllocator)
  }

  /// [vkCreateImage](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateImage.html)
  pub unsafe fn CreateImage(
    &self, device: VkDevice, pCreateInfo: &VkImageCreateInfo, pAllocator: Option<&VkAllocationCallbacks>, pImage: &mut VkImage,
  ) -> VkResult {
    (self.vkCreateImage_p)(device, pCreateInfo, pAllocator, pImage)
  }

  /// [vkDestroyImage](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyImage.html)
  pub unsafe fn DestroyImage(&self, device: VkDevice, image: VkImage, pAllocator: Option<&VkAllocationCallbacks>) {
    (self.vkDestroyImage_p)(device, image, pAllocator)
  }

  /// [vkGetImageSubresourceLayout](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageSubresourceLayout.html)
  pub unsafe fn GetImageSubresourceLayout(
    &self, device: VkDevice, image: VkImage, pSubresource: &VkImageSubresource, pLayout: &mut VkSubresourceLayout,
  ) {
    (self.vkGetImageSubresourceLayout_p)(device, image, pSubresource, pLayout)
  }

  /// [vkCreateImageView](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateImageView.html)
  pub unsafe fn CreateImageView(
    &self, device: VkDevice, pCreateInfo: &VkImageViewCreateInfo, pAllocator: Option<&VkAllocationCallbacks>, pView: &mut VkImageView,
  ) -> VkResult {
    (self.vkCreateImageView_p)(device, pCreateInfo, pAllocator, pView)
  }

  /// [vkDestroyImageView](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyImageView.html)
  pub unsafe fn DestroyImageView(&self, device: VkDevice, imageView: VkImageView, pAllocator: Option<&VkAllocationCallbacks>) {
    (self.vkDestroyImageView_p)(device, imageView, pAllocator)
  }

  /// [vkCreateShaderModule](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateShaderModule.html)
  pub unsafe fn CreateShaderModule(
    &self, device: VkDevice, pCreateInfo: &VkShaderModuleCreateInfo, pAllocator: Option<&VkAllocationCallbacks>, pShaderModule: &mut VkShaderModule,
  ) -> VkResult {
    (self.vkCreateShaderModule_p)(device, pCreateInfo, pAllocator, pShaderModule)
  }

  /// [vkDestroyShaderModule](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyShaderModule.html)
  pub unsafe fn DestroyShaderModule(&self, device: VkDevice, shaderModule: VkShaderModule, pAllocator: Option<&VkAllocationCallbacks>) {
    (self.vkDestroyShaderModule_p)(device, shaderModule, pAllocator)
  }

  /// [vkCreatePipelineCache](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreatePipelineCache.html)
  pub unsafe fn CreatePipelineCache(
    &self, device: VkDevice, pCreateInfo: &VkPipelineCacheCreateInfo, pAllocator: Option<&VkAllocationCallbacks>,
    pPipelineCache: &mut VkPipelineCache,
  ) -> VkResult {
    (self.vkCreatePipelineCache_p)(device, pCreateInfo, pAllocator, pPipelineCache)
  }

  /// [vkDestroyPipelineCache](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyPipelineCache.html)
  pub unsafe fn DestroyPipelineCache(&self, device: VkDevice, pipelineCache: VkPipelineCache, pAllocator: Option<&VkAllocationCallbacks>) {
    (self.vkDestroyPipelineCache_p)(device, pipelineCache, pAllocator)
  }

  /// [vkGetPipelineCacheData](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineCacheData.html)
  pub unsafe fn GetPipelineCacheData(
    &self, device: VkDevice, pipelineCache: VkPipelineCache, pDataSize: &mut size_t, pData: *mut c_void,
  ) -> VkResult {
    (self.vkGetPipelineCacheData_p)(device, pipelineCache, pDataSize, pData)
  }

  /// [vkMergePipelineCaches](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/xkMergePipelineCaches.html)
  pub unsafe fn MergePipelineCaches(
    &self, device: VkDevice, dstCache: VkPipelineCache, srcCacheCount: uint32_t, pSrcCaches: *const VkPipelineCache,
  ) -> VkResult {
    (self.vkMergePipelineCaches_p)(device, dstCache, srcCacheCount, pSrcCaches)
  }

  /// [vkCreateGraphicsPipelines](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateGraphicsPipelines.html)
  pub unsafe fn CreateGraphicsPipelines(
    &self, device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: uint32_t, pCreateInfos: *const VkGraphicsPipelineCreateInfo,
    pAllocator: Option<&VkAllocationCallbacks>, pPipelines: &mut VkPipeline,
  ) -> VkResult {
    (self.vkCreateGraphicsPipelines_p)(device, pipelineCache, createInfoCount, pCreateInfos, pAllocator, pPipelines)
  }

  /// [vkCreateComputePipelines](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateComputePipelines.html)
  pub unsafe fn CreateComputePipelines(
    &self, device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: uint32_t, pCreateInfos: *const VkComputePipelineCreateInfo,
    pAllocator: Option<&VkAllocationCallbacks>, pPipelines: &mut VkPipeline,
  ) -> VkResult {
    (self.vkCreateComputePipelines_p)(device, pipelineCache, createInfoCount, pCreateInfos, pAllocator, pPipelines)
  }

  /// [vkDestroyPipeline](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyPipeline.html)
  pub unsafe fn DestroyPipeline(&self, device: VkDevice, pipeline: VkPipeline, pAllocator: Option<&VkAllocationCallbacks>) {
    (self.vkDestroyPipeline_p)(device, pipeline, pAllocator)
  }

  /// [vkCreatePipelineLayout](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreatePipelineLayout.html)
  pub unsafe fn CreatePipelineLayout(
    &self, device: VkDevice, pCreateInfo: &VkPipelineLayoutCreateInfo, pAllocator: Option<&VkAllocationCallbacks>,
    pPipelineLayout: &mut VkPipelineLayout,
  ) -> VkResult {
    (self.vkCreatePipelineLayout_p)(device, pCreateInfo, pAllocator, pPipelineLayout)
  }

  /// [vkDestroyPipelineLayout](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyPipelineLayout.html)
  pub unsafe fn DestroyPipelineLayout(&self, device: VkDevice, pipelineLayout: VkPipelineLayout, pAllocator: Option<&VkAllocationCallbacks>) {
    (self.vkDestroyPipelineLayout_p)(device, pipelineLayout, pAllocator)
  }

  /// [vkCreateSampler](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSampler.html)
  pub unsafe fn CreateSampler(
    &self, device: VkDevice, pCreateInfo: &VkSamplerCreateInfo, pAllocator: Option<&VkAllocationCallbacks>, pSampler: &mut VkSampler,
  ) -> VkResult {
    (self.vkCreateSampler_p)(device, pCreateInfo, pAllocator, pSampler)
  }

  /// [vkDestroySampler](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySampler.html)
  pub unsafe fn DestroySampler(&self, device: VkDevice, sampler: VkSampler, pAllocator: Option<&VkAllocationCallbacks>) {
    (self.vkDestroySampler_p)(device, sampler, pAllocator)
  }

  /// [vkCreateDescriptorSetLayout](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDescriptorSetLayout.html)
  pub unsafe fn CreateDescriptorSetLayout(
    &self, device: VkDevice, pCreateInfo: &VkDescriptorSetLayoutCreateInfo, pAllocator: Option<&VkAllocationCallbacks>,
    pSetLayout: &mut VkDescriptorSetLayout,
  ) -> VkResult {
    (self.vkCreateDescriptorSetLayout_p)(device, pCreateInfo, pAllocator, pSetLayout)
  }

  /// [vkDestroyDescriptorSetLayout](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDescriptorSetLayout.html)
  pub unsafe fn DestroyDescriptorSetLayout(
    &self, device: VkDevice, descriptorSetLayout: VkDescriptorSetLayout, pAllocator: Option<&VkAllocationCallbacks>,
  ) {
    (self.vkDestroyDescriptorSetLayout_p)(device, descriptorSetLayout, pAllocator)
  }

  /// [vkCreateDescriptorPool](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDescriptorPool.html)
  pub unsafe fn CreateDescriptorPool(
    &self, device: VkDevice, pCreateInfo: &VkDescriptorPoolCreateInfo, pAllocator: Option<&VkAllocationCallbacks>,
    pDescriptorPool: &mut VkDescriptorPool,
  ) -> VkResult {
    (self.vkCreateDescriptorPool_p)(device, pCreateInfo, pAllocator, pDescriptorPool)
  }

  /// [vkDestroyDescriptorPool](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDescriptorPool.html)
  pub unsafe fn DestroyDescriptorPool(&self, device: VkDevice, descriptorPool: VkDescriptorPool, pAllocator: Option<&VkAllocationCallbacks>) {
    (self.vkDestroyDescriptorPool_p)(device, descriptorPool, pAllocator)
  }

  /// [vkResetDescriptorPool](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetDescriptorPool.html)
  pub unsafe fn ResetDescriptorPool(&self, device: VkDevice, descriptorPool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult {
    (self.vkResetDescriptorPool_p)(device, descriptorPool, flags)
  }

  /// [vkAllocateDescriptorSets](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAllocateDescriptorSets.html)
  pub unsafe fn AllocateDescriptorSets(
    &self, device: VkDevice, pAllocateInfo: &VkDescriptorSetAllocateInfo, pDescriptorSets: &mut VkDescriptorSet,
  ) -> VkResult {
    (self.vkAllocateDescriptorSets_p)(device, pAllocateInfo, pDescriptorSets)
  }

  /// [vkFreeDescriptorSets](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkFreeDescriptorSets.html)
  pub unsafe fn FreeDescriptorSets(
    &self, device: VkDevice, descriptorPool: VkDescriptorPool, descriptorSetCount: uint32_t, pDescriptorSets: *const VkDescriptorSet,
  ) -> VkResult {
    (self.vkFreeDescriptorSets_p)(device, descriptorPool, descriptorSetCount, pDescriptorSets)
  }

  /// [vkUpdateDescriptorSets](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUpdateDescriptorSets.html)
  pub unsafe fn UpdateDescriptorSets(
    &self, device: VkDevice, descriptorWriteCount: uint32_t, pDescriptorWrites: *const VkWriteDescriptorSet, descriptorCopyCount: uint32_t,
    pDescriptorCopies: *const VkCopyDescriptorSet,
  ) {
    (self.vkUpdateDescriptorSets_p)(device, descriptorWriteCount, pDescriptorWrites, descriptorCopyCount, pDescriptorCopies)
  }

  /// [vkCreateFramebuffer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateFramebuffer.html)
  pub unsafe fn CreateFramebuffer(
    &self, device: VkDevice, pCreateInfo: &VkFramebufferCreateInfo, pAllocator: Option<&VkAllocationCallbacks>, pFramebuffer: &mut VkFramebuffer,
  ) -> VkResult {
    (self.vkCreateFramebuffer_p)(device, pCreateInfo, pAllocator, pFramebuffer)
  }

  /// [vkDestroyFramebuffer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyFramebuffer.html)
  pub unsafe fn DestroyFramebuffer(&self, device: VkDevice, framebuffer: VkFramebuffer, pAllocator: Option<&VkAllocationCallbacks>) {
    (self.vkDestroyFramebuffer_p)(device, framebuffer, pAllocator)
  }

  /// [vkCreateRenderPass](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRenderPass.html)
  pub unsafe fn CreateRenderPass(
    &self, device: VkDevice, pCreateInfo: &VkRenderPassCreateInfo, pAllocator: Option<&VkAllocationCallbacks>, pRenderPass: &mut VkRenderPass,
  ) -> VkResult {
    (self.vkCreateRenderPass_p)(device, pCreateInfo, pAllocator, pRenderPass)
  }

  /// [vkDestroyRenderPass](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyRenderPass.html)
  pub unsafe fn DestroyRenderPass(&self, device: VkDevice, renderPass: VkRenderPass, pAllocator: Option<&VkAllocationCallbacks>) {
    (self.vkDestroyRenderPass_p)(device, renderPass, pAllocator)
  }

  /// [vkGetRenderAreaGranularity](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRenderAreaGranularity.html)
  pub unsafe fn GetRenderAreaGranularity(&self, device: VkDevice, renderPass: VkRenderPass, pGranularity: &mut VkExtent2D) {
    (self.vkGetRenderAreaGranularity_p)(device, renderPass, pGranularity)
  }

  /// [vkCreateCommandPool](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateCommandPool.html)
  pub unsafe fn CreateCommandPool(
    &self, device: VkDevice, pCreateInfo: &VkCommandPoolCreateInfo, pAllocator: Option<&VkAllocationCallbacks>, pCommandPool: &mut VkCommandPool,
  ) -> VkResult {
    (self.vkCreateCommandPool_p)(device, pCreateInfo, pAllocator, pCommandPool)
  }

  /// [vkDestroyCommandPool](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyCommandPool.html)
  pub unsafe fn DestroyCommandPool(&self, device: VkDevice, commandPool: VkCommandPool, pAllocator: Option<&VkAllocationCallbacks>) {
    (self.vkDestroyCommandPool_p)(device, commandPool, pAllocator)
  }

  /// [vkResetCommandPool](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetCommandPool.html)
  pub unsafe fn ResetCommandPool(&self, device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult {
    (self.vkResetCommandPool_p)(device, commandPool, flags)
  }

  //

  /// [vkAllocateCommandBuffers](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAllocateCommandBuffers.html)
  pub unsafe fn AllocateCommandBuffers(
    &self, device: VkDevice, pAllocateInfo: &VkCommandBufferAllocateInfo, pCommandBuffers: *mut VkCommandBuffer,
  ) -> VkResult {
    (self.vkAllocateCommandBuffers_p)(device, pAllocateInfo, pCommandBuffers)
  }

  /// [vkFreeCommandBuffers](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkFreeCommandBuffers.html)
  pub unsafe fn FreeCommandBuffers(
    &self, device: VkDevice, commandPool: VkCommandPool, commandBufferCount: uint32_t, pCommandBuffers: *const VkCommandBuffer,
  ) {
    (self.vkFreeCommandBuffers_p)(device, commandPool, commandBufferCount, pCommandBuffers)
  }

  /// [vkBeginCommandBuffer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBeginCommandBuffer.html)
  pub unsafe fn BeginCommandBuffer(&self, commandBuffer: VkCommandBuffer, pBeginInfo: &VkCommandBufferBeginInfo) -> VkResult {
    (self.vkBeginCommandBuffer_p)(commandBuffer, pBeginInfo)
  }

  /// [vkEndCommandBuffer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEndCommandBuffer.html)
  pub unsafe fn EndCommandBuffer(&self, commandBuffer: VkCommandBuffer) -> VkResult {
    (self.vkEndCommandBuffer_p)(commandBuffer)
  }

  /// [vkResetCommandBuffer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetCommandBuffer.html)
  pub unsafe fn ResetCommandBuffer(&self, commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult {
    (self.vkResetCommandBuffer_p)(commandBuffer, flags)
  }

  /// [vkCmdBindPipeline](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindPipeline.html)
  pub unsafe fn CmdBindPipeline(&self, commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline) {
    (self.vkCmdBindPipeline_p)(commandBuffer, pipelineBindPoint, pipeline)
  }

  /// [vkCmdSetViewport](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewport.html)
  pub unsafe fn CmdSetViewport(
    &self, commandBuffer: VkCommandBuffer, firstViewport: uint32_t, viewportCount: uint32_t, pViewports: *const VkViewport,
  ) {
    (self.vkCmdSetViewport_p)(commandBuffer, firstViewport, viewportCount, pViewports)
  }

  /// [vkCmdSetScissor](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetScissor.html)
  pub unsafe fn CmdSetScissor(&self, commandBuffer: VkCommandBuffer, firstScissor: uint32_t, scissorCount: uint32_t, pScissors: &VkRect2D) {
    (self.vkCmdSetScissor_p)(commandBuffer, firstScissor, scissorCount, pScissors)
  }

  /// [vkCmdSetLineWidth](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetLineWidth.html)
  pub unsafe fn CmdSetLineWidth(&self, commandBuffer: VkCommandBuffer, lineWidth: float) {
    (self.vkCmdSetLineWidth_p)(commandBuffer, lineWidth)
  }

  /// [vkCmdSetDepthBias](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthBias.html)
  pub unsafe fn CmdSetDepthBias(
    &self, commandBuffer: VkCommandBuffer, depthBiasConstantFactor: float, depthBiasClamp: float, depthBiasSlopeFactor: float,
  ) {
    (self.vkCmdSetDepthBias_p)(commandBuffer, depthBiasConstantFactor, depthBiasClamp, depthBiasSlopeFactor)
  }

  /// [vkCmdSetBlendConstants](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetBlendConstants.html)
  pub unsafe fn CmdSetBlendConstants(&self, commandBuffer: VkCommandBuffer, blendConstants: &[float; 4]) {
    (self.vkCmdSetBlendConstants_p)(commandBuffer, blendConstants)
  }

  /// [vkCmdSetDepthBounds](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthBounds.html)
  pub unsafe fn CmdSetDepthBounds(&self, commandBuffer: VkCommandBuffer, minDepthBounds: float, maxDepthBounds: float) {
    (self.vkCmdSetDepthBounds_p)(commandBuffer, minDepthBounds, maxDepthBounds)
  }

  /// [vkCmdSetStencilCompareMask](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilCompareMask.html)
  pub unsafe fn CmdSetStencilCompareMask(&self, commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: uint32_t) {
    (self.vkCmdSetStencilCompareMask_p)(commandBuffer, faceMask, compareMask)
  }

  /// [vkCmdSetStencilWriteMask](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilWriteMask.html)
  pub unsafe fn CmdSetStencilWriteMask(&self, commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: uint32_t) {
    (self.vkCmdSetStencilWriteMask_p)(commandBuffer, faceMask, writeMask)
  }

  /// [vkCmdSetStencilReference](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilReference.html)
  pub unsafe fn CmdSetStencilReference(&self, commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: uint32_t) {
    (self.vkCmdSetStencilReference_p)(commandBuffer, faceMask, reference)
  }

  /// [vkCmdBindDescriptorSets](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindDescriptorSets.html)
  pub unsafe fn CmdBindDescriptorSets(
    &self, commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, firstSet: uint32_t,
    descriptorSetCount: uint32_t, pDescriptorSets: *const VkDescriptorSet, dynamicOffsetCount: uint32_t, pDynamicOffsets: *const uint32_t,
  ) {
    (self.vkCmdBindDescriptorSets_p)(
      commandBuffer,
      pipelineBindPoint,
      layout,
      firstSet,
      descriptorSetCount,
      pDescriptorSets,
      dynamicOffsetCount,
      pDynamicOffsets,
    )
  }

  /// [vkCmdBindIndexBuffer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindIndexBuffer.html)
  pub unsafe fn CmdBindIndexBuffer(&self, commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, indexType: VkIndexType) {
    (self.vkCmdBindIndexBuffer_p)(commandBuffer, buffer, offset, indexType)
  }

  /// [vkCmdBindVertexBuffers](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindVertexBuffers.html)
  pub unsafe fn CmdBindVertexBuffers(
    &self, commandBuffer: VkCommandBuffer, firstBinding: uint32_t, bindingCount: uint32_t, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize,
  ) {
    (self.vkCmdBindVertexBuffers_p)(commandBuffer, firstBinding, bindingCount, pBuffers, pOffsets)
  }

  /// [vkCmdDraw](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDraw.html)
  pub unsafe fn CmdDraw(
    &self, commandBuffer: VkCommandBuffer, vertexCount: uint32_t, instanceCount: uint32_t, firstVertex: uint32_t, firstInstance: uint32_t,
  ) {
    (self.vkCmdDraw_p)(commandBuffer, vertexCount, instanceCount, firstVertex, firstInstance)
  }

  /// [vkCmdDrawIndexed](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexed.html)
  pub unsafe fn CmdDrawIndexed(
    &self, commandBuffer: VkCommandBuffer, indexCount: uint32_t, instanceCount: uint32_t, firstIndex: uint32_t, vertexOffset: int32_t,
    firstInstance: uint32_t,
  ) {
    (self.vkCmdDrawIndexed_p)(commandBuffer, indexCount, instanceCount, firstIndex, vertexOffset, firstInstance)
  }

  /// [vkCmdDrawIndirect](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirect.html)
  pub unsafe fn CmdDrawIndirect(
    &self, commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: uint32_t, stride: uint32_t,
  ) {
    (self.vkCmdDrawIndirect_p)(commandBuffer, buffer, offset, drawCount, stride)
  }

  /// [vkCmdDrawIndexedIndirect](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexedIndirect.html)
  pub unsafe fn CmdDrawIndexedIndirect(
    &self, commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: uint32_t, stride: uint32_t,
  ) {
    (self.vkCmdDrawIndexedIndirect_p)(commandBuffer, buffer, offset, drawCount, stride)
  }

  /// [vkCmdDispatch](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDispatch.html)
  pub unsafe fn CmdDispatch(&self, commandBuffer: VkCommandBuffer, groupCountX: uint32_t, groupCountY: uint32_t, groupCountZ: uint32_t) {
    (self.vkCmdDispatch_p)(commandBuffer, groupCountX, groupCountY, groupCountZ)
  }

  /// [vkCmdDispatchIndirect](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDispatchIndirect.html)
  pub unsafe fn CmdDispatchIndirect(&self, commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize) {
    (self.vkCmdDispatchIndirect_p)(commandBuffer, buffer, offset)
  }

  /// [vkCmdCopyBuffer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyBuffer.html)
  pub unsafe fn CmdCopyBuffer(
    &self, commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstBuffer: VkBuffer, regionCount: uint32_t, pRegions: *const VkBufferCopy,
  ) {
    (self.vkCmdCopyBuffer_p)(commandBuffer, srcBuffer, dstBuffer, regionCount, pRegions)
  }

  /// [vkCmdCopyImage](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyImage.html)
  pub unsafe fn CmdCopyImage(
    &self, commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout,
    regionCount: uint32_t, pRegions: *const VkImageCopy,
  ) {
    (self.vkCmdCopyImage_p)(commandBuffer, srcImage, srcImageLayout, dstImage, dstImageLayout, regionCount, pRegions)
  }

  /// [vkCmdBlitImage](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBlitImage.html)
  pub unsafe fn CmdBlitImage(
    &self, commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout,
    regionCount: uint32_t, pRegions: *const VkImageBlit, filter: VkFilter,
  ) {
    (self.vkCmdBlitImage_p)(commandBuffer, srcImage, srcImageLayout, dstImage, dstImageLayout, regionCount, pRegions, filter)
  }

  /// [vkCmdCopyBufferToImage](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyBufferToImage.html)
  pub unsafe fn CmdCopyBufferToImage(
    &self, commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: uint32_t,
    pRegions: *const VkBufferImageCopy,
  ) {
    (self.vkCmdCopyBufferToImage_p)(commandBuffer, srcBuffer, dstImage, dstImageLayout, regionCount, pRegions)
  }

  /// [vkCmdCopyImageToBuffer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyImageToBuffer.html)
  pub unsafe fn CmdCopyImageToBuffer(
    &self, commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstBuffer: VkBuffer, regionCount: uint32_t,
    pRegions: *const VkBufferImageCopy,
  ) {
    (self.vkCmdCopyImageToBuffer_p)(commandBuffer, srcImage, srcImageLayout, dstBuffer, regionCount, pRegions)
  }

  /// [vkCmdUpdateBuffer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdUpdateBuffer.html)
  pub unsafe fn CmdUpdateBuffer(
    &self, commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const c_void,
  ) {
    (self.vkCmdUpdateBuffer_p)(commandBuffer, dstBuffer, dstOffset, dataSize, pData)
  }

  /// [vkCmdFillBuffer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdFillBuffer.html)
  pub unsafe fn CmdFillBuffer(
    &self, commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, size: VkDeviceSize, data: uint32_t,
  ) {
    (self.vkCmdFillBuffer_p)(commandBuffer, dstBuffer, dstOffset, size, data)
  }

  /// [vkCmdClearColorImage](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdClearColorImage.html)
  pub unsafe fn CmdClearColorImage(
    &self, commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pColor: *const VkClearColorValue, rangeCount: uint32_t,
    pRanges: *const VkImageSubresourceRange,
  ) {
    (self.vkCmdClearColorImage_p)(commandBuffer, image, imageLayout, pColor, rangeCount, pRanges)
  }

  /// [vkCmdClearDepthStencilImage](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdClearDepthStencilImage.html)
  pub unsafe fn CmdClearDepthStencilImage(
    &self, commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pDepthStencil: *const VkClearDepthStencilValue,
    rangeCount: uint32_t, pRanges: *const VkImageSubresourceRange,
  ) {
    (self.vkCmdClearDepthStencilImage_p)(commandBuffer, image, imageLayout, pDepthStencil, rangeCount, pRanges)
  }

  /// [vkCmdClearAttachments](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdClearAttachments.html)
  pub unsafe fn CmdClearAttachments(
    &self, commandBuffer: VkCommandBuffer, attachmentCount: uint32_t, pAttachments: *const VkClearAttachment, rectCount: uint32_t,
    pRects: *const VkClearRect,
  ) {
    (self.vkCmdClearAttachments_p)(commandBuffer, attachmentCount, pAttachments, rectCount, pRects)
  }

  /// [vkCmdResolveImage](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdResolveImage.html)
  pub unsafe fn CmdResolveImage(
    &self, commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout,
    regionCount: uint32_t, pRegions: *const VkImageResolve,
  ) {
    (self.vkCmdResolveImage_p)(commandBuffer, srcImage, srcImageLayout, dstImage, dstImageLayout, regionCount, pRegions)
  }

  /// [vkCmdSetEvent](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetEvent.html)
  pub unsafe fn CmdSetEvent(&self, commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags) {
    (self.vkCmdSetEvent_p)(commandBuffer, event, stageMask)
  }

  /// [vkCmdResetEvent](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdResetEvent.html)
  pub unsafe fn CmdResetEvent(&self, commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags) {
    (self.vkCmdResetEvent_p)(commandBuffer, event, stageMask)
  }

  /// [vkCmdWaitEvents](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWaitEvents.html)
  pub unsafe fn CmdWaitEvents(
    &self, commandBuffer: VkCommandBuffer, eventCount: uint32_t, pEvents: *const VkEvent, srcStageMask: VkPipelineStageFlags,
    dstStageMask: VkPipelineStageFlags, memoryBarrierCount: uint32_t, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: uint32_t,
    pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: uint32_t, pImageMemoryBarriers: *const VkImageMemoryBarrier,
  ) {
    (self.vkCmdWaitEvents_p)(
      commandBuffer,
      eventCount,
      pEvents,
      srcStageMask,
      dstStageMask,
      memoryBarrierCount,
      pMemoryBarriers,
      bufferMemoryBarrierCount,
      pBufferMemoryBarriers,
      imageMemoryBarrierCount,
      pImageMemoryBarriers,
    )
  }

  /// [vkCmdPipelineBarrier](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPipelineBarrier.html)
  pub unsafe fn CmdPipelineBarrier(
    &self, commandBuffer: VkCommandBuffer, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags,
    dependencyFlags: VkDependencyFlags, memoryBarrierCount: uint32_t, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: uint32_t,
    pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: uint32_t, pImageMemoryBarriers: *const VkImageMemoryBarrier,
  ) {
    (self.vkCmdPipelineBarrier_p)(
      commandBuffer,
      srcStageMask,
      dstStageMask,
      dependencyFlags,
      memoryBarrierCount,
      pMemoryBarriers,
      bufferMemoryBarrierCount,
      pBufferMemoryBarriers,
      imageMemoryBarrierCount,
      pImageMemoryBarriers,
    )
  }

  /// [vkCmdBeginQuery](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginQuery.html)
  pub unsafe fn CmdBeginQuery(&self, commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: uint32_t, flags: VkQueryControlFlags) {
    (self.vkCmdBeginQuery_p)(commandBuffer, queryPool, query, flags)
  }

  /// [vkCmdEndQuery](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndQuery.html)
  pub unsafe fn CmdEndQuery(&self, commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: uint32_t) {
    (self.vkCmdEndQuery_p)(commandBuffer, queryPool, query)
  }

  /// [vkCmdResetQueryPool](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdResetQueryPool.html)
  pub unsafe fn CmdResetQueryPool(&self, commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: uint32_t, queryCount: uint32_t) {
    (self.vkCmdResetQueryPool_p)(commandBuffer, queryPool, firstQuery, queryCount)
  }

  /// [vkCmdWriteTimestamp](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteTimestamp.html)
  pub unsafe fn CmdWriteTimestamp(
    &self, commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlagBits, queryPool: VkQueryPool, query: uint32_t,
  ) {
    (self.vkCmdWriteTimestamp_p)(commandBuffer, pipelineStage, queryPool, query)
  }

  /// [vkCmdCopyQueryPoolResults](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyQueryPoolResults.html)
  pub unsafe fn CmdCopyQueryPoolResults(
    &self, commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: uint32_t, queryCount: uint32_t, dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize, stride: VkDeviceSize, flags: VkQueryResultFlags,
  ) {
    (self.vkCmdCopyQueryPoolResults_p)(commandBuffer, queryPool, firstQuery, queryCount, dstBuffer, dstOffset, stride, flags)
  }

  /// [vkCmdPushConstants](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPushConstants.html)
  pub unsafe fn CmdPushConstants(
    &self, commandBuffer: VkCommandBuffer, layout: VkPipelineLayout, stageFlags: VkShaderStageFlags, offset: uint32_t, size: uint32_t,
    pValues: *const c_void,
  ) {
    (self.vkCmdPushConstants_p)(commandBuffer, layout, stageFlags, offset, size, pValues)
  }

  /// [vkCmdBeginRenderPass](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginRenderPass.html)
  pub unsafe fn CmdBeginRenderPass(&self, commandBuffer: VkCommandBuffer, pRenderPassBegin: &VkRenderPassBeginInfo, contents: VkSubpassContents) {
    (self.vkCmdBeginRenderPass_p)(commandBuffer, pRenderPassBegin, contents)
  }

  /// [vkCmdNextSubpass](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdNextSubpass.html)
  pub unsafe fn CmdNextSubpass(&self, commandBuffer: VkCommandBuffer, contents: VkSubpassContents) {
    (self.vkCmdNextSubpass_p)(commandBuffer, contents)
  }

  /// [vkCmdEndRenderPass](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndRenderPass.html)
  pub unsafe fn CmdEndRenderPass(&self, commandBuffer: VkCommandBuffer) {
    (self.vkCmdEndRenderPass_p)(commandBuffer)
  }

  /// [vkCmdExecuteCommands](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdExecuteCommands.html)
  pub unsafe fn CmdExecuteCommands(&self, commandBuffer: VkCommandBuffer, commandBufferCount: uint32_t, pCommandBuffers: *const VkCommandBuffer) {
    (self.vkCmdExecuteCommands_p)(commandBuffer, commandBufferCount, pCommandBuffers)
  }
}

/// Holds Vulkan extension functions.
#[derive(Clone, Copy)]
pub struct VkExtensionFns {
  //
}

impl VkExtensionFns {
  pub unsafe fn load_from(_pre_instance_fns: PreInstanceFns, _instance: VkInstance) -> Result<Self, &'static str> {
    todo!()
  }
}
//
