use super::*;

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
  /// [VkAabbPositionsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAabbPositionsKHR.html)
  VkAabbPositionsKHR {
    minX: float,
    minY: float,
    minZ: float,
    maxX: float,
    maxY: float,
    maxZ: float,
  }
}

structure! {
  /// [VkAccelerationStructureBuildGeometryInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureBuildGeometryInfoKHR.html)
  VkAccelerationStructureBuildGeometryInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    type_: VkAccelerationStructureTypeKHR,
    /// * **Optional:** true
    flags: VkBuildAccelerationStructureFlagsKHR,
    /// * **No Auto-validity:** true
    mode: VkBuildAccelerationStructureModeKHR,
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    srcAccelerationStructure: VkAccelerationStructureKHR,
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    dstAccelerationStructure: VkAccelerationStructureKHR,
    /// * **Optional:** true
    geometryCount: uint32_t,
    /// * **Optional:** true
    /// * **Len:** geometryCount
    pGeometries: *const VkAccelerationStructureGeometryKHR,
    /// * **Optional:** true,false
    /// * **Len:** geometryCount,1
    ppGeometries: *const VkAccelerationStructureGeometryKHR,
    scratchData: VkDeviceOrHostAddressKHR,
  }
}

structure! {
  /// [VkAccelerationStructureBuildRangeInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureBuildRangeInfoKHR.html)
  VkAccelerationStructureBuildRangeInfoKHR {
    primitiveCount: uint32_t,
    primitiveOffset: uint32_t,
    firstVertex: uint32_t,
    transformOffset: uint32_t,
  }
}

structure! {
  /// [VkAccelerationStructureBuildSizesInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureBuildSizesInfoKHR.html)
  VkAccelerationStructureBuildSizesInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    accelerationStructureSize: VkDeviceSize,
    updateScratchSize: VkDeviceSize,
    buildScratchSize: VkDeviceSize,
  }
}

structure! {
  /// [VkAccelerationStructureCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureCreateInfoKHR.html)
  VkAccelerationStructureCreateInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    createFlags: VkAccelerationStructureCreateFlagsKHR,
    buffer: VkBuffer,
    /// Specified in bytes
    offset: VkDeviceSize,
    size: VkDeviceSize,
    type_: VkAccelerationStructureTypeKHR,
    /// * **Optional:** true
    deviceAddress: VkDeviceAddress,
  }
}

structure! {
  /// [VkAccelerationStructureCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureCreateInfoNV.html)
  VkAccelerationStructureCreateInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    compactedSize: VkDeviceSize,
    info: VkAccelerationStructureInfoNV,
  }
}

structure! {
  /// [VkAccelerationStructureDeviceAddressInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureDeviceAddressInfoKHR.html)
  VkAccelerationStructureDeviceAddressInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    accelerationStructure: VkAccelerationStructureKHR,
  }
}

structure! {
  /// [VkAccelerationStructureGeometryAabbsDataKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryAabbsDataKHR.html)
  VkAccelerationStructureGeometryAabbsDataKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    data: VkDeviceOrHostAddressConstKHR,
    stride: VkDeviceSize,
  }
}

structure! {
  /// [VkAccelerationStructureGeometryInstancesDataKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryInstancesDataKHR.html)
  VkAccelerationStructureGeometryInstancesDataKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    arrayOfPointers: VkBool32,
    data: VkDeviceOrHostAddressConstKHR,
  }
}

structure! {
  /// [VkAccelerationStructureGeometryKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryKHR.html)
  VkAccelerationStructureGeometryKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    geometryType: VkGeometryTypeKHR,
    geometry: VkAccelerationStructureGeometryDataKHR,
    /// * **Optional:** true
    flags: VkGeometryFlagsKHR,
  }
}

structure! {
  /// [VkAccelerationStructureGeometryTrianglesDataKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryTrianglesDataKHR.html)
  VkAccelerationStructureGeometryTrianglesDataKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    vertexFormat: VkFormat,
    vertexData: VkDeviceOrHostAddressConstKHR,
    vertexStride: VkDeviceSize,
    maxVertex: uint32_t,
    indexType: VkIndexType,
    indexData: VkDeviceOrHostAddressConstKHR,
    transformData: VkDeviceOrHostAddressConstKHR,
  }
}

structure! {
  /// [VkAccelerationStructureInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureInfoNV.html)
  VkAccelerationStructureInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    type_: VkAccelerationStructureTypeNV,
    /// * **Optional:** true
    flags: VkBuildAccelerationStructureFlagsNV,
    /// * **Optional:** true
    instanceCount: uint32_t,
    /// * **Optional:** true
    geometryCount: uint32_t,
    /// * **Len:** geometryCount
    pGeometries: *const VkGeometryNV,
  }
}

structure! {
  /// [VkAccelerationStructureInstanceKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureInstanceKHR.html)
  VkAccelerationStructureInstanceKHR {
    transform: VkTransformMatrixKHR,
    instanceCustomIndex: [uint32_t; 24],
    mask: [uint32_t; 8],
    instanceShaderBindingTableRecordOffset: [uint32_t; 24],
    /// * **Optional:** true
    flags: [VkGeometryInstanceFlagsKHR; 8],
    accelerationStructureReference: uint64_t,
  }
}

structure! {
  /// [VkAccelerationStructureMemoryRequirementsInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMemoryRequirementsInfoNV.html)
  VkAccelerationStructureMemoryRequirementsInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    type_: VkAccelerationStructureMemoryRequirementsTypeNV,
    accelerationStructure: VkAccelerationStructureNV,
  }
}

structure! {
  /// [VkAccelerationStructureVersionInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureVersionInfoKHR.html)
  VkAccelerationStructureVersionInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_VERSION_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Len:** 2*ename:VK_UUID_SIZE
    /// * **Alt Len:** 2*VK_UUID_SIZE
    pVersionData: *const uint8_t,
  }
}

structure! {
  /// [VkAcquireNextImageInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAcquireNextImageInfoKHR.html)
  VkAcquireNextImageInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Extern Sync:** true
    swapchain: VkSwapchainKHR,
    timeout: uint64_t,
    /// * **Optional:** true
    /// * **Extern Sync:** true
    semaphore: VkSemaphore,
    /// * **Optional:** true
    /// * **Extern Sync:** true
    fence: VkFence,
    deviceMask: uint32_t,
  }
}

structure! {
  /// [VkAcquireProfilingLockInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAcquireProfilingLockInfoKHR.html)
  VkAcquireProfilingLockInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_ACQUIRE_PROFILING_LOCK_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Acquire profiling lock flags
    /// * **Optional:** true
    flags: VkAcquireProfilingLockFlagsKHR,
    timeout: uint64_t,
  }
}

structure! {
  /// [VkAndroidHardwareBufferFormatPropertiesANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAndroidHardwareBufferFormatPropertiesANDROID.html)
  ///
  /// Struct Extends: [`VkAndroidHardwareBufferPropertiesANDROID`]
  VkAndroidHardwareBufferFormatPropertiesANDROID {
    /// * **Values:** [`VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    format: VkFormat,
    externalFormat: uint64_t,
    formatFeatures: VkFormatFeatureFlags,
    samplerYcbcrConversionComponents: VkComponentMapping,
    suggestedYcbcrModel: VkSamplerYcbcrModelConversion,
    suggestedYcbcrRange: VkSamplerYcbcrRange,
    suggestedXChromaOffset: VkChromaLocation,
    suggestedYChromaOffset: VkChromaLocation,
  }
}

structure! {
  /// [VkAndroidHardwareBufferPropertiesANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAndroidHardwareBufferPropertiesANDROID.html)
  VkAndroidHardwareBufferPropertiesANDROID {
    /// * **Values:** [`VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    allocationSize: VkDeviceSize,
    memoryTypeBits: uint32_t,
  }
}

structure! {
  /// [VkAndroidHardwareBufferUsageANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAndroidHardwareBufferUsageANDROID.html)
  ///
  /// Struct Extends: [`VkImageFormatProperties2`]
  VkAndroidHardwareBufferUsageANDROID {
    /// * **Values:** [`VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_USAGE_ANDROID`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    androidHardwareBufferUsage: uint64_t,
  }
}

structure! {
  /// [VkAndroidSurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAndroidSurfaceCreateInfoKHR.html)
  VkAndroidSurfaceCreateInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkAndroidSurfaceCreateFlagsKHR,
    /// * **No Auto-validity:** true
    window: *mut ANativeWindow,
  }
}

structure! {
  /// [VkAttachmentDescription](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentDescription.html)
  VkAttachmentDescription {
    /// * **Optional:** true
    flags: VkAttachmentDescriptionFlags,
    format: VkFormat,
    samples: VkSampleCountFlagBits,
    /// Load operation for color or depth data
    loadOp: VkAttachmentLoadOp,
    /// Store operation for color or depth data
    storeOp: VkAttachmentStoreOp,
    /// Load operation for stencil data
    stencilLoadOp: VkAttachmentLoadOp,
    /// Store operation for stencil data
    stencilStoreOp: VkAttachmentStoreOp,
    initialLayout: VkImageLayout,
    finalLayout: VkImageLayout,
  }
}

structure! {
  /// [VkAttachmentDescription2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentDescription2.html)
  VkAttachmentDescription2 {
    /// * **Values:** [`VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkAttachmentDescriptionFlags,
    format: VkFormat,
    samples: VkSampleCountFlagBits,
    /// Load operation for color or depth data
    loadOp: VkAttachmentLoadOp,
    /// Store operation for color or depth data
    storeOp: VkAttachmentStoreOp,
    /// Load operation for stencil data
    stencilLoadOp: VkAttachmentLoadOp,
    /// Store operation for stencil data
    stencilStoreOp: VkAttachmentStoreOp,
    initialLayout: VkImageLayout,
    finalLayout: VkImageLayout,
  }
}

structure! {
  /// [VkAttachmentDescriptionStencilLayout](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentDescriptionStencilLayout.html)
  ///
  /// Struct Extends: [`VkAttachmentDescription2`]
  VkAttachmentDescriptionStencilLayout {
    /// * **Values:** [`VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    stencilInitialLayout: VkImageLayout,
    stencilFinalLayout: VkImageLayout,
  }
}

structure! {
  /// [VkAttachmentReference](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentReference.html)
  VkAttachmentReference {
    attachment: uint32_t,
    layout: VkImageLayout,
  }
}

structure! {
  /// [VkAttachmentReference2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentReference2.html)
  VkAttachmentReference2 {
    /// * **Values:** [`VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    attachment: uint32_t,
    layout: VkImageLayout,
    /// * **No Auto-validity:** true
    aspectMask: VkImageAspectFlags,
  }
}

structure! {
  /// [VkAttachmentReferenceStencilLayout](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentReferenceStencilLayout.html)
  ///
  /// Struct Extends: [`VkAttachmentReference2`]
  VkAttachmentReferenceStencilLayout {
    /// * **Values:** [`VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    stencilLayout: VkImageLayout,
  }
}

structure! {
  /// [VkAttachmentSampleLocationsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentSampleLocationsEXT.html)
  VkAttachmentSampleLocationsEXT {
    attachmentIndex: uint32_t,
    sampleLocationsInfo: VkSampleLocationsInfoEXT,
  }
}

structure! {
  /// [VkBindAccelerationStructureMemoryInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindAccelerationStructureMemoryInfoNV.html)
  VkBindAccelerationStructureMemoryInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    accelerationStructure: VkAccelerationStructureNV,
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
    /// * **Optional:** true
    deviceIndexCount: uint32_t,
    /// * **Len:** deviceIndexCount
    pDeviceIndices: *const uint32_t,
  }
}

structure! {
  /// [VkBindBufferMemoryDeviceGroupInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindBufferMemoryDeviceGroupInfo.html)
  ///
  /// Struct Extends: [`VkBindBufferMemoryInfo`]
  VkBindBufferMemoryDeviceGroupInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    deviceIndexCount: uint32_t,
    /// * **Len:** deviceIndexCount
    pDeviceIndices: *const uint32_t,
  }
}

structure! {
  /// [VkBindBufferMemoryInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindBufferMemoryInfo.html)
  VkBindBufferMemoryInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    buffer: VkBuffer,
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
  }
}

structure! {
  /// [VkBindImageMemoryDeviceGroupInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindImageMemoryDeviceGroupInfo.html)
  ///
  /// Struct Extends: [`VkBindImageMemoryInfo`]
  VkBindImageMemoryDeviceGroupInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    deviceIndexCount: uint32_t,
    /// * **Len:** deviceIndexCount
    pDeviceIndices: *const uint32_t,
    /// * **Optional:** true
    splitInstanceBindRegionCount: uint32_t,
    /// * **Len:** splitInstanceBindRegionCount
    pSplitInstanceBindRegions: *const VkRect2D,
  }
}

structure! {
  /// [VkBindImageMemoryInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindImageMemoryInfo.html)
  VkBindImageMemoryInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    image: VkImage,
    /// * **No Auto-validity:** true
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
  }
}

structure! {
  /// [VkBindImageMemorySwapchainInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindImageMemorySwapchainInfoKHR.html)
  ///
  /// Struct Extends: [`VkBindImageMemoryInfo`]
  VkBindImageMemorySwapchainInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Extern Sync:** true
    swapchain: VkSwapchainKHR,
    imageIndex: uint32_t,
  }
}

structure! {
  /// [VkBindImagePlaneMemoryInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindImagePlaneMemoryInfo.html)
  ///
  /// Struct Extends: [`VkBindImageMemoryInfo`]
  VkBindImagePlaneMemoryInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    planeAspect: VkImageAspectFlagBits,
  }
}

structure! {
  /// [VkBindIndexBufferIndirectCommandNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindIndexBufferIndirectCommandNV.html)
  VkBindIndexBufferIndirectCommandNV {
    bufferAddress: VkDeviceAddress,
    size: uint32_t,
    indexType: VkIndexType,
  }
}

structure! {
  /// [VkBindShaderGroupIndirectCommandNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindShaderGroupIndirectCommandNV.html)
  VkBindShaderGroupIndirectCommandNV {
    groupIndex: uint32_t,
  }
}

structure! {
  /// [VkBindSparseInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindSparseInfo.html)
  VkBindSparseInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_BIND_SPARSE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    waitSemaphoreCount: uint32_t,
    /// * **Len:** waitSemaphoreCount
    pWaitSemaphores: *const VkSemaphore,
    /// * **Optional:** true
    bufferBindCount: uint32_t,
    /// * **Len:** bufferBindCount
    pBufferBinds: *const VkSparseBufferMemoryBindInfo,
    /// * **Optional:** true
    imageOpaqueBindCount: uint32_t,
    /// * **Len:** imageOpaqueBindCount
    pImageOpaqueBinds: *const VkSparseImageOpaqueMemoryBindInfo,
    /// * **Optional:** true
    imageBindCount: uint32_t,
    /// * **Len:** imageBindCount
    pImageBinds: *const VkSparseImageMemoryBindInfo,
    /// * **Optional:** true
    signalSemaphoreCount: uint32_t,
    /// * **Len:** signalSemaphoreCount
    pSignalSemaphores: *const VkSemaphore,
  }
}

structure! {
  /// [VkBindVertexBufferIndirectCommandNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindVertexBufferIndirectCommandNV.html)
  VkBindVertexBufferIndirectCommandNV {
    bufferAddress: VkDeviceAddress,
    size: uint32_t,
    stride: uint32_t,
  }
}

structure! {
  /// [VkBlitImageInfo2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBlitImageInfo2KHR.html)
  VkBlitImageInfo2KHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: uint32_t,
    /// * **Len:** regionCount
    pRegions: *const VkImageBlit2KHR,
    filter: VkFilter,
  }
}

structure! {
  /// [VkBufferCopy](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCopy.html)
  VkBufferCopy {
    /// Specified in bytes
    srcOffset: VkDeviceSize,
    /// Specified in bytes
    dstOffset: VkDeviceSize,
    /// Specified in bytes
    /// * **No Auto-validity:** true
    size: VkDeviceSize,
  }
}

structure! {
  /// [VkBufferCopy2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCopy2KHR.html)
  VkBufferCopy2KHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_BUFFER_COPY_2_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Specified in bytes
    srcOffset: VkDeviceSize,
    /// Specified in bytes
    dstOffset: VkDeviceSize,
    /// Specified in bytes
    /// * **No Auto-validity:** true
    size: VkDeviceSize,
  }
}

structure! {
  /// [VkBufferCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCreateInfo.html)
  VkBufferCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Buffer creation flags
    /// * **Optional:** true
    flags: VkBufferCreateFlags,
    /// Specified in bytes
    size: VkDeviceSize,
    /// Buffer usage flags
    usage: VkBufferUsageFlags,
    sharingMode: VkSharingMode,
    /// * **Optional:** true
    queueFamilyIndexCount: uint32_t,
    /// * **No Auto-validity:** true
    /// * **Len:** queueFamilyIndexCount
    pQueueFamilyIndices: *const uint32_t,
  }
}

structure! {
  /// [VkBufferDeviceAddressCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferDeviceAddressCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkBufferCreateInfo`]
  VkBufferDeviceAddressCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    deviceAddress: VkDeviceAddress,
  }
}

structure! {
  /// [VkBufferDeviceAddressInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferDeviceAddressInfo.html)
  VkBufferDeviceAddressInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    buffer: VkBuffer,
  }
}

structure! {
  /// [VkBufferImageCopy](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferImageCopy.html)
  VkBufferImageCopy {
    /// Specified in bytes
    bufferOffset: VkDeviceSize,
    /// Specified in texels
    bufferRowLength: uint32_t,
    bufferImageHeight: uint32_t,
    imageSubresource: VkImageSubresourceLayers,
    /// Specified in pixels for both compressed and uncompressed images
    imageOffset: VkOffset3D,
    /// Specified in pixels for both compressed and uncompressed images
    imageExtent: VkExtent3D,
  }
}

structure! {
  /// [VkBufferImageCopy2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferImageCopy2KHR.html)
  VkBufferImageCopy2KHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Specified in bytes
    bufferOffset: VkDeviceSize,
    /// Specified in texels
    bufferRowLength: uint32_t,
    bufferImageHeight: uint32_t,
    imageSubresource: VkImageSubresourceLayers,
    /// Specified in pixels for both compressed and uncompressed images
    imageOffset: VkOffset3D,
    /// Specified in pixels for both compressed and uncompressed images
    imageExtent: VkExtent3D,
  }
}

structure! {
  /// [VkBufferMemoryRequirementsInfo2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferMemoryRequirementsInfo2.html)
  VkBufferMemoryRequirementsInfo2 {
    /// * **Values:** [`VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    buffer: VkBuffer,
  }
}

structure! {
  /// [VkBufferOpaqueCaptureAddressCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferOpaqueCaptureAddressCreateInfo.html)
  ///
  /// Struct Extends: [`VkBufferCreateInfo`]
  VkBufferOpaqueCaptureAddressCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    opaqueCaptureAddress: uint64_t,
  }
}

structure! {
  /// [VkBufferViewCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferViewCreateInfo.html)
  VkBufferViewCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkBufferViewCreateFlags,
    buffer: VkBuffer,
    /// Optionally specifies format of elements
    format: VkFormat,
    /// Specified in bytes
    offset: VkDeviceSize,
    /// View size specified in bytes
    range: VkDeviceSize,
  }
}

structure! {
  /// [VkCalibratedTimestampInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCalibratedTimestampInfoEXT.html)
  VkCalibratedTimestampInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    timeDomain: VkTimeDomainEXT,
  }
}

structure! {
  /// [VkCheckpointDataNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCheckpointDataNV.html)
  VkCheckpointDataNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_CHECKPOINT_DATA_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    stage: VkPipelineStageFlagBits,
    /// * **No Auto-validity:** true
    pCheckpointMarker: *mut c_void,
  }
}

structure! {
  /// [VkClearAttachment](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearAttachment.html)
  VkClearAttachment {
    aspectMask: VkImageAspectFlags,
    colorAttachment: uint32_t,
    clearValue: VkClearValue,
  }
}

structure! {
  /// [VkClearDepthStencilValue](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearDepthStencilValue.html)
  VkClearDepthStencilValue {
    depth: float,
    stencil: uint32_t,
  }
}

structure! {
  /// [VkCoarseSampleLocationNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCoarseSampleLocationNV.html)
  VkCoarseSampleLocationNV {
    pixelX: uint32_t,
    pixelY: uint32_t,
    sample: uint32_t,
  }
}

structure! {
  /// [VkCoarseSampleOrderCustomNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCoarseSampleOrderCustomNV.html)
  VkCoarseSampleOrderCustomNV {
    shadingRate: VkShadingRatePaletteEntryNV,
    sampleCount: uint32_t,
    sampleLocationCount: uint32_t,
    /// * **Len:** sampleLocationCount
    pSampleLocations: *const VkCoarseSampleLocationNV,
  }
}

structure! {
  /// [VkCommandBufferAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferAllocateInfo.html)
  VkCommandBufferAllocateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    commandPool: VkCommandPool,
    level: VkCommandBufferLevel,
    commandBufferCount: uint32_t,
  }
}

structure! {
  /// [VkCommandBufferBeginInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferBeginInfo.html)
  VkCommandBufferBeginInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Command buffer usage flags
    /// * **Optional:** true
    flags: VkCommandBufferUsageFlags,
    /// Pointer to inheritance info for secondary command buffers
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    pInheritanceInfo: *const VkCommandBufferInheritanceInfo,
  }
}

structure! {
  /// [VkCommandBufferInheritanceConditionalRenderingInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferInheritanceConditionalRenderingInfoEXT.html)
  ///
  /// Struct Extends: [`VkCommandBufferInheritanceInfo`]
  VkCommandBufferInheritanceConditionalRenderingInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Whether this secondary command buffer may be executed during an active conditional rendering
    conditionalRenderingEnable: VkBool32,
  }
}

structure! {
  /// [VkCommandBufferInheritanceInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferInheritanceInfo.html)
  VkCommandBufferInheritanceInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Render pass for secondary command buffers
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    renderPass: VkRenderPass,
    subpass: uint32_t,
    /// Framebuffer for secondary command buffers
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    framebuffer: VkFramebuffer,
    /// Whether this secondary command buffer may be executed during an occlusion query
    occlusionQueryEnable: VkBool32,
    /// Query flags used by this secondary command buffer, if executed during an occlusion query
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    queryFlags: VkQueryControlFlags,
    /// Pipeline statistics that may be counted for this secondary command buffer
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    pipelineStatistics: VkQueryPipelineStatisticFlags,
  }
}

structure! {
  /// [VkCommandBufferInheritanceRenderPassTransformInfoQCOM](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferInheritanceRenderPassTransformInfoQCOM.html)
  ///
  /// Struct Extends: [`VkCommandBufferInheritanceInfo`]
  VkCommandBufferInheritanceRenderPassTransformInfoQCOM {
    /// * **Values:** [`VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM`]
    sType: VkStructureType,
    /// Pointer to next structure
    /// * **Optional:** true
    pNext: *mut c_void,
    /// * **No Auto-validity:** true
    transform: VkSurfaceTransformFlagBitsKHR,
    renderArea: VkRect2D,
  }
}

structure! {
  /// [VkCommandPoolCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandPoolCreateInfo.html)
  VkCommandPoolCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Command pool creation flags
    /// * **Optional:** true
    flags: VkCommandPoolCreateFlags,
    queueFamilyIndex: uint32_t,
  }
}

structure! {
  /// [VkComputePipelineCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkComputePipelineCreateInfo.html)
  VkComputePipelineCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Pipeline creation flags
    /// * **Optional:** true
    flags: VkPipelineCreateFlags,
    stage: VkPipelineShaderStageCreateInfo,
    /// Interface layout of the pipeline
    layout: VkPipelineLayout,
    /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is nonzero, it specifies the handle of the base pipeline this is a derivative of
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    basePipelineHandle: VkPipeline,
    /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is not -1, it specifies an index into pCreateInfos of the base pipeline this is a derivative of
    basePipelineIndex: int32_t,
  }
}

structure! {
  /// [VkConditionalRenderingBeginInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkConditionalRenderingBeginInfoEXT.html)
  VkConditionalRenderingBeginInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_CONDITIONAL_RENDERING_BEGIN_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    /// * **Optional:** true
    flags: VkConditionalRenderingFlagsEXT,
  }
}

structure! {
  /// [VkConformanceVersion](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkConformanceVersion.html)
  VkConformanceVersion {
    major: uint8_t,
    minor: uint8_t,
    subminor: uint8_t,
    patch: uint8_t,
  }
}

structure! {
  /// [VkCooperativeMatrixPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCooperativeMatrixPropertiesNV.html)
  VkCooperativeMatrixPropertiesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_COOPERATIVE_MATRIX_PROPERTIES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    MSize: uint32_t,
    NSize: uint32_t,
    KSize: uint32_t,
    AType: VkComponentTypeNV,
    BType: VkComponentTypeNV,
    CType: VkComponentTypeNV,
    DType: VkComponentTypeNV,
    scope: VkScopeNV,
  }
}

structure! {
  /// [VkCopyAccelerationStructureInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyAccelerationStructureInfoKHR.html)
  VkCopyAccelerationStructureInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    src: VkAccelerationStructureKHR,
    dst: VkAccelerationStructureKHR,
    mode: VkCopyAccelerationStructureModeKHR,
  }
}

structure! {
  /// [VkCopyAccelerationStructureToMemoryInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyAccelerationStructureToMemoryInfoKHR.html)
  VkCopyAccelerationStructureToMemoryInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    src: VkAccelerationStructureKHR,
    dst: VkDeviceOrHostAddressKHR,
    mode: VkCopyAccelerationStructureModeKHR,
  }
}

structure! {
  /// [VkCopyBufferInfo2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyBufferInfo2KHR.html)
  VkCopyBufferInfo2KHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    srcBuffer: VkBuffer,
    dstBuffer: VkBuffer,
    regionCount: uint32_t,
    /// * **Len:** regionCount
    pRegions: *const VkBufferCopy2KHR,
  }
}

structure! {
  /// [VkCopyBufferToImageInfo2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyBufferToImageInfo2KHR.html)
  VkCopyBufferToImageInfo2KHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    srcBuffer: VkBuffer,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: uint32_t,
    /// * **Len:** regionCount
    pRegions: *const VkBufferImageCopy2KHR,
  }
}

structure! {
  /// [VkCopyCommandTransformInfoQCOM](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyCommandTransformInfoQCOM.html)
  ///
  /// Struct Extends: [`VkBufferImageCopy2KHR`], [`VkImageBlit2KHR`]
  VkCopyCommandTransformInfoQCOM {
    /// * **Values:** [`VK_STRUCTURE_TYPE_COPY_COMMAND_TRANSFORM_INFO_QCOM`]
    sType: VkStructureType,
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    pNext: *const c_void,
    /// * **No Auto-validity:** true
    transform: VkSurfaceTransformFlagBitsKHR,
  }
}

