//! Function types.
//!
//! The naming convention here is that a `PFN_` prefix is a "pointer to a
//! function". These are all Option types around some function type.
//!
//! There's also a `_t` suffix, which is the non-nullable "inner" function type.

use super::*;

pub type PFN_vkAllocationFunction = Option<vkAllocationFunction_t>;

pub type PFN_vkFreeFunction = Option<vkFreeFunction_t>;

pub type PFN_vkInternalAllocationNotification = Option<vkInternalAllocationNotification_t>;

pub type PFN_vkInternalFreeNotification = Option<vkInternalFreeNotification_t>;

pub type PFN_vkReallocationFunction = Option<vkReallocationFunction_t>;

pub type PFN_vkVoidFunction = Option<vkVoidFunction_t>;

pub type vkGetInstanceProcAddr_t = unsafe extern "system" fn(instance: VkInstance, pName: *const char) -> PFN_vkVoidFunction;

pub type vkGetDeviceProcAddr_t = unsafe extern "system" fn(device: VkDevice, pName: *const char) -> PFN_vkVoidFunction;

pub(crate) type vkAllocationFunction_t =
  unsafe extern "system" fn(pUserData: *mut void, size: size_t, alignment: size_t, allocationScope: VkSystemAllocationScope) -> *mut void;

pub(crate) type vkFreeFunction_t = unsafe extern "system" fn(pUserData: *mut void, pMemory: *mut void);

pub(crate) type vkInternalAllocationNotification_t =
  unsafe extern "system" fn(pUserData: *mut void, size: size_t, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);

pub(crate) type vkInternalFreeNotification_t =
  unsafe extern "system" fn(pUserData: *mut void, size: size_t, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);

pub(crate) type vkReallocationFunction_t = unsafe extern "system" fn(
  pUserData: *mut void,
  pOriginal: *mut void,
  size: size_t,
  alignment: size_t,
  allocationScope: VkSystemAllocationScope,
) -> *mut void;

pub(crate) type vkVoidFunction_t = unsafe extern "system" fn();

pub(crate) type vkCreateInstance_t =
  unsafe extern "system" fn(pCreateInfo: &VkInstanceCreateInfo, pAllocator: Option<&VkAllocationCallbacks>, pInstance: &mut VkInstance) -> VkResult;

pub(crate) type vkDestroyInstance_t = unsafe extern "system" fn(instance: VkInstance, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkEnumeratePhysicalDevices_t =
  unsafe extern "system" fn(instance: VkInstance, pPhysicalDeviceCount: &mut uint32_t, pPhysicalDevices: *mut VkPhysicalDevice) -> VkResult;

pub(crate) type vkGetPhysicalDeviceFeatures_t = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pFeatures: &mut VkPhysicalDeviceFeatures);

pub(crate) type vkGetPhysicalDeviceFormatProperties_t =
  unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: &mut VkFormatProperties);

pub(crate) type vkGetPhysicalDeviceImageFormatProperties_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  format: VkFormat,
  type_: VkImageType,
  tiling: VkImageTiling,
  usage: VkImageUsageFlags,
  flags: VkImageCreateFlags,
  pImageFormatProperties: &mut VkImageFormatProperties,
) -> VkResult;

pub(crate) type vkGetPhysicalDeviceProperties_t =
  unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pProperties: &mut VkPhysicalDeviceProperties);

pub(crate) type vkGetPhysicalDeviceQueueFamilyProperties_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pQueueFamilyPropertyCount: &mut uint32_t,
  pQueueFamilyProperties: *mut VkQueueFamilyProperties,
);

pub(crate) type vkGetPhysicalDeviceMemoryProperties_t =
  unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pMemoryProperties: &mut VkPhysicalDeviceMemoryProperties);

pub(crate) type vkCreateDevice_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pCreateInfo: &VkDeviceCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
  pDevice: &mut VkDevice,
) -> VkResult;

