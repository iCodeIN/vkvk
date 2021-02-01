use super::*;

macro_rules! structure {
  (
    $(#[$s_meta:meta])*
    $s_name:ident {
      $($(#[$f_meta:meta])* $f_name:ident: $s_ty:ty),*
      $(,)?
    }
  ) => {
    $(#[$s_meta])*
    #[repr(C)]
    pub struct $s_name {
      $($(#[$f_meta])* pub $f_name: $s_ty),*
    }
  };
}

structure! {
  VkDebugUtilsMessengerCallbackDataEXT {
    ///
    /// * Values: [`VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT`]
    sType: VkStructureType,
    ///
    /// * Optional: true
    pNext: *const void,
    ///
    /// * Optional: true
    flags: VkDebugUtilsMessengerCallbackDataFlagsEXT,
    ///
    /// * Optional: true
    /// * len: null-terminated
    pMessageIdName: *const char,
    messageIdNumber: int32_t,
    ///
    /// * len: null-terminated
    pMessage: *const char,
    ///
    /// * Optional: true
    queueLabelCount: uint32_t,
    ///
    /// * len: queueLabelCount
    pQueueLabels: *const VkDebugUtilsLabelEXT,
    ///
    /// * Optional: true
    cmdBufLabelCount: uint32_t,
    ///
    /// * len: cmdBufLabelCount
    pCmdBufLabels: *const VkDebugUtilsLabelEXT,
    ///
    /// * Optional: true
    objectCount: uint32_t,
    ///
    /// * Optional: objectCount
    pObjects: *const VkDebugUtilsObjectNameInfoEXT,
  }
}

structure! {
  /// [VkDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsLabelEXT.html)
  VkDebugUtilsLabelEXT {
    ///
    /// * Value: [`VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT`]
    sType: VkStructureType,
    ///
    /// * Optional: true
    pNext: *const void,
    ///
    /// * len: null-terminated
    pLabelName: *const char,
    color: [float; 4],
  }
}

structure! {
  /// [VkDebugUtilsObjectNameInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsObjectNameInfoEXT.html)
  VkDebugUtilsObjectNameInfoEXT {
    ///
    /// * Value: [`VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT`]
    sType: VkStructureType,
    ///
    /// * Optional: true
    pNext: *const void,
    objectType: VkObjectType,
    objectHandle: uint64_t,
    ///
    /// * Optional: true
    /// * len: null-terminated
    pObjectName: *const char,
  }
}

structure! {
  /// [VkDeviceMemoryReportCallbackDataEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceMemoryReportCallbackDataEXT.html)
  VkDeviceMemoryReportCallbackDataEXT {
    ///
    /// * Value: [`VK_STRUCTURE_TYPE_DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT`]
    sType: VkStructureType,
    ///
    /// * Optional: true
    pNext: *const void,
    flags: VkDeviceMemoryReportFlagsEXT,
    type_: VkDeviceMemoryReportEventTypeEXT,
    memoryObjectId: uint64_t,
    size: VkDeviceSize,
    objectType: VkObjectType,
    objectHandle: uint64_t,
    heapIndex: uint32_t,
  }
}

structure! {
  /// [VkBaseOutStructure](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBaseOutStructure.html)
  VkBaseOutStructure {
    sType: VkStructureType,
    pNext: *mut VkBaseOutStructure,
  }
}

structure! {
  /// [VkBaseInStructure](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBaseInStructure.html)
  VkBaseInStructure {
    sType: VkStructureType,
    pNext: *const VkBaseInStructure,
  }
}

structure! {
  /// [VkOffset2D](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkOffset2D.html)
  VkOffset2D {
    x: int32_t,
    y: int32_t,
  }
}

structure! {
  /// [VkOffset3D](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkOffset3D.html)
  VkOffset3D {
    x: int32_t,
    y: int32_t,
    z: int32_t,
  }
}

structure! {
  /// [VkExtent2D](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExtent2D.html)
  VkExtent2D {
    width: uint32_t,
    height: uint32_t,
  }
}

structure! {
  /// [VkExtent3D](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExtent3D.html)
  VkExtent3D {
    width: uint32_t,
    height: uint32_t,
    depth: uint32_t,
  }
}

structure! {
  /// [VkViewport](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViewport.html)
  VkViewport {
    x: float,
    y: float,
    width: float,
    height: float,
    minDepth: float,
    maxDepth: float,
  }
}

structure! {
  /// [VkRect2D](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRect2D.html)
  VkRect2D {
    offset: VkOffset2D,
    extent: VkExtent2D,
  }
}

structure! {
  /// [VkClearRect](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearRect.html)
  VkClearRect {
    rect: VkRect2D,
    baseArrayLayer: uint32_t,
    layerCount: uint32_t,
  }
}

structure! {
  /// [VkComponentMapping](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkComponentMapping.html)
  VkComponentMapping {
    r: VkComponentSwizzle,
    g: VkComponentSwizzle,
    b: VkComponentSwizzle,
    a: VkComponentSwizzle,
  }
}

structure! {
  /// [VkPhysicalDeviceProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProperties.html)
  VkPhysicalDeviceProperties {
    apiVersion: VulkanVersion,
    driverVersion: uint32_t,
    vendorID: uint32_t,
    deviceID: uint32_t,
    deviceType: VkPhysicalDeviceType,
    deviceName: [char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
    pipelineCacheUUID: [uint8_t; VK_UUID_SIZE],
    limits: VkPhysicalDeviceLimits,
    sparseProperties: VkPhysicalDeviceSparseProperties,
  }
}

structure! {
  /// [VkPhysicalDeviceLimits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceLimits.html)
  VkPhysicalDeviceLimits {
    /// max 1D image dimension
    maxImageDimension1D: uint32_t,
    /// max 2D image dimension
    maxImageDimension2D: uint32_t,
    /// max 3D image dimension
    maxImageDimension3D: uint32_t,
    /// max cubemap image dimension
    maxImageDimensionCube: uint32_t,
    /// max layers for image arrays
    maxImageArrayLayers: uint32_t,
    /// max texel buffer size (fstexels)
    maxTexelBufferElements: uint32_t,
    /// max uniform buffer range (bytes)
    maxUniformBufferRange: uint32_t,
    /// max storage buffer range (bytes)
    maxStorageBufferRange: uint32_t,
    /// max size of the push constants pool (bytes)
    maxPushConstantsSize: uint32_t,
    /// max number of device memory allocations supported
    maxMemoryAllocationCount: uint32_t,
    /// max number of samplers that can be allocated on a device
    maxSamplerAllocationCount: uint32_t,
    /// Granularity (in bytes) at which buffers and images can be bound to adjacent memory for simultaneous usage
    bufferImageGranularity: VkDeviceSize,
    /// Total address space available for sparse allocations (bytes)
    sparseAddressSpaceSize: VkDeviceSize,
    /// max number of descriptors sets that can be bound to a pipeline
    maxBoundDescriptorSets: uint32_t,
    /// max number of samplers allowed per-stage in a descriptor set
    maxPerStageDescriptorSamplers: uint32_t,
    /// max number of uniform buffers allowed per-stage in a descriptor set
    maxPerStageDescriptorUniformBuffers: uint32_t,
    /// max number of storage buffers allowed per-stage in a descriptor set
    maxPerStageDescriptorStorageBuffers: uint32_t,
    /// max number of sampled images allowed per-stage in a descriptor set
    maxPerStageDescriptorSampledImages: uint32_t,
    /// max number of storage images allowed per-stage in a descriptor set
    maxPerStageDescriptorStorageImages: uint32_t,
    /// max number of input attachments allowed per-stage in a descriptor set
    maxPerStageDescriptorInputAttachments: uint32_t,
    /// max number of resources allowed by a single stage
    maxPerStageResources: uint32_t,
    /// max number of samplers allowed in all stages in a descriptor set
    maxDescriptorSetSamplers: uint32_t,
    /// max number of uniform buffers allowed in all stages in a descriptor set
    maxDescriptorSetUniformBuffers: uint32_t,
    /// max number of dynamic uniform buffers allowed in all stages in a descriptor set
    maxDescriptorSetUniformBuffersDynamic: uint32_t,
    /// max number of storage buffers allowed in all stages in a descriptor set
    maxDescriptorSetStorageBuffers: uint32_t,
    /// max number of dynamic storage buffers allowed in all stages in a descriptor set
    maxDescriptorSetStorageBuffersDynamic: uint32_t,
    /// max number of sampled images allowed in all stages in a descriptor set
    maxDescriptorSetSampledImages: uint32_t,
    /// max number of storage images allowed in all stages in a descriptor set
    maxDescriptorSetStorageImages: uint32_t,
    /// max number of input attachments allowed in all stages in a descriptor set
    maxDescriptorSetInputAttachments: uint32_t,
    /// max number of vertex input attribute slots
    maxVertexInputAttributes: uint32_t,
    /// max number of vertex input binding slots
    maxVertexInputBindings: uint32_t,
    /// max vertex input attribute offset added to vertex buffer offset
    maxVertexInputAttributeOffset: uint32_t,
    /// max vertex input binding stride
    maxVertexInputBindingStride: uint32_t,
    /// max number of output components written by vertex shader
    maxVertexOutputComponents: uint32_t,
    /// max level supported by tessellation primitive generator
    maxTessellationGenerationLevel: uint32_t,
    /// max patch size (vertices)
    maxTessellationPatchSize: uint32_t,
    /// max number of input components per-vertex in TCS
    maxTessellationControlPerVertexInputComponents: uint32_t,
    /// max number of output components per-vertex in TCS
    maxTessellationControlPerVertexOutputComponents: uint32_t,
    /// max number of output components per-patch in TCS
    maxTessellationControlPerPatchOutputComponents: uint32_t,
    /// max total number of per-vertex and per-patch output components in TCS
    maxTessellationControlTotalOutputComponents: uint32_t,
    /// max number of input components per vertex in TES
    maxTessellationEvaluationInputComponents: uint32_t,
    /// max number of output components per vertex in TES
    maxTessellationEvaluationOutputComponents: uint32_t,
    /// max invocation count supported in geometry shader
    maxGeometryShaderInvocations: uint32_t,
    /// max number of input components read in geometry stage
    maxGeometryInputComponents: uint32_t,
    /// max number of output components written in geometry stage
    maxGeometryOutputComponents: uint32_t,
    /// max number of vertices that can be emitted in geometry stage
    maxGeometryOutputVertices: uint32_t,
    /// max total number of components (all vertices) written in geometry stage
    maxGeometryTotalOutputComponents: uint32_t,
    /// max number of input components read in fragment stage
    maxFragmentInputComponents: uint32_t,
    /// max number of output attachments written in fragment stage
    maxFragmentOutputAttachments: uint32_t,
    /// max number of output attachments written when using dual source blending
    maxFragmentDualSrcAttachments: uint32_t,
    /// max total number of storage buffers, storage images and output buffers
    maxFragmentCombinedOutputResources: uint32_t,
    /// max total storage size of work group local storage (bytes)
    maxComputeSharedMemorySize: uint32_t,
    /// max num of compute work groups that may be dispatched by a single command (x,y,z)
    maxComputeWorkGroupCount: [uint32_t; 3],
    /// max total compute invocations in a single local work group
    maxComputeWorkGroupInvocations: uint32_t,
    /// max local size of a compute work group (x,y,z)
    maxComputeWorkGroupSize: [uint32_t; 3],
    /// number bits of subpixel precision in screen x and y
    subPixelPrecisionBits: uint32_t,
    /// number bits of precision for selecting texel weights
    subTexelPrecisionBits: uint32_t,
    /// number bits of precision for selecting mipmap weights
    mipmapPrecisionBits: uint32_t,
    /// max index value for indexed draw calls (for 32-bit indices)
    maxDrawIndexedIndexValue: uint32_t,
    /// max draw count for indirect draw calls
    maxDrawIndirectCount: uint32_t,
    /// max absolute sampler LOD bias
    maxSamplerLodBias: float,
    /// max degree of sampler anisotropy
    maxSamplerAnisotropy: float,
    /// max number of active viewports
    maxViewports: uint32_t,
    /// max viewport dimensions (x,y)
    maxViewportDimensions: [uint32_t; 2],
    /// viewport bounds range (min,max)
    viewportBoundsRange: [float; 2],
    /// number bits of subpixel precision for viewport
    viewportSubPixelBits: uint32_t,
    /// min required alignment of pointers returned by MapMemory (bytes)
    minMemoryMapAlignment: size_t,
    /// min required alignment for texel buffer offsets (bytes)
    minTexelBufferOffsetAlignment: VkDeviceSize,
    /// min required alignment for uniform buffer sizes and offsets (bytes)
    minUniformBufferOffsetAlignment: VkDeviceSize,
    /// min required alignment for storage buffer offsets (bytes)
    minStorageBufferOffsetAlignment: VkDeviceSize,
    /// min texel offset for OpTextureSampleOffset
    minTexelOffset: int32_t,
    /// max texel offset for OpTextureSampleOffset
    maxTexelOffset: uint32_t,
    /// min texel offset for OpTextureGatherOffset
    minTexelGatherOffset: int32_t,
    /// max texel offset for OpTextureGatherOffset
    maxTexelGatherOffset: uint32_t,
    /// furthest negative offset for interpolateAtOffset
    minInterpolationOffset: float,
    /// furthest positive offset for interpolateAtOffset
    maxInterpolationOffset: float,
    /// number of subpixel bits for interpolateAtOffset
    subPixelInterpolationOffsetBits: uint32_t,
    /// max width for a framebuffer
    maxFramebufferWidth: uint32_t,
    /// max height for a framebuffer
    maxFramebufferHeight: uint32_t,
    /// max layer count for a layered framebuffer
    maxFramebufferLayers: uint32_t,
    /// supported color sample counts for a framebuffer
    /// * Optional: true
    framebufferColorSampleCounts: VkSampleCountFlags,
    /// supported depth sample counts for a framebuffer
    /// * Optional: true
    framebufferDepthSampleCounts: VkSampleCountFlags,
    /// supported stencil sample counts for a framebuffer
    /// * Optional: true
    framebufferStencilSampleCounts: VkSampleCountFlags,
    /// supported sample counts for a subpass which uses no attachments
    /// * Optional: true
    framebufferNoAttachmentsSampleCounts: VkSampleCountFlags,
    /// max number of color attachments per subpass
    maxColorAttachments: uint32_t,
    /// supported color sample counts for a non-integer sampled image
    /// * Optional: true
    sampledImageColorSampleCounts: VkSampleCountFlags,
    /// supported sample counts for an integer image
    /// * Optional: true
    sampledImageIntegerSampleCounts: VkSampleCountFlags,
    /// supported depth sample counts for a sampled image
    /// * Optional: true
    sampledImageDepthSampleCounts: VkSampleCountFlags,
    /// supported stencil sample counts for a sampled image
    /// * Optional: true
    sampledImageStencilSampleCounts: VkSampleCountFlags,
    /// supported sample counts for a storage image
    /// * Optional: true
    storageImageSampleCounts: VkSampleCountFlags,
    /// max number of sample mask words
    maxSampleMaskWords: uint32_t,
    /// timestamps on graphics and compute queues
    timestampComputeAndGraphics: VkBool32,
    /// number of nanoseconds it takes for timestamp query value to increment by 1
    timestampPeriod: float,
    /// max number of clip distances
    maxClipDistances: uint32_t,
    /// max number of cull distances
    maxCullDistances: uint32_t,
    /// max combined number of user clipping
    maxCombinedClipAndCullDistances: uint32_t,
    /// distinct queue priorities available
    discreteQueuePriorities: uint32_t,
    /// range (min,max) of supported point sizes
    pointSizeRange: [float; 2],
    /// range (min,max) of supported line widths
    lineWidthRange: [float; 2],
    /// granularity of supported point sizes
    pointSizeGranularity: float,
    /// granularity of supported line widths
    lineWidthGranularity: float,
    /// line rasterization follows preferred rules
    strictLines: VkBool32,
    /// supports standard sample locations for all supported sample counts
    standardSampleLocations: VkBool32,
    /// optimal offset of buffer copies
    optimalBufferCopyOffsetAlignment: VkDeviceSize,
    /// optimal pitch of buffer copies
    optimalBufferCopyRowPitchAlignment: VkDeviceSize,
    /// minimum size and alignment for non-coherent host-mapped device memory access
    nonCoherentAtomSize: VkDeviceSize,
  }
}

structure! {
  /// [VkPhysicalDeviceSparseProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSparseProperties.html)
  VkPhysicalDeviceSparseProperties {
    /// Sparse resources support: GPU will access all 2D (single sample) sparse resources using the standard sparse image block shapes (based on pixel format)
    residencyStandard2DBlockShape: VkBool32,
    /// Sparse resources support: GPU will access all 2D (multisample) sparse resources using the standard sparse image block shapes (based on pixel format)
    residencyStandard2DMultisampleBlockShape: VkBool32,
    /// Sparse resources support: GPU will access all 3D sparse resources using the standard sparse image block shapes (based on pixel format)
    residencyStandard3DBlockShape: VkBool32,
    /// Sparse resources support: Images with mip level dimensions that are NOT a multiple of the sparse image block dimensions will be placed in the mip tail
    residencyAlignedMipSize: VkBool32,
    /// Sparse resources support: GPU can consistently access non-resident regions of a resource, all reads return as if data is 0, writes are discarded
    residencyNonResidentStrict: VkBool32,
  }
}

structure! {
  /// [VkExtensionProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExtensionProperties.html)
  VkExtensionProperties {
    /// extension name
    extensionName: [char; VK_MAX_EXTENSION_NAME_SIZE],
    /// version of the extension specification implemented
    specVersion: uint32_t,
  }
}

structure! {
  /// [VkLayerProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkLayerProperties.html)
  VkLayerProperties {
    /// layer name
    layerName: [char; VK_MAX_EXTENSION_NAME_SIZE],
    /// version of the layer specification implemented
    specVersion: uint32_t,
    /// build or release version of the layer's library
    implementationVersion: uint32_t,
    /// Free-form description of the layer
    description: [char; VK_MAX_DESCRIPTION_SIZE],
  }
}

structure! {
  /// [VkApplicationInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkApplicationInfo.html)
  VkApplicationInfo {
    /// * Values: [`VK_STRUCTURE_TYPE_APPLICATION_INFO`]
    sType:VkStructureType,
    /// * Optional: true
    pNext: *const void,
    /// * Optional: true
    /// * Len: null-terminated
    pApplicationName: *const char,
    applicationVersion: uint32_t,
    /// * Optional: true
    /// * Len: null-terminated
    pEngineName: *const char,
    engineVersion: uint32_t,
    apiVersion: VulkanVersion,
  }
}

structure! {
  /// [VkAllocationCallbacks](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAllocationCallbacks.html)
  VkAllocationCallbacks {
    /// * Optional: true
    pUserData: *mut void,
    pfnAllocation: PFN_vkAllocationFunction,
    pfnReallocation: PFN_vkReallocationFunction,
    pfnFree: PFN_vkFreeFunction,
    /// * Optional: true
    pfnInternalAllocation: PFN_vkInternalAllocationNotification,
    /// * Optional: true
    pfnInternalFree: PFN_vkInternalFreeNotification,
  }
}

structure! {
  /// [VkDeviceQueueCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceQueueCreateInfo.html)
  VkDeviceQueueCreateInfo {
    /// * Values: [`VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO`]
    sType:VkStructureType,
    /// * Optional: true
    pNext: *const void,
    /// * Optional: true
    flags: VkDeviceQueueCreateFlags,
    queueFamilyIndex: uint32_t,
    queueCount: uint32_t,
    /// * Len: queueCount
    pQueuePriorities: *const float,
  }
}