structure! {
  /// [VkCopyDescriptorSet](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyDescriptorSet.html)
  VkCopyDescriptorSet {
    /// * **Values:** [`VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Source descriptor set
    srcSet: VkDescriptorSet,
    /// Binding within the source descriptor set to copy from
    srcBinding: uint32_t,
    /// Array element within the source binding to copy from
    srcArrayElement: uint32_t,
    /// Destination descriptor set
    dstSet: VkDescriptorSet,
    /// Binding within the destination descriptor set to copy to
    dstBinding: uint32_t,
    /// Array element within the destination binding to copy to
    dstArrayElement: uint32_t,
    /// Number of descriptors to write (determines the size of the array pointed by pDescriptors)
    descriptorCount: uint32_t,
  }
}

structure! {
  /// [VkCopyImageInfo2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyImageInfo2KHR.html)
  VkCopyImageInfo2KHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: uint32_t,
    /// * **Len:** regionCount
    pRegions: *const VkImageCopy2KHR,
  }
}

structure! {
  /// [VkCopyImageToBufferInfo2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyImageToBufferInfo2KHR.html)
  VkCopyImageToBufferInfo2KHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstBuffer: VkBuffer,
    regionCount: uint32_t,
    /// * **Len:** regionCount
    pRegions: *const VkBufferImageCopy2KHR,
  }
}

structure! {
  /// [VkCopyMemoryToAccelerationStructureInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyMemoryToAccelerationStructureInfoKHR.html)
  VkCopyMemoryToAccelerationStructureInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    src: VkDeviceOrHostAddressConstKHR,
    dst: VkAccelerationStructureKHR,
    mode: VkCopyAccelerationStructureModeKHR,
  }
}

structure! {
  /// [VkD3D12FenceSubmitInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkD3D12FenceSubmitInfoKHR.html)
  ///
  /// Struct Extends: [`VkSubmitInfo`]
  VkD3D12FenceSubmitInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    waitSemaphoreValuesCount: uint32_t,
    /// * **Optional:** true
    /// * **Len:** waitSemaphoreValuesCount
    pWaitSemaphoreValues: *const uint64_t,
    /// * **Optional:** true
    signalSemaphoreValuesCount: uint32_t,
    /// * **Optional:** true
    /// * **Len:** signalSemaphoreValuesCount
    pSignalSemaphoreValues: *const uint64_t,
  }
}

structure! {
  /// [VkDebugMarkerMarkerInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugMarkerMarkerInfoEXT.html)
  VkDebugMarkerMarkerInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Name of the debug marker
    /// * **Len:** null-terminated
    pMarkerName: *const u8,
    /// Optional color for debug marker
    color: [float; 4],
  }
}

structure! {
  /// [VkDebugMarkerObjectNameInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugMarkerObjectNameInfoEXT.html)
  VkDebugMarkerObjectNameInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// The type of the object
    objectType: VkDebugReportObjectTypeEXT,
    /// The handle of the object, cast to uint64_t
    object: uint64_t,
    /// Name to apply to the object
    /// * **Len:** null-terminated
    pObjectName: *const u8,
  }
}

structure! {
  /// [VkDebugMarkerObjectTagInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugMarkerObjectTagInfoEXT.html)
  VkDebugMarkerObjectTagInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// The type of the object
    objectType: VkDebugReportObjectTypeEXT,
    /// The handle of the object, cast to uint64_t
    object: uint64_t,
    /// The name of the tag to set on the object
    tagName: uint64_t,
    /// The length in bytes of the tag data
    tagSize: size_t,
    /// Tag data to attach to the object
    /// * **Len:** tagSize
    pTag: *const c_void,
  }
}

structure! {
  /// [VkDebugReportCallbackCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugReportCallbackCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkInstanceCreateInfo`]
  VkDebugReportCallbackCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Indicates which events call this callback
    /// * **Optional:** true
    flags: VkDebugReportFlagsEXT,
    /// Function pointer of a callback function
    pfnCallback: PFN_vkDebugReportCallbackEXT,
    /// User data provided to callback function
    /// * **Optional:** true
    pUserData: *mut c_void,
  }
}

structure! {
  /// [VkDebugUtilsMessengerCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessengerCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkInstanceCreateInfo`]
  VkDebugUtilsMessengerCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkDebugUtilsMessengerCreateFlagsEXT,
    messageSeverity: VkDebugUtilsMessageSeverityFlagsEXT,
    messageType: VkDebugUtilsMessageTypeFlagsEXT,
    pfnUserCallback: PFN_vkDebugUtilsMessengerCallbackEXT,
    /// * **Optional:** true
    pUserData: *mut c_void,
  }
}

structure! {
  /// [VkDebugUtilsObjectTagInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsObjectTagInfoEXT.html)
  VkDebugUtilsObjectTagInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    objectType: VkObjectType,
    objectHandle: uint64_t,
    tagName: uint64_t,
    tagSize: size_t,
    /// * **Len:** tagSize
    pTag: *const c_void,
  }
}

structure! {
  /// [VkDedicatedAllocationBufferCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDedicatedAllocationBufferCreateInfoNV.html)
  ///
  /// Struct Extends: [`VkBufferCreateInfo`]
  VkDedicatedAllocationBufferCreateInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Whether this buffer uses a dedicated allocation
    dedicatedAllocation: VkBool32,
  }
}

structure! {
  /// [VkDedicatedAllocationImageCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDedicatedAllocationImageCreateInfoNV.html)
  ///
  /// Struct Extends: [`VkImageCreateInfo`]
  VkDedicatedAllocationImageCreateInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Whether this image uses a dedicated allocation
    dedicatedAllocation: VkBool32,
  }
}

structure! {
  /// [VkDedicatedAllocationMemoryAllocateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDedicatedAllocationMemoryAllocateInfoNV.html)
  ///
  /// Struct Extends: [`VkMemoryAllocateInfo`]
  VkDedicatedAllocationMemoryAllocateInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Image that this allocation will be bound to
    /// * **Optional:** true
    image: VkImage,
    /// Buffer that this allocation will be bound to
    /// * **Optional:** true
    buffer: VkBuffer,
  }
}

structure! {
  /// [VkDescriptorBufferInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorBufferInfo.html)
  VkDescriptorBufferInfo {
    /// Buffer used for this descriptor slot.
    /// * **Optional:** true
    buffer: VkBuffer,
    /// Base offset from buffer start in bytes to update in the descriptor set.
    offset: VkDeviceSize,
    /// Size in bytes of the buffer resource for this descriptor update.
    range: VkDeviceSize,
  }
}

structure! {
  /// [VkDescriptorImageInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorImageInfo.html)
  VkDescriptorImageInfo {
    /// Sampler to write to the descriptor in case it is a SAMPLER or COMBINED_IMAGE_SAMPLER descriptor. Ignored otherwise.
    /// * **No Auto-validity:** true
    sampler: VkSampler,
    /// Image view to write to the descriptor in case it is a SAMPLED_IMAGE, STORAGE_IMAGE, COMBINED_IMAGE_SAMPLER, or INPUT_ATTACHMENT descriptor. Ignored otherwise.
    /// * **No Auto-validity:** true
    imageView: VkImageView,
    /// Layout the image is expected to be in when accessed using this descriptor (only used if imageView is not VK_NULL_HANDLE).
    /// * **No Auto-validity:** true
    imageLayout: VkImageLayout,
  }
}

structure! {
  /// [VkDescriptorPoolCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorPoolCreateInfo.html)
  VkDescriptorPoolCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkDescriptorPoolCreateFlags,
    maxSets: uint32_t,
    poolSizeCount: uint32_t,
    /// * **Len:** poolSizeCount
    pPoolSizes: *const VkDescriptorPoolSize,
  }
}

structure! {
  /// [VkDescriptorPoolInlineUniformBlockCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorPoolInlineUniformBlockCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkDescriptorPoolCreateInfo`]
  VkDescriptorPoolInlineUniformBlockCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    maxInlineUniformBlockBindings: uint32_t,
  }
}

structure! {
  /// [VkDescriptorPoolSize](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorPoolSize.html)
  VkDescriptorPoolSize {
    type_: VkDescriptorType,
    descriptorCount: uint32_t,
  }
}

structure! {
  /// [VkDescriptorSetAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetAllocateInfo.html)
  VkDescriptorSetAllocateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    descriptorPool: VkDescriptorPool,
    descriptorSetCount: uint32_t,
    /// * **Len:** descriptorSetCount
    pSetLayouts: *const VkDescriptorSetLayout,
  }
}

structure! {
  /// [VkDescriptorSetLayoutBinding](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutBinding.html)
  VkDescriptorSetLayoutBinding {
    /// Binding number for this entry
    binding: uint32_t,
    /// Type of the descriptors in this binding
    descriptorType: VkDescriptorType,
    /// Number of descriptors in this binding
    /// * **Optional:** true
    descriptorCount: uint32_t,
    /// Shader stages this binding is visible to
    /// * **No Auto-validity:** true
    stageFlags: VkShaderStageFlags,
    /// Immutable samplers (used if descriptor type is SAMPLER or COMBINED_IMAGE_SAMPLER, is either NULL or contains count number of elements)
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    /// * **Len:** descriptorCount
    pImmutableSamplers: *const VkSampler,
  }
}

structure! {
  /// [VkDescriptorSetLayoutBindingFlagsCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutBindingFlagsCreateInfo.html)
  ///
  /// Struct Extends: [`VkDescriptorSetLayoutCreateInfo`]
  VkDescriptorSetLayoutBindingFlagsCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    bindingCount: uint32_t,
    /// * **Optional:** false,true
    /// * **Len:** bindingCount
    pBindingFlags: *const VkDescriptorBindingFlags,
  }
}

structure! {
  /// [VkDescriptorSetLayoutCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutCreateInfo.html)
  VkDescriptorSetLayoutCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkDescriptorSetLayoutCreateFlags,
    /// Number of bindings in the descriptor set layout
    /// * **Optional:** true
    bindingCount: uint32_t,
    /// Array of descriptor set layout bindings
    /// * **Len:** bindingCount
    pBindings: *const VkDescriptorSetLayoutBinding,
  }
}

structure! {
  /// [VkDescriptorSetLayoutSupport](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutSupport.html)
  VkDescriptorSetLayoutSupport {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    supported: VkBool32,
  }
}

structure! {
  /// [VkDescriptorSetVariableDescriptorCountAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetVariableDescriptorCountAllocateInfo.html)
  ///
  /// Struct Extends: [`VkDescriptorSetAllocateInfo`]
  VkDescriptorSetVariableDescriptorCountAllocateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    descriptorSetCount: uint32_t,
    /// * **Len:** descriptorSetCount
    pDescriptorCounts: *const uint32_t,
  }
}

structure! {
  /// [VkDescriptorSetVariableDescriptorCountLayoutSupport](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetVariableDescriptorCountLayoutSupport.html)
  ///
  /// Struct Extends: [`VkDescriptorSetLayoutSupport`]
  VkDescriptorSetVariableDescriptorCountLayoutSupport {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    maxVariableDescriptorCount: uint32_t,
  }
}

structure! {
  /// [VkDescriptorUpdateTemplateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateCreateInfo.html)
  VkDescriptorUpdateTemplateCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkDescriptorUpdateTemplateCreateFlags,
    /// Number of descriptor update entries to use for the update template
    descriptorUpdateEntryCount: uint32_t,
    /// Descriptor update entries for the template
    /// * **Len:** descriptorUpdateEntryCount
    pDescriptorUpdateEntries: *const VkDescriptorUpdateTemplateEntry,
    templateType: VkDescriptorUpdateTemplateType,
    /// * **No Auto-validity:** true
    descriptorSetLayout: VkDescriptorSetLayout,
    /// * **No Auto-validity:** true
    pipelineBindPoint: VkPipelineBindPoint,
    /// If used for push descriptors, this is the only allowed layout
    /// * **No Auto-validity:** true
    pipelineLayout: VkPipelineLayout,
    /// * **No Auto-validity:** true
    set: uint32_t,
  }
}

structure! {
  /// [VkDescriptorUpdateTemplateEntry](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateEntry.html)
  VkDescriptorUpdateTemplateEntry {
    /// Binding within the destination descriptor set to write
    dstBinding: uint32_t,
    /// Array element within the destination binding to write
    dstArrayElement: uint32_t,
    /// Number of descriptors to write
    descriptorCount: uint32_t,
    /// Descriptor type to write
    descriptorType: VkDescriptorType,
    /// Offset into pData where the descriptors to update are stored
    offset: size_t,
    /// Stride between two descriptors in pData when writing more than one descriptor
    stride: size_t,
  }
}

structure! {
  /// [VkDeviceDeviceMemoryReportCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceDeviceMemoryReportCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkDeviceCreateInfo`]
  VkDeviceDeviceMemoryReportCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    flags: VkDeviceMemoryReportFlagsEXT,
    pfnUserCallback: PFN_vkDeviceMemoryReportCallbackEXT,
    pUserData: *mut c_void,
  }
}

structure! {
  /// [VkDeviceDiagnosticsConfigCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceDiagnosticsConfigCreateInfoNV.html)
  ///
  /// Struct Extends: [`VkDeviceCreateInfo`]
  VkDeviceDiagnosticsConfigCreateInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkDeviceDiagnosticsConfigFlagsNV,
  }
}

structure! {
  /// [VkDeviceEventInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceEventInfoEXT.html)
  VkDeviceEventInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    deviceEvent: VkDeviceEventTypeEXT,
  }
}

structure! {
  /// [VkDeviceGroupBindSparseInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupBindSparseInfo.html)
  ///
  /// Struct Extends: [`VkBindSparseInfo`]
  VkDeviceGroupBindSparseInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    resourceDeviceIndex: uint32_t,
    memoryDeviceIndex: uint32_t,
  }
}

structure! {
  /// [VkDeviceGroupCommandBufferBeginInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupCommandBufferBeginInfo.html)
  ///
  /// Struct Extends: [`VkCommandBufferBeginInfo`]
  VkDeviceGroupCommandBufferBeginInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    deviceMask: uint32_t,
  }
}

structure! {
  /// [VkDeviceGroupDeviceCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupDeviceCreateInfo.html)
  ///
  /// Struct Extends: [`VkDeviceCreateInfo`]
  VkDeviceGroupDeviceCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    physicalDeviceCount: uint32_t,
    /// * **Len:** physicalDeviceCount
    pPhysicalDevices: *const VkPhysicalDevice,
  }
}

structure! {
  /// [VkDeviceGroupPresentCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupPresentCapabilitiesKHR.html)
  VkDeviceGroupPresentCapabilitiesKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    presentMask: [uint32_t; VK_MAX_DEVICE_GROUP_SIZE as usize],
    modes: VkDeviceGroupPresentModeFlagsKHR,
  }
}

structure! {
  /// [VkDeviceGroupPresentInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupPresentInfoKHR.html)
  ///
  /// Struct Extends: [`VkPresentInfoKHR`]
  VkDeviceGroupPresentInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    swapchainCount: uint32_t,
    /// * **Len:** swapchainCount
    pDeviceMasks: *const uint32_t,
    mode: VkDeviceGroupPresentModeFlagBitsKHR,
  }
}

structure! {
  /// [VkDeviceGroupRenderPassBeginInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupRenderPassBeginInfo.html)
  ///
  /// Struct Extends: [`VkRenderPassBeginInfo`]
  VkDeviceGroupRenderPassBeginInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    deviceMask: uint32_t,
    /// * **Optional:** true
    deviceRenderAreaCount: uint32_t,
    /// * **Len:** deviceRenderAreaCount
    pDeviceRenderAreas: *const VkRect2D,
  }
}

structure! {
  /// [VkDeviceGroupSubmitInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupSubmitInfo.html)
  ///
  /// Struct Extends: [`VkSubmitInfo`]
  VkDeviceGroupSubmitInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    waitSemaphoreCount: uint32_t,
    /// * **Len:** waitSemaphoreCount
    pWaitSemaphoreDeviceIndices: *const uint32_t,
    /// * **Optional:** true
    commandBufferCount: uint32_t,
    /// * **Len:** commandBufferCount
    pCommandBufferDeviceMasks: *const uint32_t,
    /// * **Optional:** true
    signalSemaphoreCount: uint32_t,
    /// * **Len:** signalSemaphoreCount
    pSignalSemaphoreDeviceIndices: *const uint32_t,
  }
}

structure! {
  /// [VkDeviceGroupSwapchainCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupSwapchainCreateInfoKHR.html)
  ///
  /// Struct Extends: [`VkSwapchainCreateInfoKHR`]
  VkDeviceGroupSwapchainCreateInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    modes: VkDeviceGroupPresentModeFlagsKHR,
  }
}

structure! {
  /// [VkDeviceMemoryOpaqueCaptureAddressInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceMemoryOpaqueCaptureAddressInfo.html)
  VkDeviceMemoryOpaqueCaptureAddressInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    memory: VkDeviceMemory,
  }
}

structure! {
  /// [VkDeviceMemoryOverallocationCreateInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceMemoryOverallocationCreateInfoAMD.html)
  ///
  /// Struct Extends: [`VkDeviceCreateInfo`]
  VkDeviceMemoryOverallocationCreateInfoAMD {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    overallocationBehavior: VkMemoryOverallocationBehaviorAMD,
  }
}

structure! {
  /// [VkDevicePrivateDataCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDevicePrivateDataCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkDeviceCreateInfo`]
  VkDevicePrivateDataCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    privateDataSlotRequestCount: uint32_t,
  }
}

structure! {
  /// [VkDeviceQueueGlobalPriorityCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceQueueGlobalPriorityCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkDeviceQueueCreateInfo`]
  VkDeviceQueueGlobalPriorityCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    globalPriority: VkQueueGlobalPriorityEXT,
  }
}

structure! {
  /// [VkDeviceQueueInfo2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceQueueInfo2.html)
  VkDeviceQueueInfo2 {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkDeviceQueueCreateFlags,
    queueFamilyIndex: uint32_t,
    queueIndex: uint32_t,
  }
}

structure! {
  /// [VkDirectFBSurfaceCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDirectFBSurfaceCreateInfoEXT.html)
  VkDirectFBSurfaceCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DIRECTFB_SURFACE_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkDirectFBSurfaceCreateFlagsEXT,
    /// * **No Auto-validity:** true
    dfb: *mut IDirectFB,
    /// * **No Auto-validity:** true
    surface: *mut IDirectFBSurface,
  }
}

structure! {
  /// [VkDisplayEventInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayEventInfoEXT.html)
  VkDisplayEventInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    displayEvent: VkDisplayEventTypeEXT,
  }
}

structure! {
  /// [VkDisplayModeCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayModeCreateInfoKHR.html)
  VkDisplayModeCreateInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkDisplayModeCreateFlagsKHR,
    /// The parameters this mode uses.
    parameters: VkDisplayModeParametersKHR,
  }
}

structure! {
  /// [VkDisplayModeParametersKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayModeParametersKHR.html)
  VkDisplayModeParametersKHR {
    /// Visible scanout region.
    visibleRegion: VkExtent2D,
    /// Number of times per second the display is updated.
    /// * **No Auto-validity:** true
    refreshRate: uint32_t,
  }
}

structure! {
  /// [VkDisplayModeProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayModeProperties2KHR.html)
  VkDisplayModeProperties2KHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DISPLAY_MODE_PROPERTIES_2_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    displayModeProperties: VkDisplayModePropertiesKHR,
  }
}

structure! {
  /// [VkDisplayModePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayModePropertiesKHR.html)
  VkDisplayModePropertiesKHR {
    /// Handle of this display mode.
    displayMode: VkDisplayModeKHR,
    /// The parameters this mode uses.
    parameters: VkDisplayModeParametersKHR,
  }
}

structure! {
  /// [VkDisplayNativeHdrSurfaceCapabilitiesAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayNativeHdrSurfaceCapabilitiesAMD.html)
  ///
  /// Struct Extends: [`VkSurfaceCapabilities2KHR`]
  VkDisplayNativeHdrSurfaceCapabilitiesAMD {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    localDimmingSupport: VkBool32,
  }
}

structure! {
  /// [VkDisplayPlaneCapabilities2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlaneCapabilities2KHR.html)
  VkDisplayPlaneCapabilities2KHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DISPLAY_PLANE_CAPABILITIES_2_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    capabilities: VkDisplayPlaneCapabilitiesKHR,
  }
}

structure! {
  /// [VkDisplayPlaneCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlaneCapabilitiesKHR.html)
  VkDisplayPlaneCapabilitiesKHR {
    /// Types of alpha blending supported, if any.
    /// * **Optional:** true
    supportedAlpha: VkDisplayPlaneAlphaFlagsKHR,
    /// Does the plane have any position and extent restrictions?
    minSrcPosition: VkOffset2D,
    maxSrcPosition: VkOffset2D,
    minSrcExtent: VkExtent2D,
    maxSrcExtent: VkExtent2D,
    minDstPosition: VkOffset2D,
    maxDstPosition: VkOffset2D,
    minDstExtent: VkExtent2D,
    maxDstExtent: VkExtent2D,
  }
}

structure! {
  /// [VkDisplayPlaneInfo2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlaneInfo2KHR.html)
  VkDisplayPlaneInfo2KHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DISPLAY_PLANE_INFO_2_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Extern Sync:** true
    mode: VkDisplayModeKHR,
    planeIndex: uint32_t,
  }
}

structure! {
  /// [VkDisplayPlaneProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlaneProperties2KHR.html)
  VkDisplayPlaneProperties2KHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DISPLAY_PLANE_PROPERTIES_2_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    displayPlaneProperties: VkDisplayPlanePropertiesKHR,
  }
}

structure! {
  /// [VkDisplayPlanePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPlanePropertiesKHR.html)
  VkDisplayPlanePropertiesKHR {
    /// Display the plane is currently associated with.  Will be VK_NULL_HANDLE if the plane is not in use.
    currentDisplay: VkDisplayKHR,
    /// Current z-order of the plane.
    currentStackIndex: uint32_t,
  }
}

structure! {
  /// [VkDisplayPowerInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPowerInfoEXT.html)
  VkDisplayPowerInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    powerState: VkDisplayPowerStateEXT,
  }
}

structure! {
  /// [VkDisplayPresentInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPresentInfoKHR.html)
  ///
  /// Struct Extends: [`VkPresentInfoKHR`]
  VkDisplayPresentInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Rectangle within the presentable image to read pixel data from when presenting to the display.
    srcRect: VkRect2D,
    /// Rectangle within the current display mode's visible region to display srcRectangle in.
    dstRect: VkRect2D,
    /// For smart displays, use buffered mode.  If the display properties member "persistentMode" is VK_FALSE, this member must always be VK_FALSE.
    persistent: VkBool32,
  }
}

structure! {
  /// [VkDisplayProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayProperties2KHR.html)
  VkDisplayProperties2KHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DISPLAY_PROPERTIES_2_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    displayProperties: VkDisplayPropertiesKHR,
  }
}

structure! {
  /// [VkDisplayPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPropertiesKHR.html)
  VkDisplayPropertiesKHR {
    /// Handle of the display object
    display: VkDisplayKHR,
    /// Name of the display
    /// * **Len:** null-terminated
    displayName: *const u8,
    /// In millimeters?
    physicalDimensions: VkExtent2D,
    /// Max resolution for CRT?
    physicalResolution: VkExtent2D,
    /// one or more bits from VkSurfaceTransformFlagsKHR
    /// * **Optional:** true
    supportedTransforms: VkSurfaceTransformFlagsKHR,
    /// VK_TRUE if the overlay plane's z-order can be changed on this display.
    planeReorderPossible: VkBool32,
    /// VK_TRUE if this is a "smart" display that supports self-refresh/internal buffering.
    persistentContent: VkBool32,
  }
}

structure! {
  /// [VkDisplaySurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplaySurfaceCreateInfoKHR.html)
  VkDisplaySurfaceCreateInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkDisplaySurfaceCreateFlagsKHR,
    /// The mode to use when displaying this surface
    displayMode: VkDisplayModeKHR,
    /// The plane on which this surface appears.  Must be between 0 and the value returned by vkGetPhysicalDeviceDisplayPlanePropertiesKHR() in pPropertyCount.
    planeIndex: uint32_t,
    /// The z-order of the plane.
    planeStackIndex: uint32_t,
    /// Transform to apply to the images as part of the scanout operation
    transform: VkSurfaceTransformFlagBitsKHR,
    /// Global alpha value.  Must be between 0 and 1, inclusive.  Ignored if alphaMode is not VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR
    globalAlpha: float,
    /// What type of alpha blending to use.  Must be a bit from vkGetDisplayPlanePropertiesKHR::supportedAlpha.
    alphaMode: VkDisplayPlaneAlphaFlagBitsKHR,
    /// size of the images to use with this surface
    imageExtent: VkExtent2D,
  }
}

structure! {
  /// [VkDrawMeshTasksIndirectCommandNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDrawMeshTasksIndirectCommandNV.html)
  VkDrawMeshTasksIndirectCommandNV {
    taskCount: uint32_t,
    firstTask: uint32_t,
  }
}

structure! {
  /// [VkDrmFormatModifierPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDrmFormatModifierPropertiesEXT.html)
  VkDrmFormatModifierPropertiesEXT {
    drmFormatModifier: uint64_t,
    drmFormatModifierPlaneCount: uint32_t,
    drmFormatModifierTilingFeatures: VkFormatFeatureFlags,
  }
}

structure! {
  /// [VkDrmFormatModifierPropertiesListEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDrmFormatModifierPropertiesListEXT.html)
  ///
  /// Struct Extends: [`VkFormatProperties2`]
  VkDrmFormatModifierPropertiesListEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    /// * **Optional:** true
    drmFormatModifierCount: uint32_t,
    /// * **Optional:** true,false
    /// * **Len:** drmFormatModifierCount
    pDrmFormatModifierProperties: *mut VkDrmFormatModifierPropertiesEXT,
  }
}

structure! {
  /// [VkEventCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkEventCreateInfo.html)
  VkEventCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_EVENT_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Event creation flags
    /// * **Optional:** true
    flags: VkEventCreateFlags,
  }
}

structure! {
  /// [VkExportFenceCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportFenceCreateInfo.html)
  ///
  /// Struct Extends: [`VkFenceCreateInfo`]
  VkExportFenceCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    handleTypes: VkExternalFenceHandleTypeFlags,
  }
}

structure! {
  /// [VkExportFenceWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportFenceWin32HandleInfoKHR.html)
  ///
  /// Struct Extends: [`VkFenceCreateInfo`]
  VkExportFenceWin32HandleInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    pAttributes: *const SECURITY_ATTRIBUTES,
    dwAccess: DWORD,
    name: LPCWSTR,
  }
}

structure! {
  /// [VkExportMemoryAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportMemoryAllocateInfo.html)
  ///
  /// Struct Extends: [`VkMemoryAllocateInfo`]
  VkExportMemoryAllocateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    handleTypes: VkExternalMemoryHandleTypeFlags,
  }
}