pub(crate) type vkDestroyDevice_t = unsafe extern "system" fn(device: VkDevice, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkEnumerateInstanceExtensionProperties_t =
  unsafe extern "system" fn(pLayerName: *const char, pPropertyCount: &mut uint32_t, pProperties: *mut VkExtensionProperties) -> VkResult;

pub(crate) type vkEnumerateDeviceExtensionProperties_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pLayerName: *const char,
  pPropertyCount: &mut uint32_t,
  pProperties: *mut VkExtensionProperties,
) -> VkResult;

pub(crate) type vkEnumerateInstanceLayerProperties_t =
  unsafe extern "system" fn(pPropertyCount: &mut uint32_t, pProperties: *mut VkLayerProperties) -> VkResult;

pub(crate) type vkEnumerateDeviceLayerProperties_t =
  unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: &mut uint32_t, pProperties: *mut VkLayerProperties) -> VkResult;

pub(crate) type vkGetDeviceQueue_t =
  unsafe extern "system" fn(device: VkDevice, queueFamilyIndex: uint32_t, queueIndex: uint32_t, pQueue: *mut VkQueue);

pub(crate) type vkQueueSubmit_t =
  unsafe extern "system" fn(queue: VkQueue, submitCount: uint32_t, pSubmits: *const VkSubmitInfo, fence: VkFence) -> VkResult;

pub(crate) type vkQueueWaitIdle_t = unsafe extern "system" fn(queue: VkQueue) -> VkResult;

pub(crate) type vkDeviceWaitIdle_t = unsafe extern "system" fn(device: VkDevice) -> VkResult;

pub(crate) type vkAllocateMemory_t = unsafe extern "system" fn(
  device: VkDevice,
  pAllocateInfo: &VkMemoryAllocateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
  pMemory: &mut VkDeviceMemory,
) -> VkResult;

pub(crate) type vkFreeMemory_t = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkMapMemory_t = unsafe extern "system" fn(
  device: VkDevice,
  memory: VkDeviceMemory,
  offset: VkDeviceSize,
  size: VkDeviceSize,
  flags: VkMemoryMapFlags,
  ppData: *mut c_void,
) -> VkResult;

pub(crate) type vkUnmapMemory_t = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory);

pub(crate) type vkFlushMappedMemoryRanges_t =
  unsafe extern "system" fn(device: VkDevice, memoryRangeCount: uint32_t, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;

pub(crate) type vkInvalidateMappedMemoryRanges_t =
  unsafe extern "system" fn(device: VkDevice, memoryRangeCount: uint32_t, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;

pub(crate) type vkGetDeviceMemoryCommitment_t =
  unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, pCommittedMemoryInBytes: &mut VkDeviceSize);

pub(crate) type vkBindBufferMemory_t =
  unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;

pub(crate) type vkBindImageMemory_t =
  unsafe extern "system" fn(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;

pub(crate) type vkGetBufferMemoryRequirements_t =
  unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, pMemoryRequirements: &mut VkMemoryRequirements);

pub(crate) type vkGetImageMemoryRequirements_t =
  unsafe extern "system" fn(device: VkDevice, image: VkImage, pMemoryRequirements: &mut VkMemoryRequirements);

pub(crate) type vkGetImageSparseMemoryRequirements_t = unsafe extern "system" fn(
  device: VkDevice,
  image: VkImage,
  pSparseMemoryRequirementCount: &mut uint32_t,
  pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements,
);

pub(crate) type vkGetPhysicalDeviceSparseImageFormatProperties_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  format: VkFormat,
  type_: VkImageType,
  samples: VkSampleCountFlagBits,
  usage: VkImageUsageFlags,
  tiling: VkImageTiling,
  pPropertyCount: &mut uint32_t,
  pProperties: *mut VkSparseImageFormatProperties,
);

pub(crate) type vkQueueBindSparse_t =
  unsafe extern "system" fn(queue: VkQueue, bindInfoCount: uint32_t, pBindInfo: *const VkBindSparseInfo, fence: VkFence) -> VkResult;

pub(crate) type vkCreateFence_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: &VkFenceCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
  pFence: &mut VkFence,
) -> VkResult;