structure! {
  /// [VkExportMemoryAllocateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportMemoryAllocateInfoNV.html)
  ///
  /// Struct Extends: [`VkMemoryAllocateInfo`]
  VkExportMemoryAllocateInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    handleTypes: VkExternalMemoryHandleTypeFlagsNV,
  }
}

structure! {
  /// [VkExportMemoryWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportMemoryWin32HandleInfoKHR.html)
  ///
  /// Struct Extends: [`VkMemoryAllocateInfo`]
  VkExportMemoryWin32HandleInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    pAttributes: *const SECURITY_ATTRIBUTES,
    dwAccess: DWORD,
    name: LPCWSTR,
  }
}

structure! {
  /// [VkExportMemoryWin32HandleInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportMemoryWin32HandleInfoNV.html)
  ///
  /// Struct Extends: [`VkMemoryAllocateInfo`]
  VkExportMemoryWin32HandleInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    pAttributes: *const SECURITY_ATTRIBUTES,
    /// * **Optional:** true
    dwAccess: DWORD,
  }
}

structure! {
  /// [VkExportSemaphoreCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportSemaphoreCreateInfo.html)
  ///
  /// Struct Extends: [`VkSemaphoreCreateInfo`]
  VkExportSemaphoreCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    handleTypes: VkExternalSemaphoreHandleTypeFlags,
  }
}

structure! {
  /// [VkExportSemaphoreWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportSemaphoreWin32HandleInfoKHR.html)
  ///
  /// Struct Extends: [`VkSemaphoreCreateInfo`]
  VkExportSemaphoreWin32HandleInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    pAttributes: *const SECURITY_ATTRIBUTES,
    dwAccess: DWORD,
    name: LPCWSTR,
  }
}

structure! {
  /// [VkExternalBufferProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalBufferProperties.html)
  VkExternalBufferProperties {
    /// * **Values:** [`VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    externalMemoryProperties: VkExternalMemoryProperties,
  }
}

structure! {
  /// [VkExternalFenceProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFenceProperties.html)
  VkExternalFenceProperties {
    /// * **Values:** [`VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    exportFromImportedHandleTypes: VkExternalFenceHandleTypeFlags,
    compatibleHandleTypes: VkExternalFenceHandleTypeFlags,
    /// * **Optional:** true
    externalFenceFeatures: VkExternalFenceFeatureFlags,
  }
}

structure! {
  /// [VkExternalFormatANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalFormatANDROID.html)
  ///
  /// Struct Extends: [`VkImageCreateInfo`], [`VkSamplerYcbcrConversionCreateInfo`]
  VkExternalFormatANDROID {
    /// * **Values:** [`VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_ANDROID`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    externalFormat: uint64_t,
  }
}

structure! {
  /// [VkExternalImageFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalImageFormatProperties.html)
  ///
  /// Struct Extends: [`VkImageFormatProperties2`]
  VkExternalImageFormatProperties {
    /// * **Values:** [`VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    externalMemoryProperties: VkExternalMemoryProperties,
  }
}

structure! {
  /// [VkExternalImageFormatPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalImageFormatPropertiesNV.html)
  VkExternalImageFormatPropertiesNV {
    imageFormatProperties: VkImageFormatProperties,
    /// * **Optional:** true
    externalMemoryFeatures: VkExternalMemoryFeatureFlagsNV,
    /// * **Optional:** true
    exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagsNV,
    /// * **Optional:** true
    compatibleHandleTypes: VkExternalMemoryHandleTypeFlagsNV,
  }
}

structure! {
  /// [VkExternalMemoryBufferCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryBufferCreateInfo.html)
  ///
  /// Struct Extends: [`VkBufferCreateInfo`]
  VkExternalMemoryBufferCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    handleTypes: VkExternalMemoryHandleTypeFlags,
  }
}

structure! {
  /// [VkExternalMemoryImageCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryImageCreateInfo.html)
  ///
  /// Struct Extends: [`VkImageCreateInfo`]
  VkExternalMemoryImageCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    handleTypes: VkExternalMemoryHandleTypeFlags,
  }
}

structure! {
  /// [VkExternalMemoryImageCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryImageCreateInfoNV.html)
  ///
  /// Struct Extends: [`VkImageCreateInfo`]
  VkExternalMemoryImageCreateInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    handleTypes: VkExternalMemoryHandleTypeFlagsNV,
  }
}

structure! {
  /// [VkExternalMemoryProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalMemoryProperties.html)
  VkExternalMemoryProperties {
    externalMemoryFeatures: VkExternalMemoryFeatureFlags,
    /// * **Optional:** true
    exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlags,
    compatibleHandleTypes: VkExternalMemoryHandleTypeFlags,
  }
}

structure! {
  /// [VkExternalSemaphoreProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExternalSemaphoreProperties.html)
  VkExternalSemaphoreProperties {
    /// * **Values:** [`VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    exportFromImportedHandleTypes: VkExternalSemaphoreHandleTypeFlags,
    compatibleHandleTypes: VkExternalSemaphoreHandleTypeFlags,
    /// * **Optional:** true
    externalSemaphoreFeatures: VkExternalSemaphoreFeatureFlags,
  }
}

structure! {
  /// [VkFenceCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceCreateInfo.html)
  VkFenceCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_FENCE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Fence creation flags
    /// * **Optional:** true
    flags: VkFenceCreateFlags,
  }
}

structure! {
  /// [VkFenceGetFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceGetFdInfoKHR.html)
  VkFenceGetFdInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    fence: VkFence,
    handleType: VkExternalFenceHandleTypeFlagBits,
  }
}

structure! {
  /// [VkFenceGetWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceGetWin32HandleInfoKHR.html)
  VkFenceGetWin32HandleInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    fence: VkFence,
    handleType: VkExternalFenceHandleTypeFlagBits,
  }
}

structure! {
  /// [VkFilterCubicImageViewImageFormatPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFilterCubicImageViewImageFormatPropertiesEXT.html)
  ///
  /// Struct Extends: [`VkImageFormatProperties2`]
  VkFilterCubicImageViewImageFormatPropertiesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    /// The combinations of format, image type (and image view type if provided) can be filtered with VK_FILTER_CUBIC_EXT
    filterCubic: VkBool32,
    /// The combination of format, image type (and image view type if provided) can be filtered with VK_FILTER_CUBIC_EXT and ReductionMode of Min or Max
    filterCubicMinmax: VkBool32,
  }
}

structure! {
  /// [VkFormatProperties2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFormatProperties2.html)
  VkFormatProperties2 {
    /// * **Values:** [`VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    formatProperties: VkFormatProperties,
  }
}

structure! {
  /// [VkFragmentShadingRateAttachmentInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFragmentShadingRateAttachmentInfoKHR.html)
  ///
  /// Struct Extends: [`VkSubpassDescription2`]
  VkFragmentShadingRateAttachmentInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    pFragmentShadingRateAttachment: *const VkAttachmentReference2,
    shadingRateAttachmentTexelSize: VkExtent2D,
  }
}

structure! {
  /// [VkFramebufferAttachmentImageInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebufferAttachmentImageInfo.html)
  VkFramebufferAttachmentImageInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Image creation flags
    /// * **Optional:** true
    flags: VkImageCreateFlags,
    /// Image usage flags
    usage: VkImageUsageFlags,
    width: uint32_t,
    height: uint32_t,
    layerCount: uint32_t,
    /// * **Optional:** true
    viewFormatCount: uint32_t,
    /// * **Len:** viewFormatCount
    pViewFormats: *const VkFormat,
  }
}

structure! {
  /// [VkFramebufferAttachmentsCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebufferAttachmentsCreateInfo.html)
  ///
  /// Struct Extends: [`VkFramebufferCreateInfo`]
  VkFramebufferAttachmentsCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    attachmentImageInfoCount: uint32_t,
    /// * **Len:** attachmentImageInfoCount
    pAttachmentImageInfos: *const VkFramebufferAttachmentImageInfo,
  }
}

structure! {
  /// [VkFramebufferCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebufferCreateInfo.html)
  VkFramebufferCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkFramebufferCreateFlags,
    renderPass: VkRenderPass,
    /// * **Optional:** true
    attachmentCount: uint32_t,
    /// * **No Auto-validity:** true
    /// * **Len:** attachmentCount
    pAttachments: *const VkImageView,
    width: uint32_t,
    height: uint32_t,
    layers: uint32_t,
  }
}

structure! {
  /// [VkFramebufferMixedSamplesCombinationNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebufferMixedSamplesCombinationNV.html)
  VkFramebufferMixedSamplesCombinationNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    coverageReductionMode: VkCoverageReductionModeNV,
    rasterizationSamples: VkSampleCountFlagBits,
    depthStencilSamples: VkSampleCountFlags,
    colorSamples: VkSampleCountFlags,
  }
}

structure! {
  /// [VkGeneratedCommandsInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeneratedCommandsInfoNV.html)
  VkGeneratedCommandsInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_GENERATED_COMMANDS_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    pipelineBindPoint: VkPipelineBindPoint,
    pipeline: VkPipeline,
    indirectCommandsLayout: VkIndirectCommandsLayoutNV,
    streamCount: uint32_t,
    /// * **Len:** streamCount
    pStreams: *const VkIndirectCommandsStreamNV,
    sequencesCount: uint32_t,
    preprocessBuffer: VkBuffer,
    preprocessOffset: VkDeviceSize,
    preprocessSize: VkDeviceSize,
    /// * **Optional:** true
    sequencesCountBuffer: VkBuffer,
    sequencesCountOffset: VkDeviceSize,
    /// * **Optional:** true
    sequencesIndexBuffer: VkBuffer,
    sequencesIndexOffset: VkDeviceSize,
  }
}

structure! {
  /// [VkGeneratedCommandsMemoryRequirementsInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeneratedCommandsMemoryRequirementsInfoNV.html)
  VkGeneratedCommandsMemoryRequirementsInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    pipelineBindPoint: VkPipelineBindPoint,
    pipeline: VkPipeline,
    indirectCommandsLayout: VkIndirectCommandsLayoutNV,
    maxSequencesCount: uint32_t,
  }
}

structure! {
  /// [VkGeometryAABBNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryAABBNV.html)
  VkGeometryAABBNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_GEOMETRY_AABB_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    aabbData: VkBuffer,
    numAABBs: uint32_t,
    /// Stride in bytes between AABBs
    stride: uint32_t,
    /// Offset in bytes of the first AABB in aabbData
    offset: VkDeviceSize,
  }
}

structure! {
  /// [VkGeometryDataNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryDataNV.html)
  VkGeometryDataNV {
    triangles: VkGeometryTrianglesNV,
    aabbs: VkGeometryAABBNV,
  }
}

structure! {
  /// [VkGeometryNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryNV.html)
  VkGeometryNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_GEOMETRY_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    geometryType: VkGeometryTypeKHR,
    geometry: VkGeometryDataNV,
    /// * **Optional:** true
    flags: VkGeometryFlagsKHR,
  }
}

structure! {
  /// [VkGeometryTrianglesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryTrianglesNV.html)
  VkGeometryTrianglesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_GEOMETRY_TRIANGLES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    vertexData: VkBuffer,
    vertexOffset: VkDeviceSize,
    vertexCount: uint32_t,
    vertexStride: VkDeviceSize,
    vertexFormat: VkFormat,
    /// * **Optional:** true
    indexData: VkBuffer,
    indexOffset: VkDeviceSize,
    indexCount: uint32_t,
    indexType: VkIndexType,
    /// Optional reference to array of floats representing a 3x4 row major affine transformation matrix.
    /// * **Optional:** true
    transformData: VkBuffer,
    transformOffset: VkDeviceSize,
  }
}

structure! {
  /// [VkGraphicsPipelineCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGraphicsPipelineCreateInfo.html)
  VkGraphicsPipelineCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Pipeline creation flags
    /// * **Optional:** true
    flags: VkPipelineCreateFlags,
    stageCount: uint32_t,
    /// One entry for each active shader stage
    /// * **Len:** stageCount
    pStages: *const VkPipelineShaderStageCreateInfo,
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    pInputAssemblyState: *const VkPipelineInputAssemblyStateCreateInfo,
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    pTessellationState: *const VkPipelineTessellationStateCreateInfo,
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    pViewportState: *const VkPipelineViewportStateCreateInfo,
    pRasterizationState: *const VkPipelineRasterizationStateCreateInfo,
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    pMultisampleState: *const VkPipelineMultisampleStateCreateInfo,
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    pDepthStencilState: *const VkPipelineDepthStencilStateCreateInfo,
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    pColorBlendState: *const VkPipelineColorBlendStateCreateInfo,
    /// * **Optional:** true
    pDynamicState: *const VkPipelineDynamicStateCreateInfo,
    /// Interface layout of the pipeline
    layout: VkPipelineLayout,
    renderPass: VkRenderPass,
    subpass: uint32_t,
    /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is nonzero, it specifies the handle of the base pipeline this is a derivative of
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    basePipelineHandle: VkPipeline,
    /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is not -1, it specifies an index into pCreateInfos of the base pipeline this is a derivative of
    basePipelineIndex: int32_t,
  }
}

structure! {
  /// [VkGraphicsPipelineShaderGroupsCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGraphicsPipelineShaderGroupsCreateInfoNV.html)
  ///
  /// Struct Extends: [`VkGraphicsPipelineCreateInfo`]
  VkGraphicsPipelineShaderGroupsCreateInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    groupCount: uint32_t,
    /// * **Len:** groupCount
    pGroups: *const VkGraphicsShaderGroupCreateInfoNV,
    /// * **Optional:** true
    pipelineCount: uint32_t,
    /// * **Len:** pipelineCount
    pPipelines: *const VkPipeline,
  }
}

structure! {
  /// [VkGraphicsShaderGroupCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGraphicsShaderGroupCreateInfoNV.html)
  VkGraphicsShaderGroupCreateInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_GRAPHICS_SHADER_GROUP_CREATE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    stageCount: uint32_t,
    /// * **Len:** stageCount
    pStages: *const VkPipelineShaderStageCreateInfo,
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    pTessellationState: *const VkPipelineTessellationStateCreateInfo,
  }
}

structure! {
  /// [VkHdrMetadataEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkHdrMetadataEXT.html)
  VkHdrMetadataEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_HDR_METADATA_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Display primary's Red
    /// * **No Auto-validity:** true
    displayPrimaryRed: VkXYColorEXT,
    /// Display primary's Green
    /// * **No Auto-validity:** true
    displayPrimaryGreen: VkXYColorEXT,
    /// Display primary's Blue
    /// * **No Auto-validity:** true
    displayPrimaryBlue: VkXYColorEXT,
    /// Display primary's Blue
    /// * **No Auto-validity:** true
    whitePoint: VkXYColorEXT,
    /// Display maximum luminance
    /// * **No Auto-validity:** true
    maxLuminance: float,
    /// Display minimum luminance
    /// * **No Auto-validity:** true
    minLuminance: float,
    /// Content maximum luminance
    /// * **No Auto-validity:** true
    maxContentLightLevel: float,
    /// * **No Auto-validity:** true
    maxFrameAverageLightLevel: float,
  }
}

structure! {
  /// [VkHeadlessSurfaceCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkHeadlessSurfaceCreateInfoEXT.html)
  VkHeadlessSurfaceCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_HEADLESS_SURFACE_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkHeadlessSurfaceCreateFlagsEXT,
  }
}

structure! {
  /// [VkIOSSurfaceCreateInfoMVK](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIOSSurfaceCreateInfoMVK.html)
  VkIOSSurfaceCreateInfoMVK {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkIOSSurfaceCreateFlagsMVK,
    /// * **No Auto-validity:** true
    pView: *const c_void,
  }
}

structure! {
  /// [VkImageBlit](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageBlit.html)
  VkImageBlit {
    srcSubresource: VkImageSubresourceLayers,
    /// Specified in pixels for both compressed and uncompressed images
    srcOffsets: [VkOffset3D; 2],
    dstSubresource: VkImageSubresourceLayers,
    /// Specified in pixels for both compressed and uncompressed images
    dstOffsets: [VkOffset3D; 2],
  }
}

structure! {
  /// [VkImageBlit2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageBlit2KHR.html)
  VkImageBlit2KHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_BLIT_2_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    srcSubresource: VkImageSubresourceLayers,
    /// Specified in pixels for both compressed and uncompressed images
    srcOffsets: [VkOffset3D; 2],
    dstSubresource: VkImageSubresourceLayers,
    /// Specified in pixels for both compressed and uncompressed images
    dstOffsets: [VkOffset3D; 2],
  }
}

structure! {
  /// [VkImageCopy](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageCopy.html)
  VkImageCopy {
    srcSubresource: VkImageSubresourceLayers,
    /// Specified in pixels for both compressed and uncompressed images
    srcOffset: VkOffset3D,
    dstSubresource: VkImageSubresourceLayers,
    /// Specified in pixels for both compressed and uncompressed images
    dstOffset: VkOffset3D,
    /// Specified in pixels for both compressed and uncompressed images
    extent: VkExtent3D,
  }
}

structure! {
  /// [VkImageCopy2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageCopy2KHR.html)
  VkImageCopy2KHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_COPY_2_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    srcSubresource: VkImageSubresourceLayers,
    /// Specified in pixels for both compressed and uncompressed images
    srcOffset: VkOffset3D,
    dstSubresource: VkImageSubresourceLayers,
    /// Specified in pixels for both compressed and uncompressed images
    dstOffset: VkOffset3D,
    /// Specified in pixels for both compressed and uncompressed images
    extent: VkExtent3D,
  }
}

structure! {
  /// [VkImageCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageCreateInfo.html)
  VkImageCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Image creation flags
    /// * **Optional:** true
    flags: VkImageCreateFlags,
    imageType: VkImageType,
    format: VkFormat,
    extent: VkExtent3D,
    mipLevels: uint32_t,
    arrayLayers: uint32_t,
    samples: VkSampleCountFlagBits,
    tiling: VkImageTiling,
    /// Image usage flags
    usage: VkImageUsageFlags,
    /// Cross-queue-family sharing mode
    sharingMode: VkSharingMode,
    /// Number of queue families to share across
    /// * **Optional:** true
    queueFamilyIndexCount: uint32_t,
    /// Array of queue family indices to share across
    /// * **No Auto-validity:** true
    /// * **Len:** queueFamilyIndexCount
    pQueueFamilyIndices: *const uint32_t,
    /// Initial image layout for all subresources
    initialLayout: VkImageLayout,
  }
}

structure! {
  /// [VkImageDrmFormatModifierExplicitCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageDrmFormatModifierExplicitCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkImageCreateInfo`]
  VkImageDrmFormatModifierExplicitCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    drmFormatModifier: uint64_t,
    /// * **Optional:** false
    drmFormatModifierPlaneCount: uint32_t,
    /// * **Len:** drmFormatModifierPlaneCount
    pPlaneLayouts: *const VkSubresourceLayout,
  }
}

structure! {
  /// [VkImageDrmFormatModifierListCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageDrmFormatModifierListCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkImageCreateInfo`]
  VkImageDrmFormatModifierListCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    drmFormatModifierCount: uint32_t,
    /// * **Len:** drmFormatModifierCount
    pDrmFormatModifiers: *const uint64_t,
  }
}

structure! {
  /// [VkImageDrmFormatModifierPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageDrmFormatModifierPropertiesEXT.html)
  VkImageDrmFormatModifierPropertiesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    drmFormatModifier: uint64_t,
  }
}

structure! {
  /// [VkImageFormatListCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageFormatListCreateInfo.html)
  ///
  /// Struct Extends: [`VkImageCreateInfo`], [`VkSwapchainCreateInfoKHR`], [`VkPhysicalDeviceImageFormatInfo2`]
  VkImageFormatListCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    viewFormatCount: uint32_t,
    /// * **Len:** viewFormatCount
    pViewFormats: *const VkFormat,
  }
}

structure! {
  /// [VkImageFormatProperties2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageFormatProperties2.html)
  VkImageFormatProperties2 {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    imageFormatProperties: VkImageFormatProperties,
  }
}

structure! {
  /// [VkImageMemoryRequirementsInfo2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageMemoryRequirementsInfo2.html)
  VkImageMemoryRequirementsInfo2 {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    image: VkImage,
  }
}

structure! {
  /// [VkImagePipeSurfaceCreateInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImagePipeSurfaceCreateInfoFUCHSIA.html)
  VkImagePipeSurfaceCreateInfoFUCHSIA {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkImagePipeSurfaceCreateFlagsFUCHSIA,
    imagePipeHandle: zx_handle_t,
  }
}

structure! {
  /// [VkImagePlaneMemoryRequirementsInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImagePlaneMemoryRequirementsInfo.html)
  ///
  /// Struct Extends: [`VkImageMemoryRequirementsInfo2`]
  VkImagePlaneMemoryRequirementsInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    planeAspect: VkImageAspectFlagBits,
  }
}

structure! {
  /// [VkImageResolve](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageResolve.html)
  VkImageResolve {
    srcSubresource: VkImageSubresourceLayers,
    srcOffset: VkOffset3D,
    dstSubresource: VkImageSubresourceLayers,
    dstOffset: VkOffset3D,
    extent: VkExtent3D,
  }
}

structure! {
  /// [VkImageResolve2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageResolve2KHR.html)
  VkImageResolve2KHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    srcSubresource: VkImageSubresourceLayers,
    srcOffset: VkOffset3D,
    dstSubresource: VkImageSubresourceLayers,
    dstOffset: VkOffset3D,
    extent: VkExtent3D,
  }
}

structure! {
  /// [VkImageSparseMemoryRequirementsInfo2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSparseMemoryRequirementsInfo2.html)
  VkImageSparseMemoryRequirementsInfo2 {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    image: VkImage,
  }
}

structure! {
  /// [VkImageStencilUsageCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageStencilUsageCreateInfo.html)
  ///
  /// Struct Extends: [`VkImageCreateInfo`], [`VkPhysicalDeviceImageFormatInfo2`]
  VkImageStencilUsageCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    stencilUsage: VkImageUsageFlags,
  }
}

structure! {
  /// [VkImageSubresource](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSubresource.html)
  VkImageSubresource {
    aspectMask: VkImageAspectFlags,
    mipLevel: uint32_t,
    arrayLayer: uint32_t,
  }
}

structure! {
  /// [VkImageSubresourceLayers](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSubresourceLayers.html)
  VkImageSubresourceLayers {
    aspectMask: VkImageAspectFlags,
    mipLevel: uint32_t,
    baseArrayLayer: uint32_t,
    layerCount: uint32_t,
  }
}

structure! {
  /// [VkImageSwapchainCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSwapchainCreateInfoKHR.html)
  ///
  /// Struct Extends: [`VkImageCreateInfo`]
  VkImageSwapchainCreateInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    swapchain: VkSwapchainKHR,
  }
}

structure! {
  /// [VkImageViewASTCDecodeModeEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewASTCDecodeModeEXT.html)
  ///
  /// Struct Extends: [`VkImageViewCreateInfo`]
  VkImageViewASTCDecodeModeEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_VIEW_ASTC_DECODE_MODE_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    decodeMode: VkFormat,
  }
}

structure! {
  /// [VkImageViewAddressPropertiesNVX](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewAddressPropertiesNVX.html)
  VkImageViewAddressPropertiesNVX {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_VIEW_ADDRESS_PROPERTIES_NVX`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    deviceAddress: VkDeviceAddress,
    size: VkDeviceSize,
  }
}

structure! {
  /// [VkImageViewCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewCreateInfo.html)
  VkImageViewCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkImageViewCreateFlags,
    image: VkImage,
    viewType: VkImageViewType,
    format: VkFormat,
    components: VkComponentMapping,
    subresourceRange: VkImageSubresourceRange,
  }
}

structure! {
  /// [VkImageViewHandleInfoNVX](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewHandleInfoNVX.html)
  VkImageViewHandleInfoNVX {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_VIEW_HANDLE_INFO_NVX`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    imageView: VkImageView,
    descriptorType: VkDescriptorType,
    /// * **Optional:** true
    sampler: VkSampler,
  }
}

structure! {
  /// [VkImageViewUsageCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewUsageCreateInfo.html)
  ///
  /// Struct Extends: [`VkImageViewCreateInfo`]
  VkImageViewUsageCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    usage: VkImageUsageFlags,
  }
}

structure! {
  /// [VkImportAndroidHardwareBufferInfoANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportAndroidHardwareBufferInfoANDROID.html)
  ///
  /// Struct Extends: [`VkMemoryAllocateInfo`]
  VkImportAndroidHardwareBufferInfoANDROID {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    buffer: *mut AHardwareBuffer,
  }
}

structure! {
  /// [VkImportFenceFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportFenceFdInfoKHR.html)
  VkImportFenceFdInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Extern Sync:** true
    fence: VkFence,
    /// * **Optional:** true
    flags: VkFenceImportFlags,
    handleType: VkExternalFenceHandleTypeFlagBits,
    fd: int,
  }
}

structure! {
  /// [VkImportFenceWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportFenceWin32HandleInfoKHR.html)
  VkImportFenceWin32HandleInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Extern Sync:** true
    fence: VkFence,
    /// * **Optional:** true
    flags: VkFenceImportFlags,
    /// * **No Auto-validity:** true
    handleType: VkExternalFenceHandleTypeFlagBits,
    /// * **Optional:** true
    handle: HANDLE,
    /// * **Optional:** true
    name: LPCWSTR,
  }
}

structure! {
  /// [VkImportMemoryFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportMemoryFdInfoKHR.html)
  ///
  /// Struct Extends: [`VkMemoryAllocateInfo`]
  VkImportMemoryFdInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    handleType: VkExternalMemoryHandleTypeFlagBits,
    fd: int,
  }
}

structure! {
  /// [VkImportMemoryHostPointerInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportMemoryHostPointerInfoEXT.html)
  ///
  /// Struct Extends: [`VkMemoryAllocateInfo`]
  VkImportMemoryHostPointerInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    /// * **Optional:** false
    pHostPointer: *mut c_void,
  }
}

structure! {
  /// [VkImportMemoryWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportMemoryWin32HandleInfoKHR.html)
  ///
  /// Struct Extends: [`VkMemoryAllocateInfo`]
  VkImportMemoryWin32HandleInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    handleType: VkExternalMemoryHandleTypeFlagBits,
    /// * **Optional:** true
    handle: HANDLE,
    /// * **Optional:** true
    name: LPCWSTR,
  }
}

structure! {
  /// [VkImportMemoryWin32HandleInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportMemoryWin32HandleInfoNV.html)
  ///
  /// Struct Extends: [`VkMemoryAllocateInfo`]
  VkImportMemoryWin32HandleInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    handleType: VkExternalMemoryHandleTypeFlagsNV,
    /// * **Optional:** true
    handle: HANDLE,
  }
}

structure! {
  /// [VkImportSemaphoreFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportSemaphoreFdInfoKHR.html)
  VkImportSemaphoreFdInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Extern Sync:** true
    semaphore: VkSemaphore,
    /// * **Optional:** true
    flags: VkSemaphoreImportFlags,
    handleType: VkExternalSemaphoreHandleTypeFlagBits,
    fd: int,
  }
}

structure! {
  /// [VkImportSemaphoreWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportSemaphoreWin32HandleInfoKHR.html)
  VkImportSemaphoreWin32HandleInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Extern Sync:** true
    semaphore: VkSemaphore,
    /// * **Optional:** true
    flags: VkSemaphoreImportFlags,
    /// * **No Auto-validity:** true
    handleType: VkExternalSemaphoreHandleTypeFlagBits,
    /// * **Optional:** true
    handle: HANDLE,
    /// * **Optional:** true
    name: LPCWSTR,
  }
}

structure! {
  /// [VkIndirectCommandsLayoutCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectCommandsLayoutCreateInfoNV.html)
  VkIndirectCommandsLayoutCreateInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    flags: VkIndirectCommandsLayoutUsageFlagsNV,
    pipelineBindPoint: VkPipelineBindPoint,
    tokenCount: uint32_t,
    /// * **Len:** tokenCount
    pTokens: *const VkIndirectCommandsLayoutTokenNV,
    streamCount: uint32_t,
    /// * **Len:** streamCount
    pStreamStrides: *const uint32_t,
  }
}

structure! {
  /// [VkIndirectCommandsLayoutTokenNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectCommandsLayoutTokenNV.html)
  VkIndirectCommandsLayoutTokenNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_TOKEN_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    tokenType: VkIndirectCommandsTokenTypeNV,
    stream: uint32_t,
    offset: uint32_t,
    vertexBindingUnit: uint32_t,
    vertexDynamicStride: VkBool32,
    /// * **Optional:** true
    pushconstantPipelineLayout: VkPipelineLayout,
    /// * **Optional:** true
    pushconstantShaderStageFlags: VkShaderStageFlags,
    pushconstantOffset: uint32_t,
    pushconstantSize: uint32_t,
    /// * **Optional:** true
    indirectStateFlags: VkIndirectStateFlagsNV,
    /// * **Optional:** true
    indexTypeCount: uint32_t,
    /// * **Len:** indexTypeCount
    pIndexTypes: *const VkIndexType,
    /// * **Len:** indexTypeCount
    pIndexTypeValues: *const uint32_t,
  }
}

structure! {
  /// [VkIndirectCommandsStreamNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectCommandsStreamNV.html)
  VkIndirectCommandsStreamNV {
    buffer: VkBuffer,
    offset: VkDeviceSize,
  }
}

structure! {
  /// [VkInitializePerformanceApiInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkInitializePerformanceApiInfoINTEL.html)
  VkInitializePerformanceApiInfoINTEL {
    /// * **Values:** [`VK_STRUCTURE_TYPE_INITIALIZE_PERFORMANCE_API_INFO_INTEL`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    pUserData: *mut c_void,
  }
}

structure! {
  /// [VkInputAttachmentAspectReference](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkInputAttachmentAspectReference.html)
  VkInputAttachmentAspectReference {
    subpass: uint32_t,
    inputAttachmentIndex: uint32_t,
    aspectMask: VkImageAspectFlags,
  }
}

structure! {
  /// [VkMacOSSurfaceCreateInfoMVK](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMacOSSurfaceCreateInfoMVK.html)
  VkMacOSSurfaceCreateInfoMVK {
    /// * **Values:** [`VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkMacOSSurfaceCreateFlagsMVK,
    /// * **No Auto-validity:** true
    pView: *const c_void,
  }
}

structure! {
  /// [VkMappedMemoryRange](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMappedMemoryRange.html)
  VkMappedMemoryRange {
    /// * **Values:** [`VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Mapped memory object
    memory: VkDeviceMemory,
    /// Offset within the memory object where the range starts
    offset: VkDeviceSize,
    /// Size of the range within the memory object
    size: VkDeviceSize,
  }
}

structure! {
  /// [VkMemoryAllocateFlagsInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryAllocateFlagsInfo.html)
  ///
  /// Struct Extends: [`VkMemoryAllocateInfo`]
  VkMemoryAllocateFlagsInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkMemoryAllocateFlags,
    deviceMask: uint32_t,
  }
}

structure! {
  /// [VkMemoryAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryAllocateInfo.html)
  VkMemoryAllocateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Size of memory allocation
    allocationSize: VkDeviceSize,
    /// Index of the of the memory type to allocate from
    memoryTypeIndex: uint32_t,
  }
}

structure! {
  /// [VkMemoryDedicatedAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryDedicatedAllocateInfo.html)
  ///
  /// Struct Extends: [`VkMemoryAllocateInfo`]
  VkMemoryDedicatedAllocateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Image that this allocation will be bound to
    /// * **Optional:** true
    image: VkImage,
    /// Buffer that this allocation will be bound to
    /// * **Optional:** true
    buffer: VkBuffer,
  }
}

structure! {
  /// [VkMemoryDedicatedRequirements](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryDedicatedRequirements.html)
  ///
  /// Struct Extends: [`VkMemoryRequirements2`]
  VkMemoryDedicatedRequirements {
    /// * **Values:** [`VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    prefersDedicatedAllocation: VkBool32,
    requiresDedicatedAllocation: VkBool32,
  }
}

structure! {
  /// [VkMemoryFdPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryFdPropertiesKHR.html)
  VkMemoryFdPropertiesKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    memoryTypeBits: uint32_t,
  }
}

structure! {
  /// [VkMemoryGetAndroidHardwareBufferInfoANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryGetAndroidHardwareBufferInfoANDROID.html)
  VkMemoryGetAndroidHardwareBufferInfoANDROID {
    /// * **Values:** [`VK_STRUCTURE_TYPE_MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    memory: VkDeviceMemory,
  }
}

structure! {
  /// [VkMemoryGetFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryGetFdInfoKHR.html)
  VkMemoryGetFdInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    memory: VkDeviceMemory,
    handleType: VkExternalMemoryHandleTypeFlagBits,
  }
}

structure! {
  /// [VkMemoryGetWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryGetWin32HandleInfoKHR.html)
  VkMemoryGetWin32HandleInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    memory: VkDeviceMemory,
    handleType: VkExternalMemoryHandleTypeFlagBits,
  }
}

structure! {
  /// [VkMemoryHostPointerPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryHostPointerPropertiesEXT.html)
  VkMemoryHostPointerPropertiesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    memoryTypeBits: uint32_t,
  }
}

structure! {
  /// [VkMemoryOpaqueCaptureAddressAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryOpaqueCaptureAddressAllocateInfo.html)
  ///
  /// Struct Extends: [`VkMemoryAllocateInfo`]
  VkMemoryOpaqueCaptureAddressAllocateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    opaqueCaptureAddress: uint64_t,
  }
}

structure! {
  /// [VkMemoryPriorityAllocateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryPriorityAllocateInfoEXT.html)
  ///
  /// Struct Extends: [`VkMemoryAllocateInfo`]
  VkMemoryPriorityAllocateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_MEMORY_PRIORITY_ALLOCATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    priority: float,
  }
}

structure! {
  /// [VkMemoryRequirements](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryRequirements.html)
  VkMemoryRequirements {
    /// Specified in bytes
    size: VkDeviceSize,
    /// Specified in bytes
    alignment: VkDeviceSize,
    /// Bitmask of the allowed memory type indices into memoryTypes[] for this object
    memoryTypeBits: uint32_t,
  }
}

structure! {
  /// [VkMemoryRequirements2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryRequirements2.html)
  VkMemoryRequirements2 {
    /// * **Values:** [`VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    memoryRequirements: VkMemoryRequirements,
  }
}
pub type VkMemoryRequirements2KHR = VkMemoryRequirements2;

structure! {
  /// [VkMemoryWin32HandlePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryWin32HandlePropertiesKHR.html)
  VkMemoryWin32HandlePropertiesKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    memoryTypeBits: uint32_t,
  }
}

structure! {
  /// [VkMetalSurfaceCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMetalSurfaceCreateInfoEXT.html)
  VkMetalSurfaceCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkMetalSurfaceCreateFlagsEXT,
    /// * **No Auto-validity:** true
    pLayer: *const CAMetalLayer,
  }
}

structure! {
  /// [VkMultisamplePropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMultisamplePropertiesEXT.html)
  VkMultisamplePropertiesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    maxSampleLocationGridSize: VkExtent2D,
  }
}

structure! {
  /// [VkMutableDescriptorTypeCreateInfoVALVE](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMutableDescriptorTypeCreateInfoVALVE.html)
  ///
  /// Struct Extends: [`VkDescriptorSetLayoutCreateInfo`], [`VkDescriptorPoolCreateInfo`]
  VkMutableDescriptorTypeCreateInfoVALVE {
    /// * **Values:** [`VK_STRUCTURE_TYPE_MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE`]
    sType: VkStructureType,
    pNext: *const c_void,
    /// * **Optional:** true
    mutableDescriptorTypeListCount: uint32_t,
    /// * **Len:** mutableDescriptorTypeListCount
    pMutableDescriptorTypeLists: *const VkMutableDescriptorTypeListVALVE,
  }
}

structure! {
  /// [VkMutableDescriptorTypeListVALVE](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMutableDescriptorTypeListVALVE.html)
  VkMutableDescriptorTypeListVALVE {
    /// * **Optional:** true
    descriptorTypeCount: uint32_t,
    /// * **Len:** descriptorTypeCount
    pDescriptorTypes: *const VkDescriptorType,
  }
}

#[cfg(feature = "VK_ANDROID_native_buffer")]
structure! {
  /// [VkNativeBufferANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkNativeBufferANDROID.html)
  VkNativeBufferANDROID {
    /// * **Values:** [`VK_STRUCTURE_TYPE_NATIVE_BUFFER_ANDROID`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    handle: *const c_void,
    stride: int,
    format: int,
    usage: int,
    usage2: VkNativeBufferUsage2ANDROID,
  }
}

structure! {
  /// [VkNativeBufferUsage2ANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkNativeBufferUsage2ANDROID.html)
  VkNativeBufferUsage2ANDROID {
    consumer: uint64_t,
    producer: uint64_t,
  }
}

structure! {
  /// [VkPastPresentationTimingGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPastPresentationTimingGOOGLE.html)
  VkPastPresentationTimingGOOGLE {
    /// Application-provided identifier, previously given to vkQueuePresentKHR
    presentID: uint32_t,
    /// Earliest time an image should have been presented, previously given to vkQueuePresentKHR
    desiredPresentTime: uint64_t,
    /// Time the image was actually displayed
    actualPresentTime: uint64_t,
    /// Earliest time the image could have been displayed
    earliestPresentTime: uint64_t,
    /// How early vkQueuePresentKHR was processed vs. how soon it needed to be and make earliestPresentTime
    presentMargin: uint64_t,
  }
}

structure! {
  /// [VkPerformanceConfigurationAcquireInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceConfigurationAcquireInfoINTEL.html)
  VkPerformanceConfigurationAcquireInfoINTEL {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    type_: VkPerformanceConfigurationTypeINTEL,
  }
}

structure! {
  /// [VkPerformanceCounterDescriptionKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterDescriptionKHR.html)
  VkPerformanceCounterDescriptionKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_DESCRIPTION_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkPerformanceCounterDescriptionFlagsKHR,
    name: [u8; VK_MAX_DESCRIPTION_SIZE as usize],
    category: [u8; VK_MAX_DESCRIPTION_SIZE as usize],
    description: [u8; VK_MAX_DESCRIPTION_SIZE as usize],
  }
}

structure! {
  /// [VkPerformanceCounterKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterKHR.html)
  VkPerformanceCounterKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    unit: VkPerformanceCounterUnitKHR,
    scope: VkPerformanceCounterScopeKHR,
    storage: VkPerformanceCounterStorageKHR,
    uuid: [uint8_t; VK_UUID_SIZE as usize],
  }
}

structure! {
  /// [VkPerformanceMarkerInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceMarkerInfoINTEL.html)
  VkPerformanceMarkerInfoINTEL {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PERFORMANCE_MARKER_INFO_INTEL`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    marker: uint64_t,
  }
}

structure! {
  /// [VkPerformanceOverrideInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceOverrideInfoINTEL.html)
  VkPerformanceOverrideInfoINTEL {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PERFORMANCE_OVERRIDE_INFO_INTEL`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    type_: VkPerformanceOverrideTypeINTEL,
    enable: VkBool32,
    parameter: uint64_t,
  }
}

structure! {
  /// [VkPerformanceQuerySubmitInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceQuerySubmitInfoKHR.html)
  ///
  /// Struct Extends: [`VkSubmitInfo`]
  VkPerformanceQuerySubmitInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PERFORMANCE_QUERY_SUBMIT_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Index for which counter pass to submit
    counterPassIndex: uint32_t,
  }
}

structure! {
  /// [VkPerformanceStreamMarkerInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceStreamMarkerInfoINTEL.html)
  VkPerformanceStreamMarkerInfoINTEL {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PERFORMANCE_STREAM_MARKER_INFO_INTEL`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    marker: uint32_t,
  }
}

structure! {
  /// [VkPerformanceValueINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceValueINTEL.html)
  VkPerformanceValueINTEL {
    type_: VkPerformanceValueTypeINTEL,
    data: VkPerformanceValueDataINTEL,
  }
}

structure! {
  /// [VkPhysicalDevice16BitStorageFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevice16BitStorageFeatures.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDevice16BitStorageFeatures {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    /// 16-bit integer/floating-point variables supported in BufferBlock
    storageBuffer16BitAccess: VkBool32,
    /// 16-bit integer/floating-point variables supported in BufferBlock and Block
    uniformAndStorageBuffer16BitAccess: VkBool32,
    /// 16-bit integer/floating-point variables supported in PushConstant
    storagePushConstant16: VkBool32,
    /// 16-bit integer/floating-point variables supported in shader inputs and outputs
    storageInputOutput16: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDevice4444FormatsFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevice4444FormatsFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDevice4444FormatsFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    formatA4R4G4B4: VkBool32,
    formatA4B4G4R4: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDevice8BitStorageFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevice8BitStorageFeatures.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDevice8BitStorageFeatures {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    /// 8-bit integer variables supported in StorageBuffer
    storageBuffer8BitAccess: VkBool32,
    /// 8-bit integer variables supported in StorageBuffer and Uniform
    uniformAndStorageBuffer8BitAccess: VkBool32,
    /// 8-bit integer variables supported in PushConstant
    storagePushConstant8: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceASTCDecodeFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceASTCDecodeFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceASTCDecodeFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    decodeModeSharedExponent: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceAccelerationStructureFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceAccelerationStructureFeaturesKHR.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceAccelerationStructureFeaturesKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    accelerationStructure: VkBool32,
    accelerationStructureCaptureReplay: VkBool32,
    accelerationStructureIndirectBuild: VkBool32,
    accelerationStructureHostCommands: VkBool32,
    descriptorBindingAccelerationStructureUpdateAfterBind: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceAccelerationStructurePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceAccelerationStructurePropertiesKHR.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceAccelerationStructurePropertiesKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    maxGeometryCount: uint64_t,
    maxInstanceCount: uint64_t,
    maxPrimitiveCount: uint64_t,
    maxPerStageDescriptorAccelerationStructures: uint32_t,
    maxPerStageDescriptorUpdateAfterBindAccelerationStructures: uint32_t,
    maxDescriptorSetAccelerationStructures: uint32_t,
    maxDescriptorSetUpdateAfterBindAccelerationStructures: uint32_t,
    minAccelerationStructureScratchOffsetAlignment: uint32_t,
  }
}

structure! {
  /// [VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    advancedBlendCoherentOperations: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    advancedBlendMaxColorAttachments: uint32_t,
    advancedBlendIndependentBlend: VkBool32,
    advancedBlendNonPremultipliedSrcColor: VkBool32,
    advancedBlendNonPremultipliedDstColor: VkBool32,
    advancedBlendCorrelatedOverlap: VkBool32,
    advancedBlendAllOperations: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceBufferDeviceAddressFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceBufferDeviceAddressFeatures.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceBufferDeviceAddressFeatures {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    bufferDeviceAddress: VkBool32,
    bufferDeviceAddressCaptureReplay: VkBool32,
    bufferDeviceAddressMultiDevice: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceBufferDeviceAddressFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceBufferDeviceAddressFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceBufferDeviceAddressFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    bufferDeviceAddress: VkBool32,
    bufferDeviceAddressCaptureReplay: VkBool32,
    bufferDeviceAddressMultiDevice: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceCoherentMemoryFeaturesAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCoherentMemoryFeaturesAMD.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceCoherentMemoryFeaturesAMD {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    deviceCoherentMemory: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceComputeShaderDerivativesFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceComputeShaderDerivativesFeaturesNV.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceComputeShaderDerivativesFeaturesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    computeDerivativeGroupQuads: VkBool32,
    computeDerivativeGroupLinear: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceConditionalRenderingFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceConditionalRenderingFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceConditionalRenderingFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    conditionalRendering: VkBool32,
    inheritedConditionalRendering: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceConservativeRasterizationPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceConservativeRasterizationPropertiesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    /// The size in pixels the primitive is enlarged at each edge during conservative rasterization
    primitiveOverestimationSize: float,
    /// The maximum additional overestimation the client can specify in the pipeline state
    maxExtraPrimitiveOverestimationSize: float,
    /// The granularity of extra overestimation sizes the implementations supports between 0 and maxExtraOverestimationSize
    extraPrimitiveOverestimationSizeGranularity: float,
    /// true if the implementation supports conservative rasterization underestimation mode
    primitiveUnderestimation: VkBool32,
    /// true if conservative rasterization also applies to points and lines
    conservativePointAndLineRasterization: VkBool32,
    /// true if degenerate triangles (those with zero area after snap) are rasterized
    degenerateTrianglesRasterized: VkBool32,
    /// true if degenerate lines (those with zero length after snap) are rasterized
    degenerateLinesRasterized: VkBool32,
    /// true if the implementation supports the FullyCoveredEXT SPIR-V builtin fragment shader input variable
    fullyCoveredFragmentShaderInputVariable: VkBool32,
    /// true if the implementation supports both conservative rasterization and post depth coverage sample coverage mask
    conservativeRasterizationPostDepthCoverage: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceCooperativeMatrixFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCooperativeMatrixFeaturesNV.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceCooperativeMatrixFeaturesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    cooperativeMatrix: VkBool32,
    cooperativeMatrixRobustBufferAccess: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceCooperativeMatrixPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCooperativeMatrixPropertiesNV.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceCooperativeMatrixPropertiesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    cooperativeMatrixSupportedStages: VkShaderStageFlags,
  }
}

structure! {
  /// [VkPhysicalDeviceCornerSampledImageFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCornerSampledImageFeaturesNV.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceCornerSampledImageFeaturesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    cornerSampledImage: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceCoverageReductionModeFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCoverageReductionModeFeaturesNV.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceCoverageReductionModeFeaturesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    coverageReductionMode: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceCustomBorderColorFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCustomBorderColorFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceCustomBorderColorFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    customBorderColors: VkBool32,
    customBorderColorWithoutFormat: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceCustomBorderColorPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCustomBorderColorPropertiesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceCustomBorderColorPropertiesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    maxCustomBorderColorSamplers: uint32_t,
  }
}

structure! {
  /// [VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    dedicatedAllocationImageAliasing: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceDepthClipEnableFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDepthClipEnableFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceDepthClipEnableFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    depthClipEnable: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceDepthStencilResolveProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDepthStencilResolveProperties.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceDepthStencilResolveProperties {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    /// supported depth resolve modes
    supportedDepthResolveModes: VkResolveModeFlags,
    /// supported stencil resolve modes
    supportedStencilResolveModes: VkResolveModeFlags,
    /// depth and stencil resolve modes can be set independently if one of them is none
    independentResolveNone: VkBool32,
    /// depth and stencil resolve modes can be set independently
    independentResolve: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceDescriptorIndexingFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDescriptorIndexingFeatures.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceDescriptorIndexingFeatures {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    shaderInputAttachmentArrayDynamicIndexing: VkBool32,
    shaderUniformTexelBufferArrayDynamicIndexing: VkBool32,
    shaderStorageTexelBufferArrayDynamicIndexing: VkBool32,
    shaderUniformBufferArrayNonUniformIndexing: VkBool32,
    shaderSampledImageArrayNonUniformIndexing: VkBool32,
    shaderStorageBufferArrayNonUniformIndexing: VkBool32,
    shaderStorageImageArrayNonUniformIndexing: VkBool32,
    shaderInputAttachmentArrayNonUniformIndexing: VkBool32,
    shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32,
    shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32,
    descriptorBindingUniformBufferUpdateAfterBind: VkBool32,
    descriptorBindingSampledImageUpdateAfterBind: VkBool32,
    descriptorBindingStorageImageUpdateAfterBind: VkBool32,
    descriptorBindingStorageBufferUpdateAfterBind: VkBool32,
    descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32,
    descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32,
    descriptorBindingUpdateUnusedWhilePending: VkBool32,
    descriptorBindingPartiallyBound: VkBool32,
    descriptorBindingVariableDescriptorCount: VkBool32,
    runtimeDescriptorArray: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceDescriptorIndexingProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDescriptorIndexingProperties.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceDescriptorIndexingProperties {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    maxUpdateAfterBindDescriptorsInAllPools: uint32_t,
    shaderUniformBufferArrayNonUniformIndexingNative: VkBool32,
    shaderSampledImageArrayNonUniformIndexingNative: VkBool32,
    shaderStorageBufferArrayNonUniformIndexingNative: VkBool32,
    shaderStorageImageArrayNonUniformIndexingNative: VkBool32,
    shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32,
    robustBufferAccessUpdateAfterBind: VkBool32,
    quadDivergentImplicitLod: VkBool32,
    maxPerStageDescriptorUpdateAfterBindSamplers: uint32_t,
    maxPerStageDescriptorUpdateAfterBindUniformBuffers: uint32_t,
    maxPerStageDescriptorUpdateAfterBindStorageBuffers: uint32_t,
    maxPerStageDescriptorUpdateAfterBindSampledImages: uint32_t,
    maxPerStageDescriptorUpdateAfterBindStorageImages: uint32_t,
    maxPerStageDescriptorUpdateAfterBindInputAttachments: uint32_t,
    maxPerStageUpdateAfterBindResources: uint32_t,
    maxDescriptorSetUpdateAfterBindSamplers: uint32_t,
    maxDescriptorSetUpdateAfterBindUniformBuffers: uint32_t,
    maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: uint32_t,
    maxDescriptorSetUpdateAfterBindStorageBuffers: uint32_t,
    maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: uint32_t,
    maxDescriptorSetUpdateAfterBindSampledImages: uint32_t,
    maxDescriptorSetUpdateAfterBindStorageImages: uint32_t,
    maxDescriptorSetUpdateAfterBindInputAttachments: uint32_t,
  }
}

structure! {
  /// [VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    deviceGeneratedCommands: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    maxGraphicsShaderGroupCount: uint32_t,
    maxIndirectSequenceCount: uint32_t,
    maxIndirectCommandsTokenCount: uint32_t,
    maxIndirectCommandsStreamCount: uint32_t,
    maxIndirectCommandsTokenOffset: uint32_t,
    maxIndirectCommandsStreamStride: uint32_t,
    minSequencesCountBufferOffsetAlignment: uint32_t,
    minSequencesIndexBufferOffsetAlignment: uint32_t,
    minIndirectCommandsBufferOffsetAlignment: uint32_t,
  }
}

structure! {
  /// [VkPhysicalDeviceDeviceMemoryReportFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDeviceMemoryReportFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceDeviceMemoryReportFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    deviceMemoryReport: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceDiagnosticsConfigFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDiagnosticsConfigFeaturesNV.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceDiagnosticsConfigFeaturesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    diagnosticsConfig: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceDiscardRectanglePropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDiscardRectanglePropertiesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceDiscardRectanglePropertiesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    /// max number of active discard rectangles
    maxDiscardRectangles: uint32_t,
  }
}

structure! {
  /// [VkPhysicalDeviceDriverProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDriverProperties.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceDriverProperties {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    driverID: VkDriverId,
    driverName: [u8; VK_MAX_DRIVER_NAME_SIZE as usize],
    driverInfo: [u8; VK_MAX_DRIVER_INFO_SIZE as usize],
    conformanceVersion: VkConformanceVersion,
  }
}

structure! {
  /// [VkPhysicalDeviceExclusiveScissorFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExclusiveScissorFeaturesNV.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceExclusiveScissorFeaturesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    exclusiveScissor: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceExtendedDynamicStateFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExtendedDynamicStateFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceExtendedDynamicStateFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    extendedDynamicState: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceExternalBufferInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalBufferInfo.html)
  VkPhysicalDeviceExternalBufferInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkBufferCreateFlags,
    usage: VkBufferUsageFlags,
    handleType: VkExternalMemoryHandleTypeFlagBits,
  }
}

structure! {
  /// [VkPhysicalDeviceExternalFenceInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalFenceInfo.html)
  VkPhysicalDeviceExternalFenceInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    handleType: VkExternalFenceHandleTypeFlagBits,
  }
}

structure! {
  /// [VkPhysicalDeviceExternalImageFormatInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalImageFormatInfo.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceImageFormatInfo2`]
  VkPhysicalDeviceExternalImageFormatInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    handleType: VkExternalMemoryHandleTypeFlagBits,
  }
}

structure! {
  /// [VkPhysicalDeviceExternalMemoryHostPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalMemoryHostPropertiesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    minImportedHostPointerAlignment: VkDeviceSize,
  }
}

structure! {
  /// [VkPhysicalDeviceExternalSemaphoreInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExternalSemaphoreInfo.html)
  VkPhysicalDeviceExternalSemaphoreInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    handleType: VkExternalSemaphoreHandleTypeFlagBits,
  }
}

structure! {
  /// [VkPhysicalDeviceFeatures2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFeatures2.html)
  ///
  /// Struct Extends: [`VkDeviceCreateInfo`]
  VkPhysicalDeviceFeatures2 {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    features: VkPhysicalDeviceFeatures,
  }
}