pub(crate) type vkDestroyFence_t = unsafe extern "system" fn(device: VkDevice, fence: VkFence, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkResetFences_t = unsafe extern "system" fn(device: VkDevice, fenceCount: uint32_t, pFences: *const VkFence) -> VkResult;

pub(crate) type vkGetFenceStatus_t = unsafe extern "system" fn(device: VkDevice, fence: VkFence) -> VkResult;

pub(crate) type vkWaitForFences_t =
  unsafe extern "system" fn(device: VkDevice, fenceCount: uint32_t, pFences: *const VkFence, waitAll: VkBool32, timeout: uint64_t) -> VkResult;

pub(crate) type vkCreateSemaphore_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: &VkSemaphoreCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
  pSemaphore: &mut VkSemaphore,
) -> VkResult;

pub(crate) type vkDestroySemaphore_t =
  unsafe extern "system" fn(device: VkDevice, semaphore: VkSemaphore, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkCreateEvent_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: &VkEventCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
  pEvent: &mut VkEvent,
) -> VkResult;

pub(crate) type vkDestroyEvent_t = unsafe extern "system" fn(device: VkDevice, event: VkEvent, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkGetEventStatus_t = unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult;

pub(crate) type vkSetEvent_t = unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult;

pub(crate) type vkResetEvent_t = unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult;

pub(crate) type vkCreateQueryPool_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: &VkQueryPoolCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
  pQueryPool: &mut VkQueryPool,
) -> VkResult;

pub(crate) type vkDestroyQueryPool_t =
  unsafe extern "system" fn(device: VkDevice, queryPool: VkQueryPool, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkGetQueryPoolResults_t = unsafe extern "system" fn(
  device: VkDevice,
  queryPool: VkQueryPool,
  firstQuery: uint32_t,
  queryCount: uint32_t,
  dataSize: size_t,
  pData: *mut c_void,
  stride: VkDeviceSize,
  flags: VkQueryResultFlags,
) -> VkResult;

pub(crate) type vkCreateBuffer_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: &VkBufferCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
  pBuffer: &mut VkBuffer,
) -> VkResult;

pub(crate) type vkDestroyBuffer_t = unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkCreateBufferView_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: &VkBufferViewCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
  pView: &mut VkBufferView,
) -> VkResult;

pub(crate) type vkDestroyBufferView_t =
  unsafe extern "system" fn(device: VkDevice, bufferView: VkBufferView, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkCreateImage_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: &VkImageCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
  pImage: &mut VkImage,
) -> VkResult;

pub(crate) type vkDestroyImage_t = unsafe extern "system" fn(device: VkDevice, image: VkImage, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkGetImageSubresourceLayout_t =
  unsafe extern "system" fn(device: VkDevice, image: VkImage, pSubresource: &VkImageSubresource, pLayout: &mut VkSubresourceLayout);

pub(crate) type vkCreateImageView_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: &VkImageViewCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
  pView: &mut VkImageView,
) -> VkResult;

pub(crate) type vkDestroyImageView_t =
  unsafe extern "system" fn(device: VkDevice, imageView: VkImageView, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkCreateShaderModule_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: &VkShaderModuleCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
  pShaderModule: &mut VkShaderModule,
) -> VkResult;

pub(crate) type vkDestroyShaderModule_t =
  unsafe extern "system" fn(device: VkDevice, shaderModule: VkShaderModule, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkCreatePipelineCache_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: &VkPipelineCacheCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
  pPipelineCache: &mut VkPipelineCache,
) -> VkResult;

pub(crate) type vkDestroyPipelineCache_t =
  unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkGetPipelineCacheData_t =
  unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, pDataSize: &mut size_t, pData: *mut c_void) -> VkResult;

pub(crate) type vkMergePipelineCaches_t =
  unsafe extern "system" fn(device: VkDevice, dstCache: VkPipelineCache, srcCacheCount: uint32_t, pSrcCaches: *const VkPipelineCache) -> VkResult;

pub(crate) type vkCreateGraphicsPipelines_t = unsafe extern "system" fn(
  device: VkDevice,
  pipelineCache: VkPipelineCache,
  createInfoCount: uint32_t,
  pCreateInfos: *const VkGraphicsPipelineCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
  pPipelines: &mut VkPipeline,
) -> VkResult;