structure! {
  /// [VkPhysicalDeviceFloatControlsProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFloatControlsProperties.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceFloatControlsProperties {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    denormBehaviorIndependence: VkShaderFloatControlsIndependence,
    roundingModeIndependence: VkShaderFloatControlsIndependence,
    /// An implementation can preserve signed zero, nan, inf
    shaderSignedZeroInfNanPreserveFloat16: VkBool32,
    /// An implementation can preserve signed zero, nan, inf
    shaderSignedZeroInfNanPreserveFloat32: VkBool32,
    /// An implementation can preserve signed zero, nan, inf
    shaderSignedZeroInfNanPreserveFloat64: VkBool32,
    /// An implementation can preserve  denormals
    shaderDenormPreserveFloat16: VkBool32,
    /// An implementation can preserve  denormals
    shaderDenormPreserveFloat32: VkBool32,
    /// An implementation can preserve  denormals
    shaderDenormPreserveFloat64: VkBool32,
    /// An implementation can flush to zero  denormals
    shaderDenormFlushToZeroFloat16: VkBool32,
    /// An implementation can flush to zero  denormals
    shaderDenormFlushToZeroFloat32: VkBool32,
    /// An implementation can flush to zero  denormals
    shaderDenormFlushToZeroFloat64: VkBool32,
    /// An implementation can support RTE
    shaderRoundingModeRTEFloat16: VkBool32,
    /// An implementation can support RTE
    shaderRoundingModeRTEFloat32: VkBool32,
    /// An implementation can support RTE
    shaderRoundingModeRTEFloat64: VkBool32,
    /// An implementation can support RTZ
    shaderRoundingModeRTZFloat16: VkBool32,
    /// An implementation can support RTZ
    shaderRoundingModeRTZFloat32: VkBool32,
    /// An implementation can support RTZ
    shaderRoundingModeRTZFloat64: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceFragmentDensityMap2FeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentDensityMap2FeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceFragmentDensityMap2FeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    fragmentDensityMapDeferred: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceFragmentDensityMap2PropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentDensityMap2PropertiesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceFragmentDensityMap2PropertiesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    subsampledLoads: VkBool32,
    subsampledCoarseReconstructionEarlyAccess: VkBool32,
    maxSubsampledArrayLayers: uint32_t,
    maxDescriptorSetSubsampledSamplers: uint32_t,
  }
}

structure! {
  /// [VkPhysicalDeviceFragmentDensityMapFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentDensityMapFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceFragmentDensityMapFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    fragmentDensityMap: VkBool32,
    fragmentDensityMapDynamic: VkBool32,
    fragmentDensityMapNonSubsampledImages: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceFragmentDensityMapPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentDensityMapPropertiesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceFragmentDensityMapPropertiesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    minFragmentDensityTexelSize: VkExtent2D,
    maxFragmentDensityTexelSize: VkExtent2D,
    fragmentDensityInvocations: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    fragmentShaderBarycentric: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT`]
    sType: VkStructureType,
    /// Pointer to next structure
    /// * **Optional:** true
    pNext: *mut c_void,
    fragmentShaderSampleInterlock: VkBool32,
    fragmentShaderPixelInterlock: VkBool32,
    fragmentShaderShadingRateInterlock: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    fragmentShadingRateEnums: VkBool32,
    supersampleFragmentShadingRates: VkBool32,
    noInvocationFragmentShadingRates: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    maxFragmentShadingRateInvocationCount: VkSampleCountFlagBits,
  }
}

structure! {
  /// [VkPhysicalDeviceFragmentShadingRateFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShadingRateFeaturesKHR.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceFragmentShadingRateFeaturesKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    pipelineFragmentShadingRate: VkBool32,
    primitiveFragmentShadingRate: VkBool32,
    attachmentFragmentShadingRate: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceFragmentShadingRateKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShadingRateKHR.html)
  VkPhysicalDeviceFragmentShadingRateKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    sampleCounts: VkSampleCountFlags,
    fragmentSize: VkExtent2D,
  }
}

structure! {
  /// [VkPhysicalDeviceFragmentShadingRatePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentShadingRatePropertiesKHR.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceFragmentShadingRatePropertiesKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    minFragmentShadingRateAttachmentTexelSize: VkExtent2D,
    maxFragmentShadingRateAttachmentTexelSize: VkExtent2D,
    maxFragmentShadingRateAttachmentTexelSizeAspectRatio: uint32_t,
    primitiveFragmentShadingRateWithMultipleViewports: VkBool32,
    layeredShadingRateAttachments: VkBool32,
    fragmentShadingRateNonTrivialCombinerOps: VkBool32,
    maxFragmentSize: VkExtent2D,
    maxFragmentSizeAspectRatio: uint32_t,
    maxFragmentShadingRateCoverageSamples: uint32_t,
    maxFragmentShadingRateRasterizationSamples: VkSampleCountFlagBits,
    fragmentShadingRateWithShaderDepthStencilWrites: VkBool32,
    fragmentShadingRateWithSampleMask: VkBool32,
    fragmentShadingRateWithShaderSampleMask: VkBool32,
    fragmentShadingRateWithConservativeRasterization: VkBool32,
    fragmentShadingRateWithFragmentShaderInterlock: VkBool32,
    fragmentShadingRateWithCustomSampleLocations: VkBool32,
    fragmentShadingRateStrictMultiplyCombiner: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceGroupProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceGroupProperties.html)
  VkPhysicalDeviceGroupProperties {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    physicalDeviceCount: uint32_t,
    physicalDevices: [VkPhysicalDevice; VK_MAX_DEVICE_GROUP_SIZE as usize],
    subsetAllocation: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceHostQueryResetFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceHostQueryResetFeatures.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceHostQueryResetFeatures {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    hostQueryReset: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceIDProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceIDProperties.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceIDProperties {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    deviceUUID: [uint8_t; VK_UUID_SIZE as usize],
    driverUUID: [uint8_t; VK_UUID_SIZE as usize],
    deviceLUID: [uint8_t; VK_LUID_SIZE as usize],
    deviceNodeMask: uint32_t,
    deviceLUIDValid: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceImageDrmFormatModifierInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceImageDrmFormatModifierInfoEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceImageFormatInfo2`]
  VkPhysicalDeviceImageDrmFormatModifierInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    drmFormatModifier: uint64_t,
    sharingMode: VkSharingMode,
    /// * **Optional:** true
    queueFamilyIndexCount: uint32_t,
    /// * **No Auto-validity:** true
    /// * **Len:** queueFamilyIndexCount
    pQueueFamilyIndices: *const uint32_t,
  }
}

structure! {
  /// [VkPhysicalDeviceImageFormatInfo2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceImageFormatInfo2.html)
  VkPhysicalDeviceImageFormatInfo2 {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    format: VkFormat,
    type_: VkImageType,
    tiling: VkImageTiling,
    usage: VkImageUsageFlags,
    /// * **Optional:** true
    flags: VkImageCreateFlags,
  }
}

structure! {
  /// [VkPhysicalDeviceImageRobustnessFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceImageRobustnessFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceImageRobustnessFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    robustImageAccess: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceImageViewImageFormatInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceImageViewImageFormatInfoEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceImageFormatInfo2`]
  VkPhysicalDeviceImageViewImageFormatInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    imageViewType: VkImageViewType,
  }
}

structure! {
  /// [VkPhysicalDeviceImagelessFramebufferFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceImagelessFramebufferFeatures.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceImagelessFramebufferFeatures {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    imagelessFramebuffer: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceIndexTypeUint8FeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceIndexTypeUint8FeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceIndexTypeUint8FeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    indexTypeUint8: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceInlineUniformBlockFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceInlineUniformBlockFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceInlineUniformBlockFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    inlineUniformBlock: VkBool32,
    descriptorBindingInlineUniformBlockUpdateAfterBind: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceInlineUniformBlockPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceInlineUniformBlockPropertiesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceInlineUniformBlockPropertiesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    maxInlineUniformBlockSize: uint32_t,
    maxPerStageDescriptorInlineUniformBlocks: uint32_t,
    maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks: uint32_t,
    maxDescriptorSetInlineUniformBlocks: uint32_t,
    maxDescriptorSetUpdateAfterBindInlineUniformBlocks: uint32_t,
  }
}

structure! {
  /// [VkPhysicalDeviceLineRasterizationFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceLineRasterizationFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceLineRasterizationFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    rectangularLines: VkBool32,
    bresenhamLines: VkBool32,
    smoothLines: VkBool32,
    stippledRectangularLines: VkBool32,
    stippledBresenhamLines: VkBool32,
    stippledSmoothLines: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceLineRasterizationPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceLineRasterizationPropertiesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceLineRasterizationPropertiesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    lineSubPixelPrecisionBits: uint32_t,
  }
}

structure! {
  /// [VkPhysicalDeviceMaintenance3Properties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMaintenance3Properties.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceMaintenance3Properties {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    maxPerSetDescriptors: uint32_t,
    maxMemoryAllocationSize: VkDeviceSize,
  }
}

structure! {
  /// [VkPhysicalDeviceMemoryBudgetPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMemoryBudgetPropertiesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceMemoryProperties2`]
  VkPhysicalDeviceMemoryBudgetPropertiesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    heapBudget: [VkDeviceSize; VK_MAX_MEMORY_HEAPS as usize],
    heapUsage: [VkDeviceSize; VK_MAX_MEMORY_HEAPS as usize],
  }
}

structure! {
  /// [VkPhysicalDeviceMemoryPriorityFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMemoryPriorityFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceMemoryPriorityFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    memoryPriority: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceMemoryProperties2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMemoryProperties2.html)
  VkPhysicalDeviceMemoryProperties2 {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    memoryProperties: VkPhysicalDeviceMemoryProperties,
  }
}

structure! {
  /// [VkPhysicalDeviceMeshShaderFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMeshShaderFeaturesNV.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceMeshShaderFeaturesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    taskShader: VkBool32,
    meshShader: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceMeshShaderPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMeshShaderPropertiesNV.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceMeshShaderPropertiesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    maxDrawMeshTasksCount: uint32_t,
    maxTaskWorkGroupInvocations: uint32_t,
    maxTaskWorkGroupSize: [uint32_t; 3],
    maxTaskTotalMemorySize: uint32_t,
    maxTaskOutputCount: uint32_t,
    maxMeshWorkGroupInvocations: uint32_t,
    maxMeshWorkGroupSize: [uint32_t; 3],
    maxMeshTotalMemorySize: uint32_t,
    maxMeshOutputVertices: uint32_t,
    maxMeshOutputPrimitives: uint32_t,
    maxMeshMultiviewViewCount: uint32_t,
    meshOutputPerVertexGranularity: uint32_t,
    meshOutputPerPrimitiveGranularity: uint32_t,
  }
}

structure! {
  /// [VkPhysicalDeviceMultiviewFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMultiviewFeatures.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceMultiviewFeatures {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    /// Multiple views in a renderpass
    multiview: VkBool32,
    /// Multiple views in a renderpass w/ geometry shader
    multiviewGeometryShader: VkBool32,
    /// Multiple views in a renderpass w/ tessellation shader
    multiviewTessellationShader: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    perViewPositionAllComponents: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceMultiviewProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMultiviewProperties.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceMultiviewProperties {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    /// max number of views in a subpass
    maxMultiviewViewCount: uint32_t,
    /// max instance index for a draw in a multiview subpass
    maxMultiviewInstanceIndex: uint32_t,
  }
}

structure! {
  /// [VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE`]
    sType: VkStructureType,
    /// * **No Auto-validity:** true
    pNext: *mut c_void,
    mutableDescriptorType: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDevicePCIBusInfoPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePCIBusInfoPropertiesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDevicePCIBusInfoPropertiesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    pciDomain: uint32_t,
    pciBus: uint32_t,
    pciDevice: uint32_t,
    pciFunction: uint32_t,
  }
}

structure! {
  /// [VkPhysicalDevicePerformanceQueryFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePerformanceQueryFeaturesKHR.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDevicePerformanceQueryFeaturesKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    /// performance counters supported in query pools
    performanceCounterQueryPools: VkBool32,
    /// performance counters from multiple query pools can be accessed in the same primary command buffer
    performanceCounterMultipleQueryPools: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDevicePerformanceQueryPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePerformanceQueryPropertiesKHR.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDevicePerformanceQueryPropertiesKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    /// Flag to specify whether performance queries are allowed to be used in vkCmdCopyQueryPoolResults
    /// * **No Auto-validity:** true
    allowCommandBufferQueryCopies: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDevicePipelineCreationCacheControlFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePipelineCreationCacheControlFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDevicePipelineCreationCacheControlFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    pipelineCreationCacheControl: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    pipelineExecutableInfo: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDevicePointClippingProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePointClippingProperties.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDevicePointClippingProperties {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    pointClippingBehavior: VkPointClippingBehavior,
  }
}

structure! {
  /// [VkPhysicalDevicePortabilitySubsetFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePortabilitySubsetFeaturesKHR.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDevicePortabilitySubsetFeaturesKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    constantAlphaColorBlendFactors: VkBool32,
    events: VkBool32,
    imageViewFormatReinterpretation: VkBool32,
    imageViewFormatSwizzle: VkBool32,
    imageView2DOn3DImage: VkBool32,
    multisampleArrayImage: VkBool32,
    mutableComparisonSamplers: VkBool32,
    pointPolygons: VkBool32,
    samplerMipLodBias: VkBool32,
    separateStencilMaskRef: VkBool32,
    shaderSampleRateInterpolationFunctions: VkBool32,
    tessellationIsolines: VkBool32,
    tessellationPointMode: VkBool32,
    triangleFans: VkBool32,
    vertexAttributeAccessBeyondStride: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDevicePortabilitySubsetPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePortabilitySubsetPropertiesKHR.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDevicePortabilitySubsetPropertiesKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    minVertexInputBindingStrideAlignment: uint32_t,
  }
}

#[cfg(feature = "VK_ANDROID_native_buffer")]
structure! {
  /// [VkPhysicalDevicePresentationPropertiesANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePresentationPropertiesANDROID.html)
  VkPhysicalDevicePresentationPropertiesANDROID {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENTATION_PROPERTIES_ANDROID`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    sharedImage: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDevicePrivateDataFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePrivateDataFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDevicePrivateDataFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    privateData: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceProperties2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProperties2.html)
  VkPhysicalDeviceProperties2 {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    properties: VkPhysicalDeviceProperties,
  }
}

structure! {
  /// [VkPhysicalDeviceProtectedMemoryFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProtectedMemoryFeatures.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceProtectedMemoryFeatures {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    protectedMemory: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceProtectedMemoryProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProtectedMemoryProperties.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceProtectedMemoryProperties {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    protectedNoFault: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDevicePushDescriptorPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePushDescriptorPropertiesKHR.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDevicePushDescriptorPropertiesKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    maxPushDescriptors: uint32_t,
  }
}

structure! {
  /// [VkPhysicalDeviceRayQueryFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRayQueryFeaturesKHR.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceRayQueryFeaturesKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    rayQuery: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceRayTracingPipelineFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRayTracingPipelineFeaturesKHR.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceRayTracingPipelineFeaturesKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    rayTracingPipeline: VkBool32,
    rayTracingPipelineShaderGroupHandleCaptureReplay: VkBool32,
    rayTracingPipelineShaderGroupHandleCaptureReplayMixed: VkBool32,
    rayTracingPipelineTraceRaysIndirect: VkBool32,
    rayTraversalPrimitiveCulling: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceRayTracingPipelinePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRayTracingPipelinePropertiesKHR.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceRayTracingPipelinePropertiesKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    shaderGroupHandleSize: uint32_t,
    maxRayRecursionDepth: uint32_t,
    maxShaderGroupStride: uint32_t,
    shaderGroupBaseAlignment: uint32_t,
    shaderGroupHandleCaptureReplaySize: uint32_t,
    maxRayDispatchInvocationCount: uint32_t,
    shaderGroupHandleAlignment: uint32_t,
    maxRayHitAttributeSize: uint32_t,
  }
}

structure! {
  /// [VkPhysicalDeviceRayTracingPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRayTracingPropertiesNV.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceRayTracingPropertiesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    shaderGroupHandleSize: uint32_t,
    maxRecursionDepth: uint32_t,
    maxShaderGroupStride: uint32_t,
    shaderGroupBaseAlignment: uint32_t,
    maxGeometryCount: uint64_t,
    maxInstanceCount: uint64_t,
    maxTriangleCount: uint64_t,
    maxDescriptorSetAccelerationStructures: uint32_t,
  }
}

structure! {
  /// [VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    representativeFragmentTest: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceRobustness2FeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRobustness2FeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceRobustness2FeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    robustBufferAccess2: VkBool32,
    robustImageAccess2: VkBool32,
    nullDescriptor: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceRobustness2PropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRobustness2PropertiesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceRobustness2PropertiesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    robustStorageBufferAccessSizeAlignment: VkDeviceSize,
    robustUniformBufferAccessSizeAlignment: VkDeviceSize,
  }
}

structure! {
  /// [VkPhysicalDeviceSampleLocationsPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSampleLocationsPropertiesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceSampleLocationsPropertiesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    sampleLocationSampleCounts: VkSampleCountFlags,
    maxSampleLocationGridSize: VkExtent2D,
    sampleLocationCoordinateRange: [float; 2],
    sampleLocationSubPixelBits: uint32_t,
    variableSampleLocations: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceSamplerFilterMinmaxProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSamplerFilterMinmaxProperties.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceSamplerFilterMinmaxProperties {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    filterMinmaxSingleComponentFormats: VkBool32,
    filterMinmaxImageComponentMapping: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceSamplerYcbcrConversionFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSamplerYcbcrConversionFeatures.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceSamplerYcbcrConversionFeatures {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    /// Sampler color conversion supported
    samplerYcbcrConversion: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceScalarBlockLayoutFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceScalarBlockLayoutFeatures.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceScalarBlockLayoutFeatures {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    scalarBlockLayout: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    separateDepthStencilLayouts: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceShaderAtomicFloatFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderAtomicFloatFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceShaderAtomicFloatFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    shaderBufferFloat32Atomics: VkBool32,
    shaderBufferFloat32AtomicAdd: VkBool32,
    shaderBufferFloat64Atomics: VkBool32,
    shaderBufferFloat64AtomicAdd: VkBool32,
    shaderSharedFloat32Atomics: VkBool32,
    shaderSharedFloat32AtomicAdd: VkBool32,
    shaderSharedFloat64Atomics: VkBool32,
    shaderSharedFloat64AtomicAdd: VkBool32,
    shaderImageFloat32Atomics: VkBool32,
    shaderImageFloat32AtomicAdd: VkBool32,
    sparseImageFloat32Atomics: VkBool32,
    sparseImageFloat32AtomicAdd: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceShaderAtomicInt64Features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderAtomicInt64Features.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceShaderAtomicInt64Features {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    shaderBufferInt64Atomics: VkBool32,
    shaderSharedInt64Atomics: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceShaderClockFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderClockFeaturesKHR.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceShaderClockFeaturesKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    shaderSubgroupClock: VkBool32,
    shaderDeviceClock: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceShaderCoreProperties2AMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderCoreProperties2AMD.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceShaderCoreProperties2AMD {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD`]
    sType: VkStructureType,
    /// Pointer to next structure
    /// * **Optional:** true
    pNext: *mut c_void,
    /// features supported by the shader core
    shaderCoreFeatures: VkShaderCorePropertiesFlagsAMD,
    /// number of active compute units across all shader engines/arrays
    activeComputeUnitCount: uint32_t,
  }
}

structure! {
  /// [VkPhysicalDeviceShaderCorePropertiesAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderCorePropertiesAMD.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceShaderCorePropertiesAMD {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    /// number of shader engines
    shaderEngineCount: uint32_t,
    /// number of shader arrays
    shaderArraysPerEngineCount: uint32_t,
    /// number of physical CUs per shader array
    computeUnitsPerShaderArray: uint32_t,
    /// number of SIMDs per compute unit
    simdPerComputeUnit: uint32_t,
    /// number of wavefront slots in each SIMD
    wavefrontsPerSimd: uint32_t,
    /// maximum number of threads per wavefront
    wavefrontSize: uint32_t,
    /// number of physical SGPRs per SIMD
    sgprsPerSimd: uint32_t,
    /// minimum number of SGPRs that can be allocated by a wave
    minSgprAllocation: uint32_t,
    /// number of available SGPRs
    maxSgprAllocation: uint32_t,
    /// SGPRs are allocated in groups of this size
    sgprAllocationGranularity: uint32_t,
    /// number of physical VGPRs per SIMD
    vgprsPerSimd: uint32_t,
    /// minimum number of VGPRs that can be allocated by a wave
    minVgprAllocation: uint32_t,
    /// number of available VGPRs
    maxVgprAllocation: uint32_t,
    /// VGPRs are allocated in groups of this size
    vgprAllocationGranularity: uint32_t,
  }
}

structure! {
  /// [VkPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    shaderDemoteToHelperInvocation: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceShaderDrawParametersFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderDrawParametersFeatures.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceShaderDrawParametersFeatures {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    shaderDrawParameters: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceShaderFloat16Int8Features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderFloat16Int8Features.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceShaderFloat16Int8Features {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    /// 16-bit floats (halfs) in shaders
    shaderFloat16: VkBool32,
    /// 8-bit integers in shaders
    shaderInt8: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    shaderImageInt64Atomics: VkBool32,
    sparseImageInt64Atomics: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceShaderImageFootprintFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderImageFootprintFeaturesNV.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceShaderImageFootprintFeaturesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    imageFootprint: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    shaderIntegerFunctions2: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceShaderSMBuiltinsFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderSMBuiltinsFeaturesNV.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceShaderSMBuiltinsFeaturesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    shaderSMBuiltins: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceShaderSMBuiltinsPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderSMBuiltinsPropertiesNV.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceShaderSMBuiltinsPropertiesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    shaderSMCount: uint32_t,
    shaderWarpsPerSM: uint32_t,
  }
}

structure! {
  /// [VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    /// Flag to specify whether subgroup operations with extended types are supported
    /// * **No Auto-validity:** true
    shaderSubgroupExtendedTypes: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceShaderTerminateInvocationFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShaderTerminateInvocationFeaturesKHR.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceShaderTerminateInvocationFeaturesKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    pNext: *mut c_void,
    shaderTerminateInvocation: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceShadingRateImageFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShadingRateImageFeaturesNV.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceShadingRateImageFeaturesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    shadingRateImage: VkBool32,
    shadingRateCoarseSampleOrder: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceShadingRateImagePropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceShadingRateImagePropertiesNV.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceShadingRateImagePropertiesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    shadingRateTexelSize: VkExtent2D,
    shadingRatePaletteSize: uint32_t,
    shadingRateMaxCoarseSamples: uint32_t,
  }
}

structure! {
  /// [VkPhysicalDeviceSparseImageFormatInfo2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSparseImageFormatInfo2.html)
  VkPhysicalDeviceSparseImageFormatInfo2 {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    format: VkFormat,
    type_: VkImageType,
    samples: VkSampleCountFlagBits,
    usage: VkImageUsageFlags,
    tiling: VkImageTiling,
  }
}

structure! {
  /// [VkPhysicalDeviceSubgroupProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSubgroupProperties.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceSubgroupProperties {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    /// The size of a subgroup for this queue.
    /// * **No Auto-validity:** true
    subgroupSize: uint32_t,
    /// Bitfield of what shader stages support subgroup operations
    /// * **No Auto-validity:** true
    supportedStages: VkShaderStageFlags,
    /// Bitfield of what subgroup operations are supported.
    /// * **No Auto-validity:** true
    supportedOperations: VkSubgroupFeatureFlags,
    /// Flag to specify whether quad operations are available in all stages.
    /// * **No Auto-validity:** true
    quadOperationsInAllStages: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceSubgroupSizeControlFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSubgroupSizeControlFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceSubgroupSizeControlFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    subgroupSizeControl: VkBool32,
    computeFullSubgroups: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceSubgroupSizeControlPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSubgroupSizeControlPropertiesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceSubgroupSizeControlPropertiesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    /// The minimum subgroup size supported by this device
    /// * **No Auto-validity:** true
    minSubgroupSize: uint32_t,
    /// The maximum subgroup size supported by this device
    /// * **No Auto-validity:** true
    maxSubgroupSize: uint32_t,
    /// The maximum number of subgroups supported in a workgroup
    /// * **No Auto-validity:** true
    maxComputeWorkgroupSubgroups: uint32_t,
    /// The shader stages that support specifying a subgroup size
    requiredSubgroupSizeStages: VkShaderStageFlags,
  }
}

structure! {
  /// [VkPhysicalDeviceSurfaceInfo2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSurfaceInfo2KHR.html)
  VkPhysicalDeviceSurfaceInfo2KHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    surface: VkSurfaceKHR,
  }
}

structure! {
  /// [VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    texelBufferAlignment: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    storageTexelBufferOffsetAlignmentBytes: VkDeviceSize,
    storageTexelBufferOffsetSingleTexelAlignment: VkBool32,
    uniformTexelBufferOffsetAlignmentBytes: VkDeviceSize,
    uniformTexelBufferOffsetSingleTexelAlignment: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    textureCompressionASTC_HDR: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceTimelineSemaphoreFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTimelineSemaphoreFeatures.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceTimelineSemaphoreFeatures {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    timelineSemaphore: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceTimelineSemaphoreProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTimelineSemaphoreProperties.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceTimelineSemaphoreProperties {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    maxTimelineSemaphoreValueDifference: uint64_t,
  }
}

structure! {
  /// [VkPhysicalDeviceToolPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceToolPropertiesEXT.html)
  VkPhysicalDeviceToolPropertiesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    name: [u8; VK_MAX_EXTENSION_NAME_SIZE as usize],
    version: [u8; VK_MAX_EXTENSION_NAME_SIZE as usize],
    purposes: VkToolPurposeFlagsEXT,
    description: [u8; VK_MAX_DESCRIPTION_SIZE as usize],
    layer: [u8; VK_MAX_EXTENSION_NAME_SIZE as usize],
  }
}

structure! {
  /// [VkPhysicalDeviceTransformFeedbackFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTransformFeedbackFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceTransformFeedbackFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    transformFeedback: VkBool32,
    geometryStreams: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceTransformFeedbackPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceTransformFeedbackPropertiesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceTransformFeedbackPropertiesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    maxTransformFeedbackStreams: uint32_t,
    maxTransformFeedbackBuffers: uint32_t,
    maxTransformFeedbackBufferSize: VkDeviceSize,
    maxTransformFeedbackStreamDataSize: uint32_t,
    maxTransformFeedbackBufferDataSize: uint32_t,
    maxTransformFeedbackBufferDataStride: uint32_t,
    transformFeedbackQueries: VkBool32,
    transformFeedbackStreamsLinesTriangles: VkBool32,
    transformFeedbackRasterizationStreamSelect: VkBool32,
    transformFeedbackDraw: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceUniformBufferStandardLayoutFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceUniformBufferStandardLayoutFeatures.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceUniformBufferStandardLayoutFeatures {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    uniformBufferStandardLayout: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceVariablePointersFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVariablePointersFeatures.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceVariablePointersFeatures {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    variablePointersStorageBuffer: VkBool32,
    variablePointers: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    vertexAttributeInstanceRateDivisor: VkBool32,
    vertexAttributeInstanceRateZeroDivisor: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    /// max value of vertex attribute divisor
    maxVertexAttribDivisor: uint32_t,
  }
}

structure! {
  /// [VkPhysicalDeviceVulkan11Features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVulkan11Features.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceVulkan11Features {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    /// 16-bit integer/floating-point variables supported in BufferBlock
    storageBuffer16BitAccess: VkBool32,
    /// 16-bit integer/floating-point variables supported in BufferBlock and Block
    uniformAndStorageBuffer16BitAccess: VkBool32,
    /// 16-bit integer/floating-point variables supported in PushConstant
    storagePushConstant16: VkBool32,
    /// 16-bit integer/floating-point variables supported in shader inputs and outputs
    storageInputOutput16: VkBool32,
    /// Multiple views in a renderpass
    multiview: VkBool32,
    /// Multiple views in a renderpass w/ geometry shader
    multiviewGeometryShader: VkBool32,
    /// Multiple views in a renderpass w/ tessellation shader
    multiviewTessellationShader: VkBool32,
    variablePointersStorageBuffer: VkBool32,
    variablePointers: VkBool32,
    protectedMemory: VkBool32,
    /// Sampler color conversion supported
    samplerYcbcrConversion: VkBool32,
    shaderDrawParameters: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceVulkan11Properties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVulkan11Properties.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceVulkan11Properties {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    deviceUUID: [uint8_t; VK_UUID_SIZE as usize],
    driverUUID: [uint8_t; VK_UUID_SIZE as usize],
    deviceLUID: [uint8_t; VK_LUID_SIZE as usize],
    deviceNodeMask: uint32_t,
    deviceLUIDValid: VkBool32,
    /// The size of a subgroup for this queue.
    /// * **No Auto-validity:** true
    subgroupSize: uint32_t,
    /// Bitfield of what shader stages support subgroup operations
    /// * **No Auto-validity:** true
    subgroupSupportedStages: VkShaderStageFlags,
    /// Bitfield of what subgroup operations are supported.
    /// * **No Auto-validity:** true
    subgroupSupportedOperations: VkSubgroupFeatureFlags,
    /// Flag to specify whether quad operations are available in all stages.
    /// * **No Auto-validity:** true
    subgroupQuadOperationsInAllStages: VkBool32,
    pointClippingBehavior: VkPointClippingBehavior,
    /// max number of views in a subpass
    maxMultiviewViewCount: uint32_t,
    /// max instance index for a draw in a multiview subpass
    maxMultiviewInstanceIndex: uint32_t,
    protectedNoFault: VkBool32,
    maxPerSetDescriptors: uint32_t,
    maxMemoryAllocationSize: VkDeviceSize,
  }
}

structure! {
  /// [VkPhysicalDeviceVulkan12Features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVulkan12Features.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceVulkan12Features {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    samplerMirrorClampToEdge: VkBool32,
    drawIndirectCount: VkBool32,
    /// 8-bit integer variables supported in StorageBuffer
    storageBuffer8BitAccess: VkBool32,
    /// 8-bit integer variables supported in StorageBuffer and Uniform
    uniformAndStorageBuffer8BitAccess: VkBool32,
    /// 8-bit integer variables supported in PushConstant
    storagePushConstant8: VkBool32,
    shaderBufferInt64Atomics: VkBool32,
    shaderSharedInt64Atomics: VkBool32,
    /// 16-bit floats (halfs) in shaders
    shaderFloat16: VkBool32,
    /// 8-bit integers in shaders
    shaderInt8: VkBool32,
    descriptorIndexing: VkBool32,
    shaderInputAttachmentArrayDynamicIndexing: VkBool32,
    shaderUniformTexelBufferArrayDynamicIndexing: VkBool32,
    shaderStorageTexelBufferArrayDynamicIndexing: VkBool32,
    shaderUniformBufferArrayNonUniformIndexing: VkBool32,
    shaderSampledImageArrayNonUniformIndexing: VkBool32,
    shaderStorageBufferArrayNonUniformIndexing: VkBool32,
    shaderStorageImageArrayNonUniformIndexing: VkBool32,
    shaderInputAttachmentArrayNonUniformIndexing: VkBool32,
    shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32,
    shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32,
    descriptorBindingUniformBufferUpdateAfterBind: VkBool32,
    descriptorBindingSampledImageUpdateAfterBind: VkBool32,
    descriptorBindingStorageImageUpdateAfterBind: VkBool32,
    descriptorBindingStorageBufferUpdateAfterBind: VkBool32,
    descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32,
    descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32,
    descriptorBindingUpdateUnusedWhilePending: VkBool32,
    descriptorBindingPartiallyBound: VkBool32,
    descriptorBindingVariableDescriptorCount: VkBool32,
    runtimeDescriptorArray: VkBool32,
    samplerFilterMinmax: VkBool32,
    scalarBlockLayout: VkBool32,
    imagelessFramebuffer: VkBool32,
    uniformBufferStandardLayout: VkBool32,
    shaderSubgroupExtendedTypes: VkBool32,
    separateDepthStencilLayouts: VkBool32,
    hostQueryReset: VkBool32,
    timelineSemaphore: VkBool32,
    bufferDeviceAddress: VkBool32,
    bufferDeviceAddressCaptureReplay: VkBool32,
    bufferDeviceAddressMultiDevice: VkBool32,
    vulkanMemoryModel: VkBool32,
    vulkanMemoryModelDeviceScope: VkBool32,
    vulkanMemoryModelAvailabilityVisibilityChains: VkBool32,
    shaderOutputViewportIndex: VkBool32,
    shaderOutputLayer: VkBool32,
    subgroupBroadcastDynamicId: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceVulkan12Properties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVulkan12Properties.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceProperties2`]
  VkPhysicalDeviceVulkan12Properties {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    driverID: VkDriverId,
    driverName: [u8; VK_MAX_DRIVER_NAME_SIZE as usize],
    driverInfo: [u8; VK_MAX_DRIVER_INFO_SIZE as usize],
    conformanceVersion: VkConformanceVersion,
    denormBehaviorIndependence: VkShaderFloatControlsIndependence,
    roundingModeIndependence: VkShaderFloatControlsIndependence,
    /// An implementation can preserve signed zero, nan, inf
    shaderSignedZeroInfNanPreserveFloat16: VkBool32,
    /// An implementation can preserve signed zero, nan, inf
    shaderSignedZeroInfNanPreserveFloat32: VkBool32,
    /// An implementation can preserve signed zero, nan, inf
    shaderSignedZeroInfNanPreserveFloat64: VkBool32,
    /// An implementation can preserve  denormals
    shaderDenormPreserveFloat16: VkBool32,
    /// An implementation can preserve  denormals
    shaderDenormPreserveFloat32: VkBool32,
    /// An implementation can preserve  denormals
    shaderDenormPreserveFloat64: VkBool32,
    /// An implementation can flush to zero  denormals
    shaderDenormFlushToZeroFloat16: VkBool32,
    /// An implementation can flush to zero  denormals
    shaderDenormFlushToZeroFloat32: VkBool32,
    /// An implementation can flush to zero  denormals
    shaderDenormFlushToZeroFloat64: VkBool32,
    /// An implementation can support RTE
    shaderRoundingModeRTEFloat16: VkBool32,
    /// An implementation can support RTE
    shaderRoundingModeRTEFloat32: VkBool32,
    /// An implementation can support RTE
    shaderRoundingModeRTEFloat64: VkBool32,
    /// An implementation can support RTZ
    shaderRoundingModeRTZFloat16: VkBool32,
    /// An implementation can support RTZ
    shaderRoundingModeRTZFloat32: VkBool32,
    /// An implementation can support RTZ
    shaderRoundingModeRTZFloat64: VkBool32,
    maxUpdateAfterBindDescriptorsInAllPools: uint32_t,
    shaderUniformBufferArrayNonUniformIndexingNative: VkBool32,
    shaderSampledImageArrayNonUniformIndexingNative: VkBool32,
    shaderStorageBufferArrayNonUniformIndexingNative: VkBool32,
    shaderStorageImageArrayNonUniformIndexingNative: VkBool32,
    shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32,
    robustBufferAccessUpdateAfterBind: VkBool32,
    quadDivergentImplicitLod: VkBool32,
    maxPerStageDescriptorUpdateAfterBindSamplers: uint32_t,
    maxPerStageDescriptorUpdateAfterBindUniformBuffers: uint32_t,
    maxPerStageDescriptorUpdateAfterBindStorageBuffers: uint32_t,
    maxPerStageDescriptorUpdateAfterBindSampledImages: uint32_t,
    maxPerStageDescriptorUpdateAfterBindStorageImages: uint32_t,
    maxPerStageDescriptorUpdateAfterBindInputAttachments: uint32_t,
    maxPerStageUpdateAfterBindResources: uint32_t,
    maxDescriptorSetUpdateAfterBindSamplers: uint32_t,
    maxDescriptorSetUpdateAfterBindUniformBuffers: uint32_t,
    maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: uint32_t,
    maxDescriptorSetUpdateAfterBindStorageBuffers: uint32_t,
    maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: uint32_t,
    maxDescriptorSetUpdateAfterBindSampledImages: uint32_t,
    maxDescriptorSetUpdateAfterBindStorageImages: uint32_t,
    maxDescriptorSetUpdateAfterBindInputAttachments: uint32_t,
    /// supported depth resolve modes
    supportedDepthResolveModes: VkResolveModeFlags,
    /// supported stencil resolve modes
    supportedStencilResolveModes: VkResolveModeFlags,
    /// depth and stencil resolve modes can be set independently if one of them is none
    independentResolveNone: VkBool32,
    /// depth and stencil resolve modes can be set independently
    independentResolve: VkBool32,
    filterMinmaxSingleComponentFormats: VkBool32,
    filterMinmaxImageComponentMapping: VkBool32,
    maxTimelineSemaphoreValueDifference: uint64_t,
    /// * **Optional:** true
    framebufferIntegerColorSampleCounts: VkSampleCountFlags,
  }
}

structure! {
  /// [VkPhysicalDeviceVulkanMemoryModelFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVulkanMemoryModelFeatures.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceVulkanMemoryModelFeatures {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    vulkanMemoryModel: VkBool32,
    vulkanMemoryModelDeviceScope: VkBool32,
    vulkanMemoryModelAvailabilityVisibilityChains: VkBool32,
  }
}

structure! {
  /// [VkPhysicalDeviceYcbcrImageArraysFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceYcbcrImageArraysFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceFeatures2`], [`VkDeviceCreateInfo`]
  VkPhysicalDeviceYcbcrImageArraysFeaturesEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    ycbcrImageArrays: VkBool32,
  }
}

structure! {
  /// [VkPipelineCacheCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCacheCreateInfo.html)
  VkPipelineCacheCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkPipelineCacheCreateFlags,
    /// Size of initial data to populate cache, in bytes
    /// * **Optional:** true
    initialDataSize: size_t,
    /// Initial data to populate cache
    /// * **Len:** initialDataSize
    pInitialData: *const c_void,
  }
}

structure! {
  /// [VkPipelineColorBlendAdvancedStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineColorBlendAdvancedStateCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkPipelineColorBlendStateCreateInfo`]
  VkPipelineColorBlendAdvancedStateCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    srcPremultiplied: VkBool32,
    dstPremultiplied: VkBool32,
    blendOverlap: VkBlendOverlapEXT,
  }
}

structure! {
  /// [VkPipelineColorBlendAttachmentState](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineColorBlendAttachmentState.html)
  VkPipelineColorBlendAttachmentState {
    blendEnable: VkBool32,
    srcColorBlendFactor: VkBlendFactor,
    dstColorBlendFactor: VkBlendFactor,
    colorBlendOp: VkBlendOp,
    srcAlphaBlendFactor: VkBlendFactor,
    dstAlphaBlendFactor: VkBlendFactor,
    alphaBlendOp: VkBlendOp,
    /// * **Optional:** true
    colorWriteMask: VkColorComponentFlags,
  }
}

structure! {
  /// [VkPipelineColorBlendStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineColorBlendStateCreateInfo.html)
  VkPipelineColorBlendStateCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkPipelineColorBlendStateCreateFlags,
    logicOpEnable: VkBool32,
    /// * **No Auto-validity:** true
    logicOp: VkLogicOp,
    /// # of pAttachments
    /// * **Optional:** true
    attachmentCount: uint32_t,
    /// * **Len:** attachmentCount
    pAttachments: *const VkPipelineColorBlendAttachmentState,
    blendConstants: [float; 4],
  }
}

structure! {
  /// [VkPipelineCompilerControlCreateInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCompilerControlCreateInfoAMD.html)
  ///
  /// Struct Extends: [`VkGraphicsPipelineCreateInfo`], [`VkComputePipelineCreateInfo`]
  VkPipelineCompilerControlCreateInfoAMD {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    compilerControlFlags: VkPipelineCompilerControlFlagsAMD,
  }
}

structure! {
  /// [VkPipelineCoverageModulationStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCoverageModulationStateCreateInfoNV.html)
  ///
  /// Struct Extends: [`VkPipelineMultisampleStateCreateInfo`]
  VkPipelineCoverageModulationStateCreateInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkPipelineCoverageModulationStateCreateFlagsNV,
    coverageModulationMode: VkCoverageModulationModeNV,
    coverageModulationTableEnable: VkBool32,
    /// * **Optional:** true
    coverageModulationTableCount: uint32_t,
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    /// * **Len:** coverageModulationTableCount
    pCoverageModulationTable: *const float,
  }
}

structure! {
  /// [VkPipelineCoverageReductionStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCoverageReductionStateCreateInfoNV.html)
  ///
  /// Struct Extends: [`VkPipelineMultisampleStateCreateInfo`]
  VkPipelineCoverageReductionStateCreateInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkPipelineCoverageReductionStateCreateFlagsNV,
    coverageReductionMode: VkCoverageReductionModeNV,
  }
}

structure! {
  /// [VkPipelineCoverageToColorStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCoverageToColorStateCreateInfoNV.html)
  ///
  /// Struct Extends: [`VkPipelineMultisampleStateCreateInfo`]
  VkPipelineCoverageToColorStateCreateInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkPipelineCoverageToColorStateCreateFlagsNV,
    coverageToColorEnable: VkBool32,
    /// * **Optional:** true
    coverageToColorLocation: uint32_t,
  }
}

structure! {
  /// [VkPipelineCreationFeedbackCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCreationFeedbackCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkGraphicsPipelineCreateInfo`], [`VkComputePipelineCreateInfo`], [`VkRayTracingPipelineCreateInfoNV`], [`VkRayTracingPipelineCreateInfoKHR`]
  VkPipelineCreationFeedbackCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Output pipeline creation feedback.
    pPipelineCreationFeedback: *mut VkPipelineCreationFeedbackEXT,
    pipelineStageCreationFeedbackCount: uint32_t,
    /// One entry for each shader stage specified in the parent Vk*PipelineCreateInfo struct
    /// * **Len:** pipelineStageCreationFeedbackCount
    pPipelineStageCreationFeedbacks: *mut VkPipelineCreationFeedbackEXT,
  }
}

structure! {
  /// [VkPipelineCreationFeedbackEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCreationFeedbackEXT.html)
  VkPipelineCreationFeedbackEXT {
    flags: VkPipelineCreationFeedbackFlagsEXT,
    duration: uint64_t,
  }
}

structure! {
  /// [VkPipelineDepthStencilStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDepthStencilStateCreateInfo.html)
  VkPipelineDepthStencilStateCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkPipelineDepthStencilStateCreateFlags,
    depthTestEnable: VkBool32,
    depthWriteEnable: VkBool32,
    depthCompareOp: VkCompareOp,
    /// optional (depth_bounds_test)
    depthBoundsTestEnable: VkBool32,
    stencilTestEnable: VkBool32,
    front: VkStencilOpState,
    back: VkStencilOpState,
    minDepthBounds: float,
    maxDepthBounds: float,
  }
}

structure! {
  /// [VkPipelineDiscardRectangleStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDiscardRectangleStateCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkGraphicsPipelineCreateInfo`]
  VkPipelineDiscardRectangleStateCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkPipelineDiscardRectangleStateCreateFlagsEXT,
    discardRectangleMode: VkDiscardRectangleModeEXT,
    /// * **Optional:** true
    discardRectangleCount: uint32_t,
    /// * **No Auto-validity:** true
    /// * **Len:** discardRectangleCount
    pDiscardRectangles: *const VkRect2D,
  }
}

structure! {
  /// [VkPipelineDynamicStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDynamicStateCreateInfo.html)
  VkPipelineDynamicStateCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkPipelineDynamicStateCreateFlags,
    /// * **Optional:** true
    dynamicStateCount: uint32_t,
    /// * **Len:** dynamicStateCount
    pDynamicStates: *const VkDynamicState,
  }
}

structure! {
  /// [VkPipelineExecutableInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineExecutableInfoKHR.html)
  VkPipelineExecutableInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    pipeline: VkPipeline,
    executableIndex: uint32_t,
  }
}

structure! {
  /// [VkPipelineExecutableInternalRepresentationKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineExecutableInternalRepresentationKHR.html)
  VkPipelineExecutableInternalRepresentationKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    name: [u8; VK_MAX_DESCRIPTION_SIZE as usize],
    description: [u8; VK_MAX_DESCRIPTION_SIZE as usize],
    isText: VkBool32,
    dataSize: size_t,
    /// * **Optional:** true
    /// * **Len:** dataSize
    pData: *mut c_void,
  }
}

structure! {
  /// [VkPipelineExecutablePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineExecutablePropertiesKHR.html)
  VkPipelineExecutablePropertiesKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_PROPERTIES_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    stages: VkShaderStageFlags,
    name: [u8; VK_MAX_DESCRIPTION_SIZE as usize],
    description: [u8; VK_MAX_DESCRIPTION_SIZE as usize],
    subgroupSize: uint32_t,
  }
}

structure! {
  /// [VkPipelineExecutableStatisticKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineExecutableStatisticKHR.html)
  VkPipelineExecutableStatisticKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_STATISTIC_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    name: [u8; VK_MAX_DESCRIPTION_SIZE as usize],
    description: [u8; VK_MAX_DESCRIPTION_SIZE as usize],
    format: VkPipelineExecutableStatisticFormatKHR,
    value: VkPipelineExecutableStatisticValueKHR,
  }
}

structure! {
  /// [VkPipelineFragmentShadingRateEnumStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineFragmentShadingRateEnumStateCreateInfoNV.html)
  ///
  /// Struct Extends: [`VkGraphicsPipelineCreateInfo`]
  VkPipelineFragmentShadingRateEnumStateCreateInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    shadingRateType: VkFragmentShadingRateTypeNV,
    shadingRate: VkFragmentShadingRateNV,
    combinerOps: [VkFragmentShadingRateCombinerOpKHR; 2],
  }
}

structure! {
  /// [VkPipelineFragmentShadingRateStateCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineFragmentShadingRateStateCreateInfoKHR.html)
  ///
  /// Struct Extends: [`VkGraphicsPipelineCreateInfo`]
  VkPipelineFragmentShadingRateStateCreateInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    fragmentSize: VkExtent2D,
    combinerOps: [VkFragmentShadingRateCombinerOpKHR; 2],
  }
}

structure! {
  /// [VkPipelineInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineInfoKHR.html)
  VkPipelineInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    pipeline: VkPipeline,
  }
}

structure! {
  /// [VkPipelineInputAssemblyStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineInputAssemblyStateCreateInfo.html)
  VkPipelineInputAssemblyStateCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkPipelineInputAssemblyStateCreateFlags,
    topology: VkPrimitiveTopology,
    primitiveRestartEnable: VkBool32,
  }
}

structure! {
  /// [VkPipelineLayoutCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineLayoutCreateInfo.html)
  VkPipelineLayoutCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkPipelineLayoutCreateFlags,
    /// Number of descriptor sets interfaced by the pipeline
    /// * **Optional:** true
    setLayoutCount: uint32_t,
    /// Array of setCount number of descriptor set layout objects defining the layout of the
    /// * **Len:** setLayoutCount
    pSetLayouts: *const VkDescriptorSetLayout,
    /// Number of push-constant ranges used by the pipeline
    /// * **Optional:** true
    pushConstantRangeCount: uint32_t,
    /// Array of pushConstantRangeCount number of ranges used by various shader stages
    /// * **Len:** pushConstantRangeCount
    pPushConstantRanges: *const VkPushConstantRange,
  }
}

structure! {
  /// [VkPipelineLibraryCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineLibraryCreateInfoKHR.html)
  VkPipelineLibraryCreateInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_LIBRARY_CREATE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    libraryCount: uint32_t,
    /// * **Len:** libraryCount
    pLibraries: *const VkPipeline,
  }
}

structure! {
  /// [VkPipelineMultisampleStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineMultisampleStateCreateInfo.html)
  VkPipelineMultisampleStateCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkPipelineMultisampleStateCreateFlags,
    /// Number of samples used for rasterization
    rasterizationSamples: VkSampleCountFlagBits,
    /// optional (GL45)
    sampleShadingEnable: VkBool32,
    /// optional (GL45)
    minSampleShading: float,
    /// Array of sampleMask words
    /// * **Optional:** true
    /// * **Len:** latexmath:[\lceil{\mathit{rasterizationSamples} \over 32}\rceil]
    /// * **Alt Len:** (rasterizationSamples + 31) / 32
    pSampleMask: *const VkSampleMask,
    alphaToCoverageEnable: VkBool32,
    alphaToOneEnable: VkBool32,
  }
}

structure! {
  /// [VkPipelineRasterizationConservativeStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationConservativeStateCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkPipelineRasterizationStateCreateInfo`]
  VkPipelineRasterizationConservativeStateCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Reserved
    /// * **Optional:** true
    flags: VkPipelineRasterizationConservativeStateCreateFlagsEXT,
    /// Conservative rasterization mode
    conservativeRasterizationMode: VkConservativeRasterizationModeEXT,
    /// Extra overestimation to add to the primitive
    extraPrimitiveOverestimationSize: float,
  }
}

structure! {
  /// [VkPipelineRasterizationDepthClipStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationDepthClipStateCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkPipelineRasterizationStateCreateInfo`]
  VkPipelineRasterizationDepthClipStateCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Reserved
    /// * **Optional:** true
    flags: VkPipelineRasterizationDepthClipStateCreateFlagsEXT,
    depthClipEnable: VkBool32,
  }
}

structure! {
  /// [VkPipelineRasterizationLineStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationLineStateCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkPipelineRasterizationStateCreateInfo`]
  VkPipelineRasterizationLineStateCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    lineRasterizationMode: VkLineRasterizationModeEXT,
    stippledLineEnable: VkBool32,
    lineStippleFactor: uint32_t,
    lineStipplePattern: uint16_t,
  }
}

structure! {
  /// [VkPipelineRasterizationStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationStateCreateInfo.html)
  VkPipelineRasterizationStateCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkPipelineRasterizationStateCreateFlags,
    depthClampEnable: VkBool32,
    rasterizerDiscardEnable: VkBool32,
    /// optional (GL45)
    polygonMode: VkPolygonMode,
    /// * **Optional:** true
    cullMode: VkCullModeFlags,
    frontFace: VkFrontFace,
    depthBiasEnable: VkBool32,
    depthBiasConstantFactor: float,
    depthBiasClamp: float,
    depthBiasSlopeFactor: float,
    lineWidth: float,
  }
}

structure! {
  /// [VkPipelineRasterizationStateRasterizationOrderAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationStateRasterizationOrderAMD.html)
  ///
  /// Struct Extends: [`VkPipelineRasterizationStateCreateInfo`]
  VkPipelineRasterizationStateRasterizationOrderAMD {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Rasterization order to use for the pipeline
    rasterizationOrder: VkRasterizationOrderAMD,
  }
}

structure! {
  /// [VkPipelineRasterizationStateStreamCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationStateStreamCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkPipelineRasterizationStateCreateInfo`]
  VkPipelineRasterizationStateStreamCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkPipelineRasterizationStateStreamCreateFlagsEXT,
    rasterizationStream: uint32_t,
  }
}

structure! {
  /// [VkPipelineRepresentativeFragmentTestStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRepresentativeFragmentTestStateCreateInfoNV.html)
  ///
  /// Struct Extends: [`VkGraphicsPipelineCreateInfo`]
  VkPipelineRepresentativeFragmentTestStateCreateInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    representativeFragmentTestEnable: VkBool32,
  }
}

structure! {
  /// [VkPipelineSampleLocationsStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineSampleLocationsStateCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkPipelineMultisampleStateCreateInfo`]
  VkPipelineSampleLocationsStateCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    sampleLocationsEnable: VkBool32,
    sampleLocationsInfo: VkSampleLocationsInfoEXT,
  }
}

structure! {
  /// [VkPipelineShaderStageCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineShaderStageCreateInfo.html)
  VkPipelineShaderStageCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkPipelineShaderStageCreateFlags,
    /// Shader stage
    stage: VkShaderStageFlagBits,
    /// Module containing entry point
    module: VkShaderModule,
    /// Null-terminated entry point name
    /// * **Len:** null-terminated
    pName: *const u8,
    /// * **Optional:** true
    pSpecializationInfo: *const VkSpecializationInfo,
  }
}

structure! {
  /// [VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkPipelineShaderStageCreateInfo`]
  VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    requiredSubgroupSize: uint32_t,
  }
}

structure! {
  /// [VkPipelineTessellationDomainOriginStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineTessellationDomainOriginStateCreateInfo.html)
  ///
  /// Struct Extends: [`VkPipelineTessellationStateCreateInfo`]
  VkPipelineTessellationDomainOriginStateCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    domainOrigin: VkTessellationDomainOrigin,
  }
}

structure! {
  /// [VkPipelineTessellationStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineTessellationStateCreateInfo.html)
  VkPipelineTessellationStateCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkPipelineTessellationStateCreateFlags,
    patchControlPoints: uint32_t,
  }
}

structure! {
  /// [VkPipelineVertexInputDivisorStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineVertexInputDivisorStateCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkPipelineVertexInputStateCreateInfo`]
  VkPipelineVertexInputDivisorStateCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    vertexBindingDivisorCount: uint32_t,
    /// * **Len:** vertexBindingDivisorCount
    pVertexBindingDivisors: *const VkVertexInputBindingDivisorDescriptionEXT,
  }
}

structure! {
  /// [VkPipelineVertexInputStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineVertexInputStateCreateInfo.html)
  VkPipelineVertexInputStateCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkPipelineVertexInputStateCreateFlags,
    /// number of bindings
    /// * **Optional:** true
    vertexBindingDescriptionCount: uint32_t,
    /// * **Len:** vertexBindingDescriptionCount
    pVertexBindingDescriptions: *const VkVertexInputBindingDescription,
    /// number of attributes
    /// * **Optional:** true
    vertexAttributeDescriptionCount: uint32_t,
    /// * **Len:** vertexAttributeDescriptionCount
    pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription,
  }
}