pub(crate) type vkCreateComputePipelines_t = unsafe extern "system" fn(
  device: VkDevice,
  pipelineCache: VkPipelineCache,
  createInfoCount: uint32_t,
  pCreateInfos: *const VkComputePipelineCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
  pPipelines: &mut VkPipeline,
) -> VkResult;

pub(crate) type vkDestroyPipeline_t = unsafe extern "system" fn(device: VkDevice, pipeline: VkPipeline, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkCreatePipelineLayout_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: &VkPipelineLayoutCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
  pPipelineLayout: &mut VkPipelineLayout,
) -> VkResult;

pub(crate) type vkDestroyPipelineLayout_t =
  unsafe extern "system" fn(device: VkDevice, pipelineLayout: VkPipelineLayout, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkCreateSampler_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: &VkSamplerCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
  pSampler: &mut VkSampler,
) -> VkResult;

pub(crate) type vkDestroySampler_t = unsafe extern "system" fn(device: VkDevice, sampler: VkSampler, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkCreateDescriptorSetLayout_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: &VkDescriptorSetLayoutCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
  pSetLayout: &mut VkDescriptorSetLayout,
) -> VkResult;

pub(crate) type vkDestroyDescriptorSetLayout_t =
  unsafe extern "system" fn(device: VkDevice, descriptorSetLayout: VkDescriptorSetLayout, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkCreateDescriptorPool_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: &VkDescriptorPoolCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
  pDescriptorPool: &mut VkDescriptorPool,
) -> VkResult;

pub(crate) type vkDestroyDescriptorPool_t =
  unsafe extern "system" fn(device: VkDevice, descriptorPool: VkDescriptorPool, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkResetDescriptorPool_t =
  unsafe extern "system" fn(device: VkDevice, descriptorPool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult;

pub(crate) type vkAllocateDescriptorSets_t =
  unsafe extern "system" fn(device: VkDevice, pAllocateInfo: &VkDescriptorSetAllocateInfo, pDescriptorSets: &mut VkDescriptorSet) -> VkResult;

pub(crate) type vkFreeDescriptorSets_t = unsafe extern "system" fn(
  device: VkDevice,
  descriptorPool: VkDescriptorPool,
  descriptorSetCount: uint32_t,
  pDescriptorSets: *const VkDescriptorSet,
) -> VkResult;

pub(crate) type vkUpdateDescriptorSets_t = unsafe extern "system" fn(
  device: VkDevice,
  descriptorWriteCount: uint32_t,
  pDescriptorWrites: *const VkWriteDescriptorSet,
  descriptorCopyCount: uint32_t,
  pDescriptorCopies: *const VkCopyDescriptorSet,
);

pub(crate) type vkCreateFramebuffer_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: &VkFramebufferCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
  pFramebuffer: &mut VkFramebuffer,
) -> VkResult;

pub(crate) type vkDestroyFramebuffer_t =
  unsafe extern "system" fn(device: VkDevice, framebuffer: VkFramebuffer, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkCreateRenderPass_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: &VkRenderPassCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
  pRenderPass: &mut VkRenderPass,
) -> VkResult;

pub(crate) type vkDestroyRenderPass_t =
  unsafe extern "system" fn(device: VkDevice, renderPass: VkRenderPass, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkGetRenderAreaGranularity_t = unsafe extern "system" fn(device: VkDevice, renderPass: VkRenderPass, pGranularity: &mut VkExtent2D);

pub(crate) type vkCreateCommandPool_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: &VkCommandPoolCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
  pCommandPool: &mut VkCommandPool,
) -> VkResult;

pub(crate) type vkDestroyCommandPool_t =
  unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkResetCommandPool_t =
  unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult;

pub(crate) type vkAllocateCommandBuffers_t =
  unsafe extern "system" fn(device: VkDevice, pAllocateInfo: &VkCommandBufferAllocateInfo, pCommandBuffers: *mut VkCommandBuffer) -> VkResult;

pub(crate) type vkFreeCommandBuffers_t =
  unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, commandBufferCount: uint32_t, pCommandBuffers: *const VkCommandBuffer);