structure! {
  /// [VkPipelineViewportCoarseSampleOrderStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportCoarseSampleOrderStateCreateInfoNV.html)
  ///
  /// Struct Extends: [`VkPipelineViewportStateCreateInfo`]
  VkPipelineViewportCoarseSampleOrderStateCreateInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    sampleOrderType: VkCoarseSampleOrderTypeNV,
    /// * **Optional:** true
    customSampleOrderCount: uint32_t,
    /// * **Len:** customSampleOrderCount
    pCustomSampleOrders: *const VkCoarseSampleOrderCustomNV,
  }
}

structure! {
  /// [VkPipelineViewportExclusiveScissorStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportExclusiveScissorStateCreateInfoNV.html)
  ///
  /// Struct Extends: [`VkPipelineViewportStateCreateInfo`]
  VkPipelineViewportExclusiveScissorStateCreateInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    exclusiveScissorCount: uint32_t,
    /// * **No Auto-validity:** true
    /// * **Len:** exclusiveScissorCount
    pExclusiveScissors: *const VkRect2D,
  }
}

structure! {
  /// [VkPipelineViewportShadingRateImageStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportShadingRateImageStateCreateInfoNV.html)
  ///
  /// Struct Extends: [`VkPipelineViewportStateCreateInfo`]
  VkPipelineViewportShadingRateImageStateCreateInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    shadingRateImageEnable: VkBool32,
    /// * **Optional:** true
    viewportCount: uint32_t,
    /// * **No Auto-validity:** true
    /// * **Len:** viewportCount
    pShadingRatePalettes: *const VkShadingRatePaletteNV,
  }
}

structure! {
  /// [VkPipelineViewportStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportStateCreateInfo.html)
  VkPipelineViewportStateCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkPipelineViewportStateCreateFlags,
    /// * **Optional:** true
    viewportCount: uint32_t,
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    /// * **Len:** viewportCount
    pViewports: *const VkViewport,
    /// * **Optional:** true
    scissorCount: uint32_t,
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    /// * **Len:** scissorCount
    pScissors: *const VkRect2D,
  }
}

structure! {
  /// [VkPipelineViewportSwizzleStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportSwizzleStateCreateInfoNV.html)
  ///
  /// Struct Extends: [`VkPipelineViewportStateCreateInfo`]
  VkPipelineViewportSwizzleStateCreateInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkPipelineViewportSwizzleStateCreateFlagsNV,
    viewportCount: uint32_t,
    /// * **Len:** viewportCount
    pViewportSwizzles: *const VkViewportSwizzleNV,
  }
}

structure! {
  /// [VkPipelineViewportWScalingStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportWScalingStateCreateInfoNV.html)
  ///
  /// Struct Extends: [`VkPipelineViewportStateCreateInfo`]
  VkPipelineViewportWScalingStateCreateInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    viewportWScalingEnable: VkBool32,
    viewportCount: uint32_t,
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    /// * **Len:** viewportCount
    pViewportWScalings: *const VkViewportWScalingNV,
  }
}

#[cfg(feature = "google_games_platform")]
structure! {
  /// [VkPresentFrameTokenGGP](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentFrameTokenGGP.html)
  ///
  /// Struct Extends: [`VkPresentInfoKHR`]
  VkPresentFrameTokenGGP {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PRESENT_FRAME_TOKEN_GGP`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    frameToken: GgpFrameToken,
  }
}

structure! {
  /// [VkPresentInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentInfoKHR.html)
  VkPresentInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PRESENT_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Number of semaphores to wait for before presenting
    /// * **Optional:** true
    waitSemaphoreCount: uint32_t,
    /// Semaphores to wait for before presenting
    /// * **Len:** waitSemaphoreCount
    pWaitSemaphores: *const VkSemaphore,
    /// Number of swapchains to present in this call
    swapchainCount: uint32_t,
    /// Swapchains to present an image from
    /// * **Len:** swapchainCount
    pSwapchains: *const VkSwapchainKHR,
    /// Indices of which presentable images to present
    /// * **Len:** swapchainCount
    pImageIndices: *const uint32_t,
    /// Optional (i.e. if non-NULL) VkResult for each swapchain
    /// * **Optional:** true
    /// * **Len:** swapchainCount
    pResults: *mut VkResult,
  }
}

structure! {
  /// [VkPresentRegionKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentRegionKHR.html)
  VkPresentRegionKHR {
    /// Number of rectangles in pRectangles
    /// * **Optional:** true
    rectangleCount: uint32_t,
    /// Array of rectangles that have changed in a swapchain's image(s)
    /// * **Optional:** true
    /// * **Len:** rectangleCount
    pRectangles: *const VkRectLayerKHR,
  }
}

structure! {
  /// [VkPresentRegionsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentRegionsKHR.html)
  ///
  /// Struct Extends: [`VkPresentInfoKHR`]
  VkPresentRegionsKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Copy of VkPresentInfoKHR::swapchainCount
    swapchainCount: uint32_t,
    /// The regions that have changed
    /// * **Optional:** true
    /// * **Len:** swapchainCount
    pRegions: *const VkPresentRegionKHR,
  }
}

structure! {
  /// [VkPresentTimeGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentTimeGOOGLE.html)
  VkPresentTimeGOOGLE {
    /// Application-provided identifier
    presentID: uint32_t,
    /// Earliest time an image should be presented
    desiredPresentTime: uint64_t,
  }
}

structure! {
  /// [VkPresentTimesInfoGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentTimesInfoGOOGLE.html)
  ///
  /// Struct Extends: [`VkPresentInfoKHR`]
  VkPresentTimesInfoGOOGLE {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Copy of VkPresentInfoKHR::swapchainCount
    swapchainCount: uint32_t,
    /// The earliest times to present images
    /// * **Optional:** true
    /// * **Len:** swapchainCount
    pTimes: *const VkPresentTimeGOOGLE,
  }
}

structure! {
  /// [VkPrivateDataSlotCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPrivateDataSlotCreateInfoEXT.html)
  VkPrivateDataSlotCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    flags: VkPrivateDataSlotCreateFlagsEXT,
  }
}

structure! {
  /// [VkProtectedSubmitInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkProtectedSubmitInfo.html)
  ///
  /// Struct Extends: [`VkSubmitInfo`]
  VkProtectedSubmitInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_PROTECTED_SUBMIT_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Submit protected command buffers
    protectedSubmit: VkBool32,
  }
}

structure! {
  /// [VkPushConstantRange](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPushConstantRange.html)
  VkPushConstantRange {
    /// Which stages use the range
    stageFlags: VkShaderStageFlags,
    /// Start of the range, in bytes
    offset: uint32_t,
    /// Size of the range, in bytes
    size: uint32_t,
  }
}

structure! {
  /// [VkQueryPoolCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolCreateInfo.html)
  VkQueryPoolCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkQueryPoolCreateFlags,
    queryType: VkQueryType,
    queryCount: uint32_t,
    /// Optional
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    pipelineStatistics: VkQueryPipelineStatisticFlags,
  }
}

structure! {
  /// [VkQueryPoolPerformanceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolPerformanceCreateInfoKHR.html)
  ///
  /// Struct Extends: [`VkQueryPoolCreateInfo`]
  VkQueryPoolPerformanceCreateInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    queueFamilyIndex: uint32_t,
    counterIndexCount: uint32_t,
    /// * **Len:** counterIndexCount
    pCounterIndices: *const uint32_t,
  }
}

structure! {
  /// [VkQueryPoolPerformanceQueryCreateInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolPerformanceQueryCreateInfoINTEL.html)
  ///
  /// Struct Extends: [`VkQueryPoolCreateInfo`]
  VkQueryPoolPerformanceQueryCreateInfoINTEL {
    /// * **Values:** [`VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    performanceCountersSampling: VkQueryPoolSamplingModeINTEL,
  }
}

structure! {
  /// [VkQueueFamilyCheckpointPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueFamilyCheckpointPropertiesNV.html)
  ///
  /// Struct Extends: [`VkQueueFamilyProperties2`]
  VkQueueFamilyCheckpointPropertiesNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    checkpointExecutionStageMask: VkPipelineStageFlags,
  }
}

structure! {
  /// [VkQueueFamilyProperties2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueFamilyProperties2.html)
  VkQueueFamilyProperties2 {
    /// * **Values:** [`VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    queueFamilyProperties: VkQueueFamilyProperties,
  }
}

structure! {
  /// [VkRayTracingPipelineCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingPipelineCreateInfoKHR.html)
  VkRayTracingPipelineCreateInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Pipeline creation flags
    /// * **Optional:** true
    flags: VkPipelineCreateFlags,
    /// * **Optional:** true
    stageCount: uint32_t,
    /// One entry for each active shader stage
    /// * **Len:** stageCount
    pStages: *const VkPipelineShaderStageCreateInfo,
    /// * **Optional:** true
    groupCount: uint32_t,
    /// * **Len:** groupCount
    pGroups: *const VkRayTracingShaderGroupCreateInfoKHR,
    maxPipelineRayRecursionDepth: uint32_t,
    /// * **Optional:** true
    pLibraryInfo: *const VkPipelineLibraryCreateInfoKHR,
    /// * **Optional:** true
    pLibraryInterface: *const VkRayTracingPipelineInterfaceCreateInfoKHR,
    /// * **Optional:** true
    pDynamicState: *const VkPipelineDynamicStateCreateInfo,
    /// Interface layout of the pipeline
    layout: VkPipelineLayout,
    /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is nonzero, it specifies the handle of the base pipeline this is a derivative of
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    basePipelineHandle: VkPipeline,
    /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is not -1, it specifies an index into pCreateInfos of the base pipeline this is a derivative of
    basePipelineIndex: int32_t,
  }
}

structure! {
  /// [VkRayTracingPipelineCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingPipelineCreateInfoNV.html)
  VkRayTracingPipelineCreateInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Pipeline creation flags
    /// * **Optional:** true
    flags: VkPipelineCreateFlags,
    stageCount: uint32_t,
    /// One entry for each active shader stage
    /// * **Len:** stageCount
    pStages: *const VkPipelineShaderStageCreateInfo,
    groupCount: uint32_t,
    /// * **Len:** groupCount
    pGroups: *const VkRayTracingShaderGroupCreateInfoNV,
    maxRecursionDepth: uint32_t,
    /// Interface layout of the pipeline
    layout: VkPipelineLayout,
    /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is nonzero, it specifies the handle of the base pipeline this is a derivative of
    /// * **Optional:** true
    /// * **No Auto-validity:** true
    basePipelineHandle: VkPipeline,
    /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is not -1, it specifies an index into pCreateInfos of the base pipeline this is a derivative of
    basePipelineIndex: int32_t,
  }
}

structure! {
  /// [VkRayTracingPipelineInterfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingPipelineInterfaceCreateInfoKHR.html)
  VkRayTracingPipelineInterfaceCreateInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    maxPipelineRayPayloadSize: uint32_t,
    maxPipelineRayHitAttributeSize: uint32_t,
  }
}

structure! {
  /// [VkRayTracingShaderGroupCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingShaderGroupCreateInfoKHR.html)
  VkRayTracingShaderGroupCreateInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    type_: VkRayTracingShaderGroupTypeKHR,
    generalShader: uint32_t,
    closestHitShader: uint32_t,
    anyHitShader: uint32_t,
    intersectionShader: uint32_t,
    /// * **Optional:** true
    pShaderGroupCaptureReplayHandle: *const c_void,
  }
}

structure! {
  /// [VkRayTracingShaderGroupCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingShaderGroupCreateInfoNV.html)
  VkRayTracingShaderGroupCreateInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    type_: VkRayTracingShaderGroupTypeKHR,
    generalShader: uint32_t,
    closestHitShader: uint32_t,
    anyHitShader: uint32_t,
    intersectionShader: uint32_t,
  }
}

structure! {
  /// [VkRectLayerKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRectLayerKHR.html)
  VkRectLayerKHR {
    /// upper-left corner of a rectangle that has not changed, in pixels of a presentation images
    offset: VkOffset2D,
    /// Dimensions of a rectangle that has not changed, in pixels of a presentation images
    /// * **No Auto-validity:** true
    extent: VkExtent2D,
    /// Layer of a swapchain's image(s), for stereoscopic-3D images
    layer: uint32_t,
  }
}

structure! {
  /// [VkRefreshCycleDurationGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRefreshCycleDurationGOOGLE.html)
  VkRefreshCycleDurationGOOGLE {
    /// Number of nanoseconds from the start of one refresh cycle to the next
    refreshDuration: uint64_t,
  }
}

structure! {
  /// [VkRenderPassAttachmentBeginInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassAttachmentBeginInfo.html)
  ///
  /// Struct Extends: [`VkRenderPassBeginInfo`]
  VkRenderPassAttachmentBeginInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    attachmentCount: uint32_t,
    /// * **Len:** attachmentCount
    pAttachments: *const VkImageView,
  }
}

structure! {
  /// [VkRenderPassBeginInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassBeginInfo.html)
  VkRenderPassBeginInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    renderPass: VkRenderPass,
    framebuffer: VkFramebuffer,
    renderArea: VkRect2D,
    /// * **Optional:** true
    clearValueCount: uint32_t,
    /// * **Len:** clearValueCount
    pClearValues: *const VkClearValue,
  }
}

structure! {
  /// [VkRenderPassCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassCreateInfo.html)
  VkRenderPassCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkRenderPassCreateFlags,
    /// * **Optional:** true
    attachmentCount: uint32_t,
    /// * **Len:** attachmentCount
    pAttachments: *const VkAttachmentDescription,
    subpassCount: uint32_t,
    /// * **Len:** subpassCount
    pSubpasses: *const VkSubpassDescription,
    /// * **Optional:** true
    dependencyCount: uint32_t,
    /// * **Len:** dependencyCount
    pDependencies: *const VkSubpassDependency,
  }
}

structure! {
  /// [VkRenderPassCreateInfo2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassCreateInfo2.html)
  VkRenderPassCreateInfo2 {
    /// * **Values:** [`VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkRenderPassCreateFlags,
    /// * **Optional:** true
    attachmentCount: uint32_t,
    /// * **Len:** attachmentCount
    pAttachments: *const VkAttachmentDescription2,
    subpassCount: uint32_t,
    /// * **Len:** subpassCount
    pSubpasses: *const VkSubpassDescription2,
    /// * **Optional:** true
    dependencyCount: uint32_t,
    /// * **Len:** dependencyCount
    pDependencies: *const VkSubpassDependency2,
    /// * **Optional:** true
    correlatedViewMaskCount: uint32_t,
    /// * **Len:** correlatedViewMaskCount
    pCorrelatedViewMasks: *const uint32_t,
  }
}

structure! {
  /// [VkRenderPassFragmentDensityMapCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassFragmentDensityMapCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkRenderPassCreateInfo`], [`VkRenderPassCreateInfo2`]
  VkRenderPassFragmentDensityMapCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    fragmentDensityMapAttachment: VkAttachmentReference,
  }
}

structure! {
  /// [VkRenderPassInputAttachmentAspectCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassInputAttachmentAspectCreateInfo.html)
  ///
  /// Struct Extends: [`VkRenderPassCreateInfo`]
  VkRenderPassInputAttachmentAspectCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    aspectReferenceCount: uint32_t,
    /// * **Len:** aspectReferenceCount
    pAspectReferences: *const VkInputAttachmentAspectReference,
  }
}

structure! {
  /// [VkRenderPassMultiviewCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassMultiviewCreateInfo.html)
  ///
  /// Struct Extends: [`VkRenderPassCreateInfo`]
  VkRenderPassMultiviewCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    subpassCount: uint32_t,
    /// * **Len:** subpassCount
    pViewMasks: *const uint32_t,
    /// * **Optional:** true
    dependencyCount: uint32_t,
    /// * **Len:** dependencyCount
    pViewOffsets: *const int32_t,
    /// * **Optional:** true
    correlationMaskCount: uint32_t,
    /// * **Len:** correlationMaskCount
    pCorrelationMasks: *const uint32_t,
  }
}

structure! {
  /// [VkRenderPassSampleLocationsBeginInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassSampleLocationsBeginInfoEXT.html)
  ///
  /// Struct Extends: [`VkRenderPassBeginInfo`]
  VkRenderPassSampleLocationsBeginInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    attachmentInitialSampleLocationsCount: uint32_t,
    /// * **Len:** attachmentInitialSampleLocationsCount
    pAttachmentInitialSampleLocations: *const VkAttachmentSampleLocationsEXT,
    /// * **Optional:** true
    postSubpassSampleLocationsCount: uint32_t,
    /// * **Len:** postSubpassSampleLocationsCount
    pPostSubpassSampleLocations: *const VkSubpassSampleLocationsEXT,
  }
}

structure! {
  /// [VkRenderPassTransformBeginInfoQCOM](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassTransformBeginInfoQCOM.html)
  ///
  /// Struct Extends: [`VkRenderPassBeginInfo`]
  VkRenderPassTransformBeginInfoQCOM {
    /// * **Values:** [`VK_STRUCTURE_TYPE_RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM`]
    sType: VkStructureType,
    /// Pointer to next structure
    /// * **Optional:** true
    pNext: *mut c_void,
    /// * **No Auto-validity:** true
    transform: VkSurfaceTransformFlagBitsKHR,
  }
}

structure! {
  /// [VkResolveImageInfo2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkResolveImageInfo2KHR.html)
  VkResolveImageInfo2KHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: uint32_t,
    /// * **Len:** regionCount
    pRegions: *const VkImageResolve2KHR,
  }
}

structure! {
  /// [VkSampleLocationEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSampleLocationEXT.html)
  VkSampleLocationEXT {
    x: float,
    y: float,
  }
}

structure! {
  /// [VkSampleLocationsInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSampleLocationsInfoEXT.html)
  ///
  /// Struct Extends: [`VkImageMemoryBarrier`]
  VkSampleLocationsInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **No Auto-validity:** true
    sampleLocationsPerPixel: VkSampleCountFlagBits,
    sampleLocationGridSize: VkExtent2D,
    /// * **Optional:** true
    sampleLocationsCount: uint32_t,
    /// * **Len:** sampleLocationsCount
    pSampleLocations: *const VkSampleLocationEXT,
  }
}

structure! {
  /// [VkSamplerCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerCreateInfo.html)
  VkSamplerCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkSamplerCreateFlags,
    /// Filter mode for magnification
    magFilter: VkFilter,
    /// Filter mode for minifiation
    minFilter: VkFilter,
    /// Mipmap selection mode
    mipmapMode: VkSamplerMipmapMode,
    addressModeU: VkSamplerAddressMode,
    addressModeV: VkSamplerAddressMode,
    addressModeW: VkSamplerAddressMode,
    mipLodBias: float,
    anisotropyEnable: VkBool32,
    maxAnisotropy: float,
    compareEnable: VkBool32,
    /// * **No Auto-validity:** true
    compareOp: VkCompareOp,
    minLod: float,
    maxLod: float,
    /// * **No Auto-validity:** true
    borderColor: VkBorderColor,
    unnormalizedCoordinates: VkBool32,
  }
}

structure! {
  /// [VkSamplerCustomBorderColorCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerCustomBorderColorCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkSamplerCreateInfo`]
  VkSamplerCustomBorderColorCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    customBorderColor: VkClearColorValue,
    format: VkFormat,
  }
}

structure! {
  /// [VkSamplerReductionModeCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerReductionModeCreateInfo.html)
  ///
  /// Struct Extends: [`VkSamplerCreateInfo`]
  VkSamplerReductionModeCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    reductionMode: VkSamplerReductionMode,
  }
}

structure! {
  /// [VkSamplerYcbcrConversionCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrConversionCreateInfo.html)
  VkSamplerYcbcrConversionCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    format: VkFormat,
    ycbcrModel: VkSamplerYcbcrModelConversion,
    ycbcrRange: VkSamplerYcbcrRange,
    components: VkComponentMapping,
    xChromaOffset: VkChromaLocation,
    yChromaOffset: VkChromaLocation,
    chromaFilter: VkFilter,
    forceExplicitReconstruction: VkBool32,
  }
}

structure! {
  /// [VkSamplerYcbcrConversionImageFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrConversionImageFormatProperties.html)
  ///
  /// Struct Extends: [`VkImageFormatProperties2`]
  VkSamplerYcbcrConversionImageFormatProperties {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    combinedImageSamplerDescriptorCount: uint32_t,
  }
}

structure! {
  /// [VkSamplerYcbcrConversionInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrConversionInfo.html)
  ///
  /// Struct Extends: [`VkSamplerCreateInfo`], [`VkImageViewCreateInfo`]
  VkSamplerYcbcrConversionInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    conversion: VkSamplerYcbcrConversion,
  }
}

structure! {
  /// [VkSemaphoreCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreCreateInfo.html)
  VkSemaphoreCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Semaphore creation flags
    /// * **Optional:** true
    flags: VkSemaphoreCreateFlags,
  }
}

structure! {
  /// [VkSemaphoreGetFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreGetFdInfoKHR.html)
  VkSemaphoreGetFdInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    semaphore: VkSemaphore,
    handleType: VkExternalSemaphoreHandleTypeFlagBits,
  }
}

structure! {
  /// [VkSemaphoreGetWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreGetWin32HandleInfoKHR.html)
  VkSemaphoreGetWin32HandleInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    semaphore: VkSemaphore,
    handleType: VkExternalSemaphoreHandleTypeFlagBits,
  }
}

structure! {
  /// [VkSemaphoreSignalInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreSignalInfo.html)
  VkSemaphoreSignalInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    semaphore: VkSemaphore,
    value: uint64_t,
  }
}

structure! {
  /// [VkSemaphoreTypeCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreTypeCreateInfo.html)
  ///
  /// Struct Extends: [`VkSemaphoreCreateInfo`], [`VkPhysicalDeviceExternalSemaphoreInfo`]
  VkSemaphoreTypeCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    semaphoreType: VkSemaphoreType,
    initialValue: uint64_t,
  }
}

structure! {
  /// [VkSemaphoreWaitInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreWaitInfo.html)
  VkSemaphoreWaitInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkSemaphoreWaitFlags,
    semaphoreCount: uint32_t,
    /// * **Len:** semaphoreCount
    pSemaphores: *const VkSemaphore,
    /// * **Len:** semaphoreCount
    pValues: *const uint64_t,
  }
}

structure! {
  /// [VkSetStateFlagsIndirectCommandNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSetStateFlagsIndirectCommandNV.html)
  VkSetStateFlagsIndirectCommandNV {
    data: uint32_t,
  }
}

structure! {
  /// [VkShaderModuleCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderModuleCreateInfo.html)
  VkShaderModuleCreateInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkShaderModuleCreateFlags,
    /// Specified in bytes
    codeSize: size_t,
    /// Binary code of size `codeSize`
    /// * **Len:** `codeSize / 4`
    pCode: *const uint32_t,
  }
}

structure! {
  /// [VkShaderModuleValidationCacheCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderModuleValidationCacheCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkShaderModuleCreateInfo`]
  VkShaderModuleValidationCacheCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    validationCache: VkValidationCacheEXT,
  }
}

structure! {
  /// [VkShaderResourceUsageAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderResourceUsageAMD.html)
  VkShaderResourceUsageAMD {
    numUsedVgprs: uint32_t,
    numUsedSgprs: uint32_t,
    ldsSizePerLocalWorkGroup: uint32_t,
    ldsUsageSizeInBytes: size_t,
    scratchMemUsageInBytes: size_t,
  }
}

structure! {
  /// [VkShaderStatisticsInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderStatisticsInfoAMD.html)
  VkShaderStatisticsInfoAMD {
    shaderStageMask: VkShaderStageFlags,
    resourceUsage: VkShaderResourceUsageAMD,
    numPhysicalVgprs: uint32_t,
    numPhysicalSgprs: uint32_t,
    numAvailableVgprs: uint32_t,
    numAvailableSgprs: uint32_t,
    computeWorkGroupSize: [uint32_t; 3],
  }
}

structure! {
  /// [VkShadingRatePaletteNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShadingRatePaletteNV.html)
  VkShadingRatePaletteNV {
    shadingRatePaletteEntryCount: uint32_t,
    /// * **Len:** shadingRatePaletteEntryCount
    pShadingRatePaletteEntries: *const VkShadingRatePaletteEntryNV,
  }
}

structure! {
  /// [VkSharedPresentSurfaceCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSharedPresentSurfaceCapabilitiesKHR.html)
  ///
  /// Struct Extends: [`VkSurfaceCapabilities2KHR`]
  VkSharedPresentSurfaceCapabilitiesKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    /// Supported image usage flags if swapchain created using a shared present mode
    /// * **Optional:** true
    sharedPresentSupportedUsageFlags: VkImageUsageFlags,
  }
}

structure! {
  /// [VkSparseBufferMemoryBindInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseBufferMemoryBindInfo.html)
  VkSparseBufferMemoryBindInfo {
    buffer: VkBuffer,
    bindCount: uint32_t,
    /// * **Len:** bindCount
    pBinds: *const VkSparseMemoryBind,
  }
}

structure! {
  /// [VkSparseImageFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageFormatProperties.html)
  VkSparseImageFormatProperties {
    /// * **Optional:** true
    aspectMask: VkImageAspectFlags,
    imageGranularity: VkExtent3D,
    /// * **Optional:** true
    flags: VkSparseImageFormatFlags,
  }
}

structure! {
  /// [VkSparseImageFormatProperties2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageFormatProperties2.html)
  VkSparseImageFormatProperties2 {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    properties: VkSparseImageFormatProperties,
  }
}

structure! {
  /// [VkSparseImageMemoryBind](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageMemoryBind.html)
  VkSparseImageMemoryBind {
    subresource: VkImageSubresource,
    offset: VkOffset3D,
    extent: VkExtent3D,
    /// * **Optional:** true
    memory: VkDeviceMemory,
    /// Specified in bytes
    memoryOffset: VkDeviceSize,
    /// * **Optional:** true
    flags: VkSparseMemoryBindFlags,
  }
}

structure! {
  /// [VkSparseImageMemoryBindInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageMemoryBindInfo.html)
  VkSparseImageMemoryBindInfo {
    image: VkImage,
    bindCount: uint32_t,
    /// * **Len:** bindCount
    pBinds: *const VkSparseImageMemoryBind,
  }
}

structure! {
  /// [VkSparseImageMemoryRequirements](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageMemoryRequirements.html)
  VkSparseImageMemoryRequirements {
    formatProperties: VkSparseImageFormatProperties,
    imageMipTailFirstLod: uint32_t,
    /// Specified in bytes, must be a multiple of sparse block size in bytes / alignment
    imageMipTailSize: VkDeviceSize,
    /// Specified in bytes, must be a multiple of sparse block size in bytes / alignment
    imageMipTailOffset: VkDeviceSize,
    /// Specified in bytes, must be a multiple of sparse block size in bytes / alignment
    imageMipTailStride: VkDeviceSize,
  }
}

structure! {
  /// [VkSparseImageMemoryRequirements2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageMemoryRequirements2.html)
  VkSparseImageMemoryRequirements2 {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    memoryRequirements: VkSparseImageMemoryRequirements,
  }
}

structure! {
  /// [VkSparseImageOpaqueMemoryBindInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageOpaqueMemoryBindInfo.html)
  VkSparseImageOpaqueMemoryBindInfo {
    image: VkImage,
    bindCount: uint32_t,
    /// * **Len:** bindCount
    pBinds: *const VkSparseMemoryBind,
  }
}

structure! {
  /// [VkSparseMemoryBind](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseMemoryBind.html)
  VkSparseMemoryBind {
    /// Specified in bytes
    resourceOffset: VkDeviceSize,
    /// Specified in bytes
    size: VkDeviceSize,
    /// * **Optional:** true
    memory: VkDeviceMemory,
    /// Specified in bytes
    memoryOffset: VkDeviceSize,
    /// * **Optional:** true
    flags: VkSparseMemoryBindFlags,
  }
}

structure! {
  /// [VkSpecializationInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSpecializationInfo.html)
  VkSpecializationInfo {
    /// Number of entries in the map
    /// * **Optional:** true
    mapEntryCount: uint32_t,
    /// Array of map entries
    /// * **Len:** mapEntryCount
    pMapEntries: *const VkSpecializationMapEntry,
    /// Size in bytes of pData
    /// * **Optional:** true
    dataSize: size_t,
    /// Pointer to SpecConstant data
    /// * **Len:** dataSize
    pData: *const c_void,
  }
}

structure! {
  /// [VkSpecializationMapEntry](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSpecializationMapEntry.html)
  VkSpecializationMapEntry {
    /// The SpecConstant ID specified in the BIL
    constantID: uint32_t,
    /// Offset of the value in the data block
    offset: uint32_t,
    /// Size in bytes of the SpecConstant
    /// * **No Auto-validity:** true
    size: size_t,
  }
}

structure! {
  /// [VkStencilOpState](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStencilOpState.html)
  VkStencilOpState {
    failOp: VkStencilOp,
    passOp: VkStencilOp,
    depthFailOp: VkStencilOp,
    compareOp: VkCompareOp,
    compareMask: uint32_t,
    writeMask: uint32_t,
    reference: uint32_t,
  }
}

#[cfg(feature = "google_games_platform")]
structure! {
  /// [VkStreamDescriptorSurfaceCreateInfoGGP](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStreamDescriptorSurfaceCreateInfoGGP.html)
  VkStreamDescriptorSurfaceCreateInfoGGP {
    /// * **Values:** [`VK_STRUCTURE_TYPE_STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkStreamDescriptorSurfaceCreateFlagsGGP,
    streamDescriptor: GgpStreamDescriptor,
  }
}

structure! {
  /// [VkStridedDeviceAddressRegionKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStridedDeviceAddressRegionKHR.html)
  VkStridedDeviceAddressRegionKHR {
    /// * **Optional:** true
    deviceAddress: VkDeviceAddress,
    stride: VkDeviceSize,
    size: VkDeviceSize,
  }
}

structure! {
  /// [VkSubmitInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubmitInfo.html)
  VkSubmitInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SUBMIT_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    waitSemaphoreCount: uint32_t,
    /// * **Len:** waitSemaphoreCount
    pWaitSemaphores: *const VkSemaphore,
    /// * **Len:** waitSemaphoreCount
    pWaitDstStageMask: *const VkPipelineStageFlags,
    /// * **Optional:** true
    commandBufferCount: uint32_t,
    /// * **Len:** commandBufferCount
    pCommandBuffers: *const VkCommandBuffer,
    /// * **Optional:** true
    signalSemaphoreCount: uint32_t,
    /// * **Len:** signalSemaphoreCount
    pSignalSemaphores: *const VkSemaphore,
  }
}

structure! {
  /// [VkSubpassBeginInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassBeginInfo.html)
  VkSubpassBeginInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    contents: VkSubpassContents,
  }
}

structure! {
  /// [VkSubpassDependency](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDependency.html)
  VkSubpassDependency {
    srcSubpass: uint32_t,
    dstSubpass: uint32_t,
    srcStageMask: VkPipelineStageFlags,
    dstStageMask: VkPipelineStageFlags,
    /// Memory accesses from the source of the dependency to synchronize
    /// * **Optional:** true
    srcAccessMask: VkAccessFlags,
    /// Memory accesses from the destination of the dependency to synchronize
    /// * **Optional:** true
    dstAccessMask: VkAccessFlags,
    /// * **Optional:** true
    dependencyFlags: VkDependencyFlags,
  }
}

structure! {
  /// [VkSubpassDependency2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDependency2.html)
  VkSubpassDependency2 {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    srcSubpass: uint32_t,
    dstSubpass: uint32_t,
    srcStageMask: VkPipelineStageFlags,
    dstStageMask: VkPipelineStageFlags,
    /// * **Optional:** true
    srcAccessMask: VkAccessFlags,
    /// * **Optional:** true
    dstAccessMask: VkAccessFlags,
    /// * **Optional:** true
    dependencyFlags: VkDependencyFlags,
    viewOffset: int32_t,
  }
}

structure! {
  /// [VkSubpassDescription](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDescription.html)
  VkSubpassDescription {
    /// * **Optional:** true
    flags: VkSubpassDescriptionFlags,
    /// Must be VK_PIPELINE_BIND_POINT_GRAPHICS for now
    pipelineBindPoint: VkPipelineBindPoint,
    /// * **Optional:** true
    inputAttachmentCount: uint32_t,
    /// * **Len:** inputAttachmentCount
    pInputAttachments: *const VkAttachmentReference,
    /// * **Optional:** true
    colorAttachmentCount: uint32_t,
    /// * **Len:** colorAttachmentCount
    pColorAttachments: *const VkAttachmentReference,
    /// * **Optional:** true
    /// * **Len:** colorAttachmentCount
    pResolveAttachments: *const VkAttachmentReference,
    /// * **Optional:** true
    pDepthStencilAttachment: *const VkAttachmentReference,
    /// * **Optional:** true
    preserveAttachmentCount: uint32_t,
    /// * **Len:** preserveAttachmentCount
    pPreserveAttachments: *const uint32_t,
  }
}

structure! {
  /// [VkSubpassDescription2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDescription2.html)
  VkSubpassDescription2 {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkSubpassDescriptionFlags,
    pipelineBindPoint: VkPipelineBindPoint,
    viewMask: uint32_t,
    /// * **Optional:** true
    inputAttachmentCount: uint32_t,
    /// * **Len:** inputAttachmentCount
    pInputAttachments: *const VkAttachmentReference2,
    /// * **Optional:** true
    colorAttachmentCount: uint32_t,
    /// * **Len:** colorAttachmentCount
    pColorAttachments: *const VkAttachmentReference2,
    /// * **Optional:** true
    /// * **Len:** colorAttachmentCount
    pResolveAttachments: *const VkAttachmentReference2,
    /// * **Optional:** true
    pDepthStencilAttachment: *const VkAttachmentReference2,
    /// * **Optional:** true
    preserveAttachmentCount: uint32_t,
    /// * **Len:** preserveAttachmentCount
    pPreserveAttachments: *const uint32_t,
  }
}

structure! {
  /// [VkSubpassDescriptionDepthStencilResolve](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDescriptionDepthStencilResolve.html)
  ///
  /// Struct Extends: [`VkSubpassDescription2`]
  VkSubpassDescriptionDepthStencilResolve {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// depth resolve mode
    /// * **No Auto-validity:** true
    depthResolveMode: VkResolveModeFlagBits,
    /// stencil resolve mode
    /// * **No Auto-validity:** true
    stencilResolveMode: VkResolveModeFlagBits,
    /// depth/stencil resolve attachment
    /// * **Optional:** true
    pDepthStencilResolveAttachment: *const VkAttachmentReference2,
  }
}

structure! {
  /// [VkSubpassEndInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassEndInfo.html)
  VkSubpassEndInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SUBPASS_END_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
  }
}

structure! {
  /// [VkSubpassSampleLocationsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassSampleLocationsEXT.html)
  VkSubpassSampleLocationsEXT {
    subpassIndex: uint32_t,
    sampleLocationsInfo: VkSampleLocationsInfoEXT,
  }
}

structure! {
  /// [VkSubresourceLayout](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubresourceLayout.html)
  VkSubresourceLayout {
    /// Specified in bytes
    offset: VkDeviceSize,
    /// Specified in bytes
    size: VkDeviceSize,
    /// Specified in bytes
    rowPitch: VkDeviceSize,
    /// Specified in bytes
    arrayPitch: VkDeviceSize,
    /// Specified in bytes
    depthPitch: VkDeviceSize,
  }
}

structure! {
  /// [VkSurfaceCapabilities2EXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCapabilities2EXT.html)
  VkSurfaceCapabilities2EXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    /// Supported minimum number of images for the surface
    minImageCount: uint32_t,
    /// Supported maximum number of images for the surface, 0 for unlimited
    maxImageCount: uint32_t,
    /// Current image width and height for the surface, (0, 0) if undefined
    currentExtent: VkExtent2D,
    /// Supported minimum image width and height for the surface
    minImageExtent: VkExtent2D,
    /// Supported maximum image width and height for the surface
    maxImageExtent: VkExtent2D,
    /// Supported maximum number of image layers for the surface
    maxImageArrayLayers: uint32_t,
    /// 1 or more bits representing the transforms supported
    /// * **Optional:** true
    supportedTransforms: VkSurfaceTransformFlagsKHR,
    /// The surface's current transform relative to the device's natural orientation
    currentTransform: VkSurfaceTransformFlagBitsKHR,
    /// 1 or more bits representing the alpha compositing modes supported
    /// * **Optional:** true
    supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
    /// Supported image usage flags for the surface
    /// * **Optional:** true
    supportedUsageFlags: VkImageUsageFlags,
    /// * **Optional:** true
    supportedSurfaceCounters: VkSurfaceCounterFlagsEXT,
  }
}

structure! {
  /// [VkSurfaceCapabilities2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCapabilities2KHR.html)
  VkSurfaceCapabilities2KHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    surfaceCapabilities: VkSurfaceCapabilitiesKHR,
  }
}

structure! {
  /// [VkSurfaceCapabilitiesFullScreenExclusiveEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCapabilitiesFullScreenExclusiveEXT.html)
  ///
  /// Struct Extends: [`VkSurfaceCapabilities2KHR`]
  VkSurfaceCapabilitiesFullScreenExclusiveEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    fullScreenExclusiveSupported: VkBool32,
  }
}

structure! {
  /// [VkSurfaceCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCapabilitiesKHR.html)
  VkSurfaceCapabilitiesKHR {
    /// Supported minimum number of images for the surface
    minImageCount: uint32_t,
    /// Supported maximum number of images for the surface, 0 for unlimited
    maxImageCount: uint32_t,
    /// Current image width and height for the surface, (0, 0) if undefined
    currentExtent: VkExtent2D,
    /// Supported minimum image width and height for the surface
    minImageExtent: VkExtent2D,
    /// Supported maximum image width and height for the surface
    maxImageExtent: VkExtent2D,
    /// Supported maximum number of image layers for the surface
    maxImageArrayLayers: uint32_t,
    /// 1 or more bits representing the transforms supported
    /// * **Optional:** true
    supportedTransforms: VkSurfaceTransformFlagsKHR,
    /// The surface's current transform relative to the device's natural orientation
    currentTransform: VkSurfaceTransformFlagBitsKHR,
    /// 1 or more bits representing the alpha compositing modes supported
    /// * **Optional:** true
    supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
    /// Supported image usage flags for the surface
    /// * **Optional:** true
    supportedUsageFlags: VkImageUsageFlags,
  }
}

structure! {
  /// [VkSurfaceFormat2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceFormat2KHR.html)
  VkSurfaceFormat2KHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    surfaceFormat: VkSurfaceFormatKHR,
  }
}

structure! {
  /// [VkSurfaceFormatKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceFormatKHR.html)
  VkSurfaceFormatKHR {
    /// Supported pair of rendering format
    format: VkFormat,
    /// and color space for the surface
    colorSpace: VkColorSpaceKHR,
  }
}

structure! {
  /// [VkSurfaceFullScreenExclusiveInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceFullScreenExclusiveInfoEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceSurfaceInfo2KHR`], [`VkSwapchainCreateInfoKHR`]
  VkSurfaceFullScreenExclusiveInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    fullScreenExclusive: VkFullScreenExclusiveEXT,
  }
}

structure! {
  /// [VkSurfaceFullScreenExclusiveWin32InfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceFullScreenExclusiveWin32InfoEXT.html)
  ///
  /// Struct Extends: [`VkPhysicalDeviceSurfaceInfo2KHR`], [`VkSwapchainCreateInfoKHR`]
  VkSurfaceFullScreenExclusiveWin32InfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    hmonitor: HMONITOR,
  }
}

structure! {
  /// [VkSurfaceProtectedCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceProtectedCapabilitiesKHR.html)
  ///
  /// Struct Extends: [`VkSurfaceCapabilities2KHR`]
  VkSurfaceProtectedCapabilitiesKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Represents if surface can be protected
    supportsProtected: VkBool32,
  }
}

structure! {
  /// [VkSwapchainCounterCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainCounterCreateInfoEXT.html)
  ///
  /// Struct Extends: [`VkSwapchainCreateInfoKHR`]
  VkSwapchainCounterCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    surfaceCounters: VkSurfaceCounterFlagsEXT,
  }
}

structure! {
  /// [VkSwapchainCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainCreateInfoKHR.html)
  VkSwapchainCreateInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkSwapchainCreateFlagsKHR,
    /// The swapchain's target surface
    surface: VkSurfaceKHR,
    /// Minimum number of presentation images the application needs
    minImageCount: uint32_t,
    /// Format of the presentation images
    imageFormat: VkFormat,
    /// Colorspace of the presentation images
    imageColorSpace: VkColorSpaceKHR,
    /// Dimensions of the presentation images
    imageExtent: VkExtent2D,
    /// Determines the number of views for multiview/stereo presentation
    imageArrayLayers: uint32_t,
    /// Bits indicating how the presentation images will be used
    imageUsage: VkImageUsageFlags,
    /// Sharing mode used for the presentation images
    imageSharingMode: VkSharingMode,
    /// Number of queue families having access to the images in case of concurrent sharing mode
    /// * **Optional:** true
    queueFamilyIndexCount: uint32_t,
    /// Array of queue family indices having access to the images in case of concurrent sharing mode
    /// * **No Auto-validity:** true
    /// * **Len:** queueFamilyIndexCount
    pQueueFamilyIndices: *const uint32_t,
    /// The transform, relative to the device's natural orientation, applied to the image content prior to presentation
    preTransform: VkSurfaceTransformFlagBitsKHR,
    /// The alpha blending mode used when compositing this surface with other surfaces in the window system
    compositeAlpha: VkCompositeAlphaFlagBitsKHR,
    /// Which presentation mode to use for presents on this swap chain
    presentMode: VkPresentModeKHR,
    /// Specifies whether presentable images may be affected by window clip regions
    clipped: VkBool32,
    /// Existing swap chain to replace, if any
    /// * **Optional:** true
    oldSwapchain: VkSwapchainKHR,
  }
}

structure! {
  /// [VkSwapchainDisplayNativeHdrCreateInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainDisplayNativeHdrCreateInfoAMD.html)
  ///
  /// Struct Extends: [`VkSwapchainCreateInfoKHR`]
  VkSwapchainDisplayNativeHdrCreateInfoAMD {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    localDimmingEnable: VkBool32,
  }
}

#[cfg(feature = "VK_ANDROID_native_buffer")]
structure! {
  /// [VkSwapchainImageCreateInfoANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainImageCreateInfoANDROID.html)
  VkSwapchainImageCreateInfoANDROID {
    /// * **Values:** [`VK_STRUCTURE_TYPE_SWAPCHAIN_IMAGE_CREATE_INFO_ANDROID`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    usage: VkSwapchainImageUsageFlagsANDROID,
  }
}

structure! {
  /// [VkTextureLODGatherFormatPropertiesAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTextureLODGatherFormatPropertiesAMD.html)
  ///
  /// Struct Extends: [`VkImageFormatProperties2`]
  VkTextureLODGatherFormatPropertiesAMD {
    /// * **Values:** [`VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *mut c_void,
    supportsTextureGatherLODBiasAMD: VkBool32,
  }
}

structure! {
  /// [VkTimelineSemaphoreSubmitInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTimelineSemaphoreSubmitInfo.html)
  ///
  /// Struct Extends: [`VkSubmitInfo`], [`VkBindSparseInfo`]
  VkTimelineSemaphoreSubmitInfo {
    /// * **Values:** [`VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    waitSemaphoreValueCount: uint32_t,
    /// * **Optional:** true
    /// * **Len:** waitSemaphoreValueCount
    pWaitSemaphoreValues: *const uint64_t,
    /// * **Optional:** true
    signalSemaphoreValueCount: uint32_t,
    /// * **Optional:** true
    /// * **Len:** signalSemaphoreValueCount
    pSignalSemaphoreValues: *const uint64_t,
  }
}

structure! {
  /// [VkTraceRaysIndirectCommandKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTraceRaysIndirectCommandKHR.html)
  VkTraceRaysIndirectCommandKHR {
    width: uint32_t,
    height: uint32_t,
    depth: uint32_t,
  }
}

structure! {
  /// [VkTransformMatrixKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTransformMatrixKHR.html)
  VkTransformMatrixKHR {
    matrix: [[float; 4]; 3],
  }
}

structure! {
  /// [VkValidationCacheCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationCacheCreateInfoEXT.html)
  VkValidationCacheCreateInfoEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkValidationCacheCreateFlagsEXT,
    /// * **Optional:** true
    initialDataSize: size_t,
    /// * **Len:** initialDataSize
    pInitialData: *const c_void,
  }
}

structure! {
  /// [VkValidationFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationFeaturesEXT.html)
  ///
  /// Struct Extends: [`VkInstanceCreateInfo`]
  VkValidationFeaturesEXT {
    /// Must be VK_STRUCTURE_TYPE_VALIDATION_FEATURES_EXT
    /// * **Values:** [`VK_STRUCTURE_TYPE_VALIDATION_FEATURES_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Number of validation features to enable
    /// * **Optional:** true
    enabledValidationFeatureCount: uint32_t,
    /// Validation features to enable
    /// * **Len:** enabledValidationFeatureCount
    pEnabledValidationFeatures: *const VkValidationFeatureEnableEXT,
    /// Number of validation features to disable
    /// * **Optional:** true
    disabledValidationFeatureCount: uint32_t,
    /// Validation features to disable
    /// * **Len:** disabledValidationFeatureCount
    pDisabledValidationFeatures: *const VkValidationFeatureDisableEXT,
  }
}

structure! {
  /// [VkValidationFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationFlagsEXT.html)
  ///
  /// Struct Extends: [`VkInstanceCreateInfo`]
  VkValidationFlagsEXT {
    /// Must be VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT
    /// * **Values:** [`VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Number of validation checks to disable
    disabledValidationCheckCount: uint32_t,
    /// Validation checks to disable
    /// * **Len:** disabledValidationCheckCount
    pDisabledValidationChecks: *const VkValidationCheckEXT,
  }
}

structure! {
  /// [VkVertexInputAttributeDescription](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputAttributeDescription.html)
  VkVertexInputAttributeDescription {
    /// location of the shader vertex attrib
    location: uint32_t,
    /// Vertex buffer binding id
    binding: uint32_t,
    /// format of source data
    format: VkFormat,
    /// Offset of first element in bytes from base of vertex
    offset: uint32_t,
  }
}

structure! {
  /// [VkVertexInputBindingDescription](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputBindingDescription.html)
  VkVertexInputBindingDescription {
    /// Vertex buffer binding id
    binding: uint32_t,
    /// Distance between vertices in bytes (0 = no advancement)
    stride: uint32_t,
    /// The rate at which the vertex data is consumed
    inputRate: VkVertexInputRate,
  }
}

structure! {
  /// [VkVertexInputBindingDivisorDescriptionEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputBindingDivisorDescriptionEXT.html)
  VkVertexInputBindingDivisorDescriptionEXT {
    binding: uint32_t,
    divisor: uint32_t,
  }
}

structure! {
  /// [VkViSurfaceCreateInfoNN](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViSurfaceCreateInfoNN.html)
  VkViSurfaceCreateInfoNN {
    /// * **Values:** [`VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkViSurfaceCreateFlagsNN,
    /// * **No Auto-validity:** true
    window: *mut c_void,
  }
}

structure! {
  /// [VkViewportSwizzleNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViewportSwizzleNV.html)
  VkViewportSwizzleNV {
    x: VkViewportCoordinateSwizzleNV,
    y: VkViewportCoordinateSwizzleNV,
    z: VkViewportCoordinateSwizzleNV,
    w: VkViewportCoordinateSwizzleNV,
  }
}

structure! {
  /// [VkViewportWScalingNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViewportWScalingNV.html)
  VkViewportWScalingNV {
    xcoeff: float,
    ycoeff: float,
  }
}

structure! {
  /// [VkWaylandSurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWaylandSurfaceCreateInfoKHR.html)
  VkWaylandSurfaceCreateInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkWaylandSurfaceCreateFlagsKHR,
    /// * **No Auto-validity:** true
    display: *mut wl_display,
    /// * **No Auto-validity:** true
    surface: *mut wl_surface,
  }
}

structure! {
  /// [VkWin32KeyedMutexAcquireReleaseInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWin32KeyedMutexAcquireReleaseInfoKHR.html)
  ///
  /// Struct Extends: [`VkSubmitInfo`]
  VkWin32KeyedMutexAcquireReleaseInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    acquireCount: uint32_t,
    /// * **Len:** acquireCount
    pAcquireSyncs: *const VkDeviceMemory,
    /// * **Len:** acquireCount
    pAcquireKeys: *const uint64_t,
    /// * **Len:** acquireCount
    pAcquireTimeouts: *const uint32_t,
    /// * **Optional:** true
    releaseCount: uint32_t,
    /// * **Len:** releaseCount
    pReleaseSyncs: *const VkDeviceMemory,
    /// * **Len:** releaseCount
    pReleaseKeys: *const uint64_t,
  }
}

structure! {
  /// [VkWin32KeyedMutexAcquireReleaseInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWin32KeyedMutexAcquireReleaseInfoNV.html)
  ///
  /// Struct Extends: [`VkSubmitInfo`]
  VkWin32KeyedMutexAcquireReleaseInfoNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    acquireCount: uint32_t,
    /// * **Len:** acquireCount
    pAcquireSyncs: *const VkDeviceMemory,
    /// * **Len:** acquireCount
    pAcquireKeys: *const uint64_t,
    /// * **Len:** acquireCount
    pAcquireTimeoutMilliseconds: *const uint32_t,
    /// * **Optional:** true
    releaseCount: uint32_t,
    /// * **Len:** releaseCount
    pReleaseSyncs: *const VkDeviceMemory,
    /// * **Len:** releaseCount
    pReleaseKeys: *const uint64_t,
  }
}

structure! {
  /// [VkWin32SurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWin32SurfaceCreateInfoKHR.html)
  VkWin32SurfaceCreateInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkWin32SurfaceCreateFlagsKHR,
    hinstance: HINSTANCE,
    hwnd: HWND,
  }
}

structure! {
  /// [VkWriteDescriptorSet](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWriteDescriptorSet.html)
  VkWriteDescriptorSet {
    /// * **Values:** [`VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// Destination descriptor set
    /// * **No Auto-validity:** true
    dstSet: VkDescriptorSet,
    /// Binding within the destination descriptor set to write
    dstBinding: uint32_t,
    /// Array element within the destination binding to write
    dstArrayElement: uint32_t,
    /// Number of descriptors to write (determines the size of the array pointed by pDescriptors)
    descriptorCount: uint32_t,
    /// Descriptor type to write (determines which members of the array pointed by pDescriptors are going to be used)
    descriptorType: VkDescriptorType,
    /// Sampler, image view, and layout for SAMPLER, COMBINED_IMAGE_SAMPLER, {SAMPLED,STORAGE}_IMAGE, and INPUT_ATTACHMENT descriptor types.
    /// * **No Auto-validity:** true
    /// * **Len:** descriptorCount
    pImageInfo: *const VkDescriptorImageInfo,
    /// Raw buffer, size, and offset for {UNIFORM,STORAGE}_BUFFER{_DYNAMIC} descriptor types.
    /// * **No Auto-validity:** true
    /// * **Len:** descriptorCount
    pBufferInfo: *const VkDescriptorBufferInfo,
    /// Buffer view to write to the descriptor for {UNIFORM,STORAGE}_TEXEL_BUFFER descriptor types.
    /// * **No Auto-validity:** true
    /// * **Len:** descriptorCount
    pTexelBufferView: *const VkBufferView,
  }
}

structure! {
  /// [VkWriteDescriptorSetAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWriteDescriptorSetAccelerationStructureKHR.html)
  ///
  /// Struct Extends: [`VkWriteDescriptorSet`]
  VkWriteDescriptorSetAccelerationStructureKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    accelerationStructureCount: uint32_t,
    /// * **Optional:** false,true
    /// * **Len:** accelerationStructureCount
    pAccelerationStructures: *const VkAccelerationStructureKHR,
  }
}

structure! {
  /// [VkWriteDescriptorSetAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWriteDescriptorSetAccelerationStructureNV.html)
  ///
  /// Struct Extends: [`VkWriteDescriptorSet`]
  VkWriteDescriptorSetAccelerationStructureNV {
    /// * **Values:** [`VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    accelerationStructureCount: uint32_t,
    /// * **Optional:** false,true
    /// * **Len:** accelerationStructureCount
    pAccelerationStructures: *const VkAccelerationStructureNV,
  }
}

structure! {
  /// [VkWriteDescriptorSetInlineUniformBlockEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWriteDescriptorSetInlineUniformBlockEXT.html)
  ///
  /// Struct Extends: [`VkWriteDescriptorSet`]
  VkWriteDescriptorSetInlineUniformBlockEXT {
    /// * **Values:** [`VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    dataSize: uint32_t,
    /// * **Len:** dataSize
    pData: *const c_void,
  }
}

structure! {
  /// [VkXYColorEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXYColorEXT.html)
  VkXYColorEXT {
    x: float,
    y: float,
  }
}

structure! {
  /// [VkXcbSurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXcbSurfaceCreateInfoKHR.html)
  VkXcbSurfaceCreateInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkXcbSurfaceCreateFlagsKHR,
    /// * **No Auto-validity:** true
    connection: *mut xcb_connection_t,
    window: xcb_window_t,
  }
}

structure! {
  /// [VkXlibSurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXlibSurfaceCreateInfoKHR.html)
  VkXlibSurfaceCreateInfoKHR {
    /// * **Values:** [`VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR`]
    sType: VkStructureType,
    /// * **Optional:** true
    pNext: *const c_void,
    /// * **Optional:** true
    flags: VkXlibSurfaceCreateFlagsKHR,
    /// * **No Auto-validity:** true
    dpy: *mut Display,
    window: Window,
  }
}