pub(crate) type vkBeginCommandBuffer_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pBeginInfo: &VkCommandBufferBeginInfo) -> VkResult;

pub(crate) type vkEndCommandBuffer_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer) -> VkResult;

pub(crate) type vkResetCommandBuffer_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult;

pub(crate) type vkCmdBindPipeline_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline);

pub(crate) type vkCmdSetViewport_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstViewport: uint32_t, viewportCount: uint32_t, pViewports: *const VkViewport);

pub(crate) type vkCmdSetScissor_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstScissor: uint32_t, scissorCount: uint32_t, pScissors: &VkRect2D);

pub(crate) type vkCmdSetLineWidth_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, lineWidth: float);

pub(crate) type vkCmdSetDepthBias_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthBiasConstantFactor: float, depthBiasClamp: float, depthBiasSlopeFactor: float);

pub(crate) type vkCmdSetBlendConstants_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, blendConstants: &[float; 4]);

pub(crate) type vkCmdSetDepthBounds_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, minDepthBounds: float, maxDepthBounds: float);

pub(crate) type vkCmdSetStencilCompareMask_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: uint32_t);

pub(crate) type vkCmdSetStencilWriteMask_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: uint32_t);

pub(crate) type vkCmdSetStencilReference_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: uint32_t);

pub(crate) type vkCmdBindDescriptorSets_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pipelineBindPoint: VkPipelineBindPoint,
  layout: VkPipelineLayout,
  firstSet: uint32_t,
  descriptorSetCount: uint32_t,
  pDescriptorSets: *const VkDescriptorSet,
  dynamicOffsetCount: uint32_t,
  pDynamicOffsets: *const uint32_t,
);

pub(crate) type vkCmdBindIndexBuffer_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, indexType: VkIndexType);

pub(crate) type vkCmdBindVertexBuffers_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  firstBinding: uint32_t,
  bindingCount: uint32_t,
  pBuffers: *const VkBuffer,
  pOffsets: *const VkDeviceSize,
);

pub(crate) type vkCmdDraw_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  vertexCount: uint32_t,
  instanceCount: uint32_t,
  firstVertex: uint32_t,
  firstInstance: uint32_t,
);

pub(crate) type vkCmdDrawIndexed_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  indexCount: uint32_t,
  instanceCount: uint32_t,
  firstIndex: uint32_t,
  vertexOffset: int32_t,
  firstInstance: uint32_t,
);

pub(crate) type vkCmdDrawIndirect_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: uint32_t, stride: uint32_t);

pub(crate) type vkCmdDrawIndexedIndirect_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: uint32_t, stride: uint32_t);

pub(crate) type vkCmdDispatch_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, groupCountX: uint32_t, groupCountY: uint32_t, groupCountZ: uint32_t);

pub(crate) type vkCmdDispatchIndirect_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize);

pub(crate) type vkCmdCopyBuffer_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  srcBuffer: VkBuffer,
  dstBuffer: VkBuffer,
  regionCount: uint32_t,
  pRegions: *const VkBufferCopy,
);

pub(crate) type vkCmdCopyImage_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  srcImage: VkImage,
  srcImageLayout: VkImageLayout,
  dstImage: VkImage,
  dstImageLayout: VkImageLayout,
  regionCount: uint32_t,
  pRegions: *const VkImageCopy,
);

pub(crate) type vkCmdBlitImage_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  srcImage: VkImage,
  srcImageLayout: VkImageLayout,
  dstImage: VkImage,
  dstImageLayout: VkImageLayout,
  regionCount: uint32_t,
  pRegions: *const VkImageBlit,
  filter: VkFilter,
);

pub(crate) type vkCmdCopyBufferToImage_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  srcBuffer: VkBuffer,
  dstImage: VkImage,
  dstImageLayout: VkImageLayout,
  regionCount: uint32_t,
  pRegions: *const VkBufferImageCopy,
);

pub(crate) type vkCmdCopyImageToBuffer_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  srcImage: VkImage,
  srcImageLayout: VkImageLayout,
  dstBuffer: VkBuffer,
  regionCount: uint32_t,
  pRegions: *const VkBufferImageCopy,
);

pub(crate) type vkCmdUpdateBuffer_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  dstBuffer: VkBuffer,
  dstOffset: VkDeviceSize,
  dataSize: VkDeviceSize,
  pData: *const c_void,
);

pub(crate) type vkCmdFillBuffer_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, size: VkDeviceSize, data: uint32_t);

pub(crate) type vkCmdClearColorImage_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  image: VkImage,
  imageLayout: VkImageLayout,
  pColor: *const VkClearColorValue,
  rangeCount: uint32_t,
  pRanges: *const VkImageSubresourceRange,
);

pub(crate) type vkCmdClearDepthStencilImage_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  image: VkImage,
  imageLayout: VkImageLayout,
  pDepthStencil: *const VkClearDepthStencilValue,
  rangeCount: uint32_t,
  pRanges: *const VkImageSubresourceRange,
);

pub(crate) type vkCmdClearAttachments_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  attachmentCount: uint32_t,
  pAttachments: *const VkClearAttachment,
  rectCount: uint32_t,
  pRects: *const VkClearRect,
);

pub(crate) type vkCmdResolveImage_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  srcImage: VkImage,
  srcImageLayout: VkImageLayout,
  dstImage: VkImage,
  dstImageLayout: VkImageLayout,
  regionCount: uint32_t,
  pRegions: *const VkImageResolve,
);

pub(crate) type vkCmdSetEvent_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);

pub(crate) type vkCmdResetEvent_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);

pub(crate) type vkCmdWaitEvents_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  eventCount: uint32_t,
  pEvents: *const VkEvent,
  srcStageMask: VkPipelineStageFlags,
  dstStageMask: VkPipelineStageFlags,
  memoryBarrierCount: uint32_t,
  pMemoryBarriers: *const VkMemoryBarrier,
  bufferMemoryBarrierCount: uint32_t,
  pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
  imageMemoryBarrierCount: uint32_t,
  pImageMemoryBarriers: *const VkImageMemoryBarrier,
);

pub(crate) type vkCmdPipelineBarrier_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  srcStageMask: VkPipelineStageFlags,
  dstStageMask: VkPipelineStageFlags,
  dependencyFlags: VkDependencyFlags,
  memoryBarrierCount: uint32_t,
  pMemoryBarriers: *const VkMemoryBarrier,
  bufferMemoryBarrierCount: uint32_t,
  pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
  imageMemoryBarrierCount: uint32_t,
  pImageMemoryBarriers: *const VkImageMemoryBarrier,
);

pub(crate) type vkCmdBeginQuery_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: uint32_t, flags: VkQueryControlFlags);

pub(crate) type vkCmdEndQuery_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: uint32_t);

pub(crate) type vkCmdResetQueryPool_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: uint32_t, queryCount: uint32_t);

pub(crate) type vkCmdWriteTimestamp_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlagBits, queryPool: VkQueryPool, query: uint32_t);

pub(crate) type vkCmdCopyQueryPoolResults_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  queryPool: VkQueryPool,
  firstQuery: uint32_t,
  queryCount: uint32_t,
  dstBuffer: VkBuffer,
  dstOffset: VkDeviceSize,
  stride: VkDeviceSize,
  flags: VkQueryResultFlags,
);

pub(crate) type vkCmdPushConstants_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  layout: VkPipelineLayout,
  stageFlags: VkShaderStageFlags,
  offset: uint32_t,
  size: uint32_t,
  pValues: *const c_void,
);

pub(crate) type vkCmdBeginRenderPass_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pRenderPassBegin: &VkRenderPassBeginInfo, contents: VkSubpassContents);

pub(crate) type vkCmdNextSubpass_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, contents: VkSubpassContents);

pub(crate) type vkCmdEndRenderPass_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer);

pub(crate) type vkCmdExecuteCommands_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, commandBufferCount: uint32_t, pCommandBuffers: *const VkCommandBuffer);
