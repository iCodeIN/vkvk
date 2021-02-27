#![allow(non_snake_case)]

//! Structure types to interface with Vulkan.

use super::*;

use core::mem::zeroed;

/// [VkDeviceAddress](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceAddress.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkDeviceAddress(pub uint64_t);

/// [VkDeviceSize](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceSize.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkDeviceSize(pub uint64_t);

/// [VkExtent2D](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExtent2D.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkExtent2D {
  pub width: uint32_t,
  pub height: uint32_t,
}

/// [VkExtent3D](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExtent3D.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkExtent3D {
  pub width: uint32_t,
  pub height: uint32_t,
  pub depth: uint32_t,
}

/// [VkOffset2D](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkOffset2D.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkOffset2D {
  pub x: int32_t,
  pub y: int32_t,
}

/// [VkOffset3D](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkOffset3D.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkOffset3D {
  pub x: int32_t,
  pub y: int32_t,
  pub z: int32_t,
}

/// [VkRect2D](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRect2D.html)
///
/// The `extent` forms the *exclusive* edges of the rectangle.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkRect2D {
  pub offset: VkOffset2D,
  pub extent: VkExtent2D,
}

/// [VkBaseInStructure](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBaseInStructure.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkBaseInStructure {
  pub sType: VkStructureType,
  pub pNext: *const VkBaseInStructure,
}
impl Default for VkBaseInStructure {
  fn default() -> Self {
    unsafe { zeroed() }
  }
}

/// [VkBaseOutStructure](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBaseOutStructure.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkBaseOutStructure {
  pub sType: VkStructureType,
  pub pNext: *mut VkBaseOutStructure,
}
impl Default for VkBaseOutStructure {
  fn default() -> Self {
    unsafe { zeroed() }
  }
}

/// [VkBufferMemoryBarrier](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferMemoryBarrier.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkBufferMemoryBarrier {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER`]
  pub sType: VkStructureType,
  ///
  /// * **Optional:** true
  pub pNext: *const c_void,
  /// Memory accesses from the source of the dependency to synchronize
  /// * **No Auto-validity:** true
  pub srcAccessMask: VkAccessFlags,
  /// Memory accesses from the destination of the dependency to synchronize
  /// * **No Auto-validity:** true
  pub dstAccessMask: VkAccessFlags,
  /// Queue family to transition ownership from
  pub srcQueueFamilyIndex: uint32_t,
  /// Queue family to transition ownership to
  pub dstQueueFamilyIndex: uint32_t,
  /// Buffer to sync
  pub buffer: VkBuffer,
  /// Offset within the buffer to sync
  pub offset: VkDeviceSize,
  /// Amount of bytes to sync
  pub size: VkDeviceSize,
}
impl Default for VkBufferMemoryBarrier {
  fn default() -> Self {
    VkBufferMemoryBarrier { sType: VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER, ..unsafe { zeroed() } }
  }
}

/// [VkDispatchIndirectCommand](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDispatchIndirectCommand.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkDispatchIndirectCommand {
  ///
  /// * **No Auto-validity:** true
  pub x: uint32_t,
  ///
  /// * **No Auto-validity:** true
  pub y: uint32_t,
  ///
  /// * **No Auto-validity:** true
  pub z: uint32_t,
}

/// [VkDrawIndexedIndirectCommand](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDrawIndexedIndirectCommand.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkDrawIndexedIndirectCommand {
  pub indexCount: uint32_t,
  pub instanceCount: uint32_t,
  pub firstIndex: uint32_t,
  pub vertexOffset: int32_t,
  ///
  /// * **No Auto-validity:** true
  pub firstInstance: uint32_t,
}

/// [VkDrawIndirectCommand](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDrawIndirectCommand.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkDrawIndirectCommand {
  pub vertexCount: uint32_t,
  pub instanceCount: uint32_t,
  pub firstVertex: uint32_t,
  ///
  /// * **No Auto-validity:** true
  pub firstInstance: uint32_t,
}

/// [VkImageMemoryBarrier](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageMemoryBarrier.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkImageMemoryBarrier {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER`]
  pub sType: VkStructureType,
  ///
  /// * **Optional:** true
  pub pNext: *const c_void,
  /// Memory accesses from the source of the dependency to synchronize
  /// * **No Auto-validity:** true
  pub srcAccessMask: VkAccessFlags,
  /// Memory accesses from the destination of the dependency to synchronize
  /// * **No Auto-validity:** true
  pub dstAccessMask: VkAccessFlags,
  /// Current layout of the image
  pub oldLayout: VkImageLayout,
  /// New layout to transition the image to
  pub newLayout: VkImageLayout,
  /// Queue family to transition ownership from
  pub srcQueueFamilyIndex: uint32_t,
  /// Queue family to transition ownership to
  pub dstQueueFamilyIndex: uint32_t,
  /// Image to sync
  pub image: VkImage,
  /// Subresource range to sync
  pub subresourceRange: VkImageSubresourceRange,
}
impl Default for VkImageMemoryBarrier {
  fn default() -> Self {
    VkImageMemoryBarrier { sType: VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER, ..unsafe { zeroed() } }
  }
}

/// [VkMemoryBarrier](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryBarrier.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkMemoryBarrier {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_MEMORY_BARRIER`]
  pub sType: VkStructureType,
  ///
  /// * **Optional:** true
  pub pNext: *const c_void,
  /// Memory accesses from the source of the dependency to synchronize
  /// * **Optional:** true
  pub srcAccessMask: VkAccessFlags,
  /// Memory accesses from the destination of the dependency to synchronize
  /// * **Optional:** true
  pub dstAccessMask: VkAccessFlags,
}
impl Default for VkMemoryBarrier {
  fn default() -> Self {
    VkMemoryBarrier { sType: VK_STRUCTURE_TYPE_MEMORY_BARRIER, ..unsafe { zeroed() } }
  }
}

/// [VkImageSubresourceRange](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSubresourceRange.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkImageSubresourceRange {
  pub aspectMask: VkImageAspectFlags,
  pub baseMipLevel: uint32_t,
  pub levelCount: uint32_t,
  pub baseArrayLayer: uint32_t,
  pub layerCount: uint32_t,
}

/// [VkAllocationCallbacks](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAllocationCallbacks.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAllocationCallbacks {
  ///
  /// * Optional: true
  pub pUserData: *mut void,
  pub pfnAllocation: PFN_vkAllocationFunction,
  pub pfnReallocation: PFN_vkReallocationFunction,
  pub pfnFree: PFN_vkFreeFunction,
  ///
  /// * Optional: true
  pub pfnInternalAllocation: PFN_vkInternalAllocationNotification,
  ///
  /// * Optional: true
  pub pfnInternalFree: PFN_vkInternalFreeNotification,
}
// TODO: impl Debug for VkAllocationCallbacks
impl Default for VkAllocationCallbacks {
  fn default() -> Self {
    unsafe { zeroed() }
  }
}

/// [VkApplicationInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkApplicationInfo.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkApplicationInfo {
  ///
  /// * Values: [`VK_STRUCTURE_TYPE_APPLICATION_INFO`]
  pub sType: VkStructureType,
  ///
  /// * Optional: true
  pub pNext: *const void,
  ///
  /// * Optional: true
  /// * Len: null-terminated
  pub pApplicationName: *const char,
  pub applicationVersion: uint32_t,
  ///
  /// * Optional: true
  /// * Len: null-terminated
  pub pEngineName: *const char,
  pub engineVersion: uint32_t,
  pub apiVersion: VulkanVersion,
}
impl Default for VkApplicationInfo {
  fn default() -> Self {
    unsafe { zeroed() }
  }
}

/// [VkFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFormatProperties.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkFormatProperties {
  /// Format features in case of linear tiling
  /// * **Optional:** true
  pub linearTilingFeatures: VkFormatFeatureFlags,
  /// Format features in case of optimal tiling
  /// * **Optional:** true
  pub optimalTilingFeatures: VkFormatFeatureFlags,
  /// Format features supported by buffers
  /// * **Optional:** true
  pub bufferFeatures: VkFormatFeatureFlags,
}

/// [VkImageFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageFormatProperties.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkImageFormatProperties {
  /// max image dimensions for this resource type
  pub maxExtent: VkExtent3D,
  /// max number of mipmap levels for this resource type
  pub maxMipLevels: uint32_t,
  /// max array size for this resource type
  pub maxArrayLayers: uint32_t,
  /// supported sample counts for this resource type
  /// * **Optional:** true
  pub sampleCounts: VkSampleCountFlags,
  /// max size (in bytes) of this resource type
  pub maxResourceSize: VkDeviceSize,
}

/// [VkInstanceCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkInstanceCreateInfo.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkInstanceCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO`]
  pub sType: VkStructureType,
  ///
  /// * **Optional:** true
  pub pNext: *const c_void,
  ///
  /// * **Optional:** true
  pub flags: VkInstanceCreateFlags,
  ///
  /// * **Optional:** true
  pub pApplicationInfo: *const VkApplicationInfo,
  ///
  /// * **Optional:** true
  pub enabledLayerCount: uint32_t,
  /// Ordered list of layer names to be enabled
  /// * **Len:** enabledLayerCount,null-terminated
  pub ppEnabledLayerNames: *const u8,
  ///
  /// * **Optional:** true
  pub enabledExtensionCount: uint32_t,
  /// Extension names to be enabled
  /// * **Len:** enabledExtensionCount,null-terminated
  pub ppEnabledExtensionNames: *const u8,
}
impl Default for VkInstanceCreateInfo {
  fn default() -> Self {
    unsafe { zeroed() }
  }
}

/// [VkMemoryHeap](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryHeap.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkMemoryHeap {
  /// Available memory in the heap
  pub size: VkDeviceSize,
  /// Flags for the heap
  /// * **Optional:** true
  pub flags: VkMemoryHeapFlags,
}

/// [VkMemoryType](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryType.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkMemoryType {
  /// Memory properties of this memory type
  /// * **Optional:** true
  pub propertyFlags: VkMemoryPropertyFlags,
  /// Index of the memory heap allocations of this memory type are taken from
  pub heapIndex: uint32_t,
}

/// [VkPhysicalDeviceFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFeatures.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkPhysicalDeviceFeatures {
  /// out of bounds buffer accesses are well defined
  pub robustBufferAccess: VkBool32,
  /// full 32-bit range of indices for indexed draw calls
  pub fullDrawIndexUint32: VkBool32,
  /// image views which are arrays of cube maps
  pub imageCubeArray: VkBool32,
  /// blending operations are controlled per-attachment
  pub independentBlend: VkBool32,
  /// geometry stage
  pub geometryShader: VkBool32,
  /// tessellation control and evaluation stage
  pub tessellationShader: VkBool32,
  /// per-sample shading and interpolation
  pub sampleRateShading: VkBool32,
  /// blend operations which take two sources
  pub dualSrcBlend: VkBool32,
  /// logic operations
  pub logicOp: VkBool32,
  /// multi draw indirect
  pub multiDrawIndirect: VkBool32,
  /// indirect draws can use non-zero firstInstance
  pub drawIndirectFirstInstance: VkBool32,
  /// depth clamping
  pub depthClamp: VkBool32,
  /// depth bias clamping
  pub depthBiasClamp: VkBool32,
  /// point and wireframe fill modes
  pub fillModeNonSolid: VkBool32,
  /// depth bounds test
  pub depthBounds: VkBool32,
  /// lines with width greater than 1
  pub wideLines: VkBool32,
  /// points with size greater than 1
  pub largePoints: VkBool32,
  /// the fragment alpha component can be forced to maximum representable alpha
  /// value
  pub alphaToOne: VkBool32,
  /// viewport arrays
  pub multiViewport: VkBool32,
  /// anisotropic sampler filtering
  pub samplerAnisotropy: VkBool32,
  /// ETC texture compression formats
  pub textureCompressionETC2: VkBool32,
  /// ASTC LDR texture compression formats
  pub textureCompressionASTC_LDR: VkBool32,
  /// BC1-7 texture compressed formats
  pub textureCompressionBC: VkBool32,
  /// precise occlusion queries returning actual sample counts
  pub occlusionQueryPrecise: VkBool32,
  /// pipeline statistics query
  pub pipelineStatisticsQuery: VkBool32,
  /// stores and atomic ops on storage buffers and images are supported in
  /// vertex, tessellation, and geometry stages
  pub vertexPipelineStoresAndAtomics: VkBool32,
  /// stores and atomic ops on storage buffers and images are supported in the
  /// fragment stage
  pub fragmentStoresAndAtomics: VkBool32,
  /// tessellation and geometry stages can export point size
  pub shaderTessellationAndGeometryPointSize: VkBool32,
  /// image gather with run-time values and independent offsets
  pub shaderImageGatherExtended: VkBool32,
  /// the extended set of formats can be used for storage images
  pub shaderStorageImageExtendedFormats: VkBool32,
  /// multisample images can be used for storage images
  pub shaderStorageImageMultisample: VkBool32,
  /// read from storage image does not require format qualifier
  pub shaderStorageImageReadWithoutFormat: VkBool32,
  /// write to storage image does not require format qualifier
  pub shaderStorageImageWriteWithoutFormat: VkBool32,
  /// arrays of uniform buffers can be accessed with dynamically uniform indices
  pub shaderUniformBufferArrayDynamicIndexing: VkBool32,
  /// arrays of sampled images can be accessed with dynamically uniform indices
  pub shaderSampledImageArrayDynamicIndexing: VkBool32,
  /// arrays of storage buffers can be accessed with dynamically uniform indices
  pub shaderStorageBufferArrayDynamicIndexing: VkBool32,
  /// arrays of storage images can be accessed with dynamically uniform indices
  pub shaderStorageImageArrayDynamicIndexing: VkBool32,
  /// clip distance in shaders
  pub shaderClipDistance: VkBool32,
  /// cull distance in shaders
  pub shaderCullDistance: VkBool32,
  /// 64-bit floats (doubles) in shaders
  pub shaderFloat64: VkBool32,
  /// 64-bit integers in shaders
  pub shaderInt64: VkBool32,
  /// 16-bit integers in shaders
  pub shaderInt16: VkBool32,
  /// shader can use texture operations that return resource residency
  /// information (requires sparseNonResident support)
  pub shaderResourceResidency: VkBool32,
  /// shader can use texture operations that specify minimum resource LOD
  pub shaderResourceMinLod: VkBool32,
  /// Sparse resources support: Resource memory can be managed at opaque page
  /// level rather than object level
  pub sparseBinding: VkBool32,
  /// Sparse resources support: GPU can access partially resident buffers
  pub sparseResidencyBuffer: VkBool32,
  /// Sparse resources support: GPU can access partially resident 2D (non-MSAA
  /// non-depth/stencil) images
  pub sparseResidencyImage2D: VkBool32,
  /// Sparse resources support: GPU can access partially resident 3D images
  pub sparseResidencyImage3D: VkBool32,
  /// Sparse resources support: GPU can access partially resident MSAA 2D images
  /// with 2 samples
  pub sparseResidency2Samples: VkBool32,
  /// Sparse resources support: GPU can access partially resident MSAA 2D images
  /// with 4 samples
  pub sparseResidency4Samples: VkBool32,
  /// Sparse resources support: GPU can access partially resident MSAA 2D images
  /// with 8 samples
  pub sparseResidency8Samples: VkBool32,
  /// Sparse resources support: GPU can access partially resident MSAA 2D images
  /// with 16 samples
  pub sparseResidency16Samples: VkBool32,
  /// Sparse resources support: GPU can correctly access data aliased into
  /// multiple locations (opt-in)
  pub sparseResidencyAliased: VkBool32,
  /// multisample rate must be the same for all pipelines in a subpass
  pub variableMultisampleRate: VkBool32,
  /// Queries may be inherited from primary to secondary command buffers
  pub inheritedQueries: VkBool32,
}

/// [VkPhysicalDeviceLimits](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceLimits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(C)]
pub struct VkPhysicalDeviceLimits {
  /// max 1D image dimension
  pub maxImageDimension1D: uint32_t,
  /// max 2D image dimension
  pub maxImageDimension2D: uint32_t,
  /// max 3D image dimension
  pub maxImageDimension3D: uint32_t,
  /// max cubemap image dimension
  pub maxImageDimensionCube: uint32_t,
  /// max layers for image arrays
  pub maxImageArrayLayers: uint32_t,
  /// max texel buffer size (fstexels)
  pub maxTexelBufferElements: uint32_t,
  /// max uniform buffer range (bytes)
  pub maxUniformBufferRange: uint32_t,
  /// max storage buffer range (bytes)
  pub maxStorageBufferRange: uint32_t,
  /// max size of the push constants pool (bytes)
  pub maxPushConstantsSize: uint32_t,
  /// max number of device memory allocations supported
  pub maxMemoryAllocationCount: uint32_t,
  /// max number of samplers that can be allocated on a device
  pub maxSamplerAllocationCount: uint32_t,
  /// Granularity (in bytes) at which buffers and images can be bound to
  /// adjacent memory for simultaneous usage
  pub bufferImageGranularity: VkDeviceSize,
  /// Total address space available for sparse allocations (bytes)
  pub sparseAddressSpaceSize: VkDeviceSize,
  /// max number of descriptors sets that can be bound to a pipeline
  pub maxBoundDescriptorSets: uint32_t,
  /// max number of samplers allowed per-stage in a descriptor set
  pub maxPerStageDescriptorSamplers: uint32_t,
  /// max number of uniform buffers allowed per-stage in a descriptor set
  pub maxPerStageDescriptorUniformBuffers: uint32_t,
  /// max number of storage buffers allowed per-stage in a descriptor set
  pub maxPerStageDescriptorStorageBuffers: uint32_t,
  /// max number of sampled images allowed per-stage in a descriptor set
  pub maxPerStageDescriptorSampledImages: uint32_t,
  /// max number of storage images allowed per-stage in a descriptor set
  pub maxPerStageDescriptorStorageImages: uint32_t,
  /// max number of input attachments allowed per-stage in a descriptor set
  pub maxPerStageDescriptorInputAttachments: uint32_t,
  /// max number of resources allowed by a single stage
  pub maxPerStageResources: uint32_t,
  /// max number of samplers allowed in all stages in a descriptor set
  pub maxDescriptorSetSamplers: uint32_t,
  /// max number of uniform buffers allowed in all stages in a descriptor set
  pub maxDescriptorSetUniformBuffers: uint32_t,
  /// max number of dynamic uniform buffers allowed in all stages in a
  /// descriptor set
  pub maxDescriptorSetUniformBuffersDynamic: uint32_t,
  /// max number of storage buffers allowed in all stages in a descriptor set
  pub maxDescriptorSetStorageBuffers: uint32_t,
  /// max number of dynamic storage buffers allowed in all stages in a
  /// descriptor set
  pub maxDescriptorSetStorageBuffersDynamic: uint32_t,
  /// max number of sampled images allowed in all stages in a descriptor set
  pub maxDescriptorSetSampledImages: uint32_t,
  /// max number of storage images allowed in all stages in a descriptor set
  pub maxDescriptorSetStorageImages: uint32_t,
  /// max number of input attachments allowed in all stages in a descriptor set
  pub maxDescriptorSetInputAttachments: uint32_t,
  /// max number of vertex input attribute slots
  pub maxVertexInputAttributes: uint32_t,
  /// max number of vertex input binding slots
  pub maxVertexInputBindings: uint32_t,
  /// max vertex input attribute offset added to vertex buffer offset
  pub maxVertexInputAttributeOffset: uint32_t,
  /// max vertex input binding stride
  pub maxVertexInputBindingStride: uint32_t,
  /// max number of output components written by vertex shader
  pub maxVertexOutputComponents: uint32_t,
  /// max level supported by tessellation primitive generator
  pub maxTessellationGenerationLevel: uint32_t,
  /// max patch size (vertices)
  pub maxTessellationPatchSize: uint32_t,
  /// max number of input components per-vertex in TCS
  pub maxTessellationControlPerVertexInputComponents: uint32_t,
  /// max number of output components per-vertex in TCS
  pub maxTessellationControlPerVertexOutputComponents: uint32_t,
  /// max number of output components per-patch in TCS
  pub maxTessellationControlPerPatchOutputComponents: uint32_t,
  /// max total number of per-vertex and per-patch output components in TCS
  pub maxTessellationControlTotalOutputComponents: uint32_t,
  /// max number of input components per vertex in TES
  pub maxTessellationEvaluationInputComponents: uint32_t,
  /// max number of output components per vertex in TES
  pub maxTessellationEvaluationOutputComponents: uint32_t,
  /// max invocation count supported in geometry shader
  pub maxGeometryShaderInvocations: uint32_t,
  /// max number of input components read in geometry stage
  pub maxGeometryInputComponents: uint32_t,
  /// max number of output components written in geometry stage
  pub maxGeometryOutputComponents: uint32_t,
  /// max number of vertices that can be emitted in geometry stage
  pub maxGeometryOutputVertices: uint32_t,
  /// max total number of components (all vertices) written in geometry stage
  pub maxGeometryTotalOutputComponents: uint32_t,
  /// max number of input components read in fragment stage
  pub maxFragmentInputComponents: uint32_t,
  /// max number of output attachments written in fragment stage
  pub maxFragmentOutputAttachments: uint32_t,
  /// max number of output attachments written when using dual source blending
  pub maxFragmentDualSrcAttachments: uint32_t,
  /// max total number of storage buffers, storage images and output buffers
  pub maxFragmentCombinedOutputResources: uint32_t,
  /// max total storage size of work group local storage (bytes)
  pub maxComputeSharedMemorySize: uint32_t,
  /// max num of compute work groups that may be dispatched by a single command
  /// (x,y,z)
  pub maxComputeWorkGroupCount: [uint32_t; 3],
  /// max total compute invocations in a single local work group
  pub maxComputeWorkGroupInvocations: uint32_t,
  /// max local size of a compute work group (x,y,z)
  pub maxComputeWorkGroupSize: [uint32_t; 3],
  /// number bits of subpixel precision in screen x and y
  pub subPixelPrecisionBits: uint32_t,
  /// number bits of precision for selecting texel weights
  pub subTexelPrecisionBits: uint32_t,
  /// number bits of precision for selecting mipmap weights
  pub mipmapPrecisionBits: uint32_t,
  /// max index value for indexed draw calls (for 32-bit indices)
  pub maxDrawIndexedIndexValue: uint32_t,
  /// max draw count for indirect draw calls
  pub maxDrawIndirectCount: uint32_t,
  /// max absolute sampler LOD bias
  pub maxSamplerLodBias: float,
  /// max degree of sampler anisotropy
  pub maxSamplerAnisotropy: float,
  /// max number of active viewports
  pub maxViewports: uint32_t,
  /// max viewport dimensions (x,y)
  pub maxViewportDimensions: [uint32_t; 2],
  /// viewport bounds range (min,max)
  pub viewportBoundsRange: [float; 2],
  /// number bits of subpixel precision for viewport
  pub viewportSubPixelBits: uint32_t,
  /// min required alignment of pointers returned by MapMemory (bytes)
  pub minMemoryMapAlignment: size_t,
  /// min required alignment for texel buffer offsets (bytes)
  pub minTexelBufferOffsetAlignment: VkDeviceSize,
  /// min required alignment for uniform buffer sizes and offsets (bytes)
  pub minUniformBufferOffsetAlignment: VkDeviceSize,
  /// min required alignment for storage buffer offsets (bytes)
  pub minStorageBufferOffsetAlignment: VkDeviceSize,
  /// min texel offset for OpTextureSampleOffset
  pub minTexelOffset: int32_t,
  /// max texel offset for OpTextureSampleOffset
  pub maxTexelOffset: uint32_t,
  /// min texel offset for OpTextureGatherOffset
  pub minTexelGatherOffset: int32_t,
  /// max texel offset for OpTextureGatherOffset
  pub maxTexelGatherOffset: uint32_t,
  /// furthest negative offset for interpolateAtOffset
  pub minInterpolationOffset: float,
  /// furthest positive offset for interpolateAtOffset
  pub maxInterpolationOffset: float,
  /// number of subpixel bits for interpolateAtOffset
  pub subPixelInterpolationOffsetBits: uint32_t,
  /// max width for a framebuffer
  pub maxFramebufferWidth: uint32_t,
  /// max height for a framebuffer
  pub maxFramebufferHeight: uint32_t,
  /// max layer count for a layered framebuffer
  pub maxFramebufferLayers: uint32_t,
  /// supported color sample counts for a framebuffer
  /// * Optional: true
  pub framebufferColorSampleCounts: VkSampleCountFlags,
  /// supported depth sample counts for a framebuffer
  /// * Optional: true
  pub framebufferDepthSampleCounts: VkSampleCountFlags,
  /// supported stencil sample counts for a framebuffer
  /// * Optional: true
  pub framebufferStencilSampleCounts: VkSampleCountFlags,
  /// supported sample counts for a subpass which uses no attachments
  /// * Optional: true
  pub framebufferNoAttachmentsSampleCounts: VkSampleCountFlags,
  /// max number of color attachments per subpass
  pub maxColorAttachments: uint32_t,
  /// supported color sample counts for a non-integer sampled image
  /// * Optional: true
  pub sampledImageColorSampleCounts: VkSampleCountFlags,
  /// supported sample counts for an integer image
  /// * Optional: true
  pub sampledImageIntegerSampleCounts: VkSampleCountFlags,
  /// supported depth sample counts for a sampled image
  /// * Optional: true
  pub sampledImageDepthSampleCounts: VkSampleCountFlags,
  /// supported stencil sample counts for a sampled image
  /// * Optional: true
  pub sampledImageStencilSampleCounts: VkSampleCountFlags,
  /// supported sample counts for a storage image
  /// * Optional: true
  pub storageImageSampleCounts: VkSampleCountFlags,
  /// max number of sample mask words
  pub maxSampleMaskWords: uint32_t,
  /// timestamps on graphics and compute queues
  pub timestampComputeAndGraphics: VkBool32,
  /// number of nanoseconds it takes for timestamp query value to increment by 1
  pub timestampPeriod: float,
  /// max number of clip distances
  pub maxClipDistances: uint32_t,
  /// max number of cull distances
  pub maxCullDistances: uint32_t,
  /// max combined number of user clipping
  pub maxCombinedClipAndCullDistances: uint32_t,
  /// distinct queue priorities available
  pub discreteQueuePriorities: uint32_t,
  /// range (min,max) of supported point sizes
  pub pointSizeRange: [float; 2],
  /// range (min,max) of supported line widths
  pub lineWidthRange: [float; 2],
  /// granularity of supported point sizes
  pub pointSizeGranularity: float,
  /// granularity of supported line widths
  pub lineWidthGranularity: float,
  /// line rasterization follows preferred rules
  pub strictLines: VkBool32,
  /// supports standard sample locations for all supported sample counts
  pub standardSampleLocations: VkBool32,
  /// optimal offset of buffer copies
  pub optimalBufferCopyOffsetAlignment: VkDeviceSize,
  /// optimal pitch of buffer copies
  pub optimalBufferCopyRowPitchAlignment: VkDeviceSize,
  /// minimum size and alignment for non-coherent host-mapped device memory
  /// access
  pub nonCoherentAtomSize: VkDeviceSize,
}

/// [VkPhysicalDeviceMemoryProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMemoryProperties.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties {
  pub memoryTypeCount: uint32_t,
  pub memoryTypes: [VkMemoryType; VK_MAX_MEMORY_TYPES],
  pub memoryHeapCount: uint32_t,
  pub memoryHeaps: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS],
}

/// [VkPhysicalDeviceProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProperties.html)
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct VkPhysicalDeviceProperties {
  pub apiVersion: VulkanVersion,
  pub driverVersion: uint32_t,
  pub vendorID: uint32_t,
  pub deviceID: uint32_t,
  pub deviceType: VkPhysicalDeviceType,
  pub deviceName: [char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
  pub pipelineCacheUUID: [uint8_t; VK_UUID_SIZE],
  pub limits: VkPhysicalDeviceLimits,
  pub sparseProperties: VkPhysicalDeviceSparseProperties,
}
impl Default for VkPhysicalDeviceProperties {
  fn default() -> Self {
    unsafe { zeroed() }
  }
}

/// [VkPhysicalDeviceSparseProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSparseProperties.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkPhysicalDeviceSparseProperties {
  /// Sparse resources support: GPU will access all 2D (single sample) sparse
  /// resources using the standard sparse image block shapes (based on pixel
  /// format)
  pub residencyStandard2DBlockShape: VkBool32,
  /// Sparse resources support: GPU will access all 2D (multisample) sparse
  /// resources using the standard sparse image block shapes (based on pixel
  /// format)
  pub residencyStandard2DMultisampleBlockShape: VkBool32,
  /// Sparse resources support: GPU will access all 3D sparse resources using
  /// the standard sparse image block shapes (based on pixel format)
  pub residencyStandard3DBlockShape: VkBool32,
  /// Sparse resources support: Images with mip level dimensions that are NOT a
  /// multiple of the sparse image block dimensions will be placed in the mip
  /// tail
  pub residencyAlignedMipSize: VkBool32,
  /// Sparse resources support: GPU can consistently access non-resident regions
  /// of a resource, all reads return as if data is 0, writes are discarded
  pub residencyNonResidentStrict: VkBool32,
}

/// [VkQueueFamilyProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueFamilyProperties.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkQueueFamilyProperties {
  /// Queue flags
  /// * **Optional:** true
  pub queueFlags: VkQueueFlags,
  pub queueCount: uint32_t,
  pub timestampValidBits: uint32_t,
  /// Minimum alignment requirement for image transfers
  pub minImageTransferGranularity: VkExtent3D,
}

/// [VkDeviceCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceCreateInfo.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkDeviceCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO`]
  pub sType: VkStructureType,
  ///
  /// * **Optional:** true
  pub pNext: *const c_void,
  ///
  /// * **Optional:** true
  pub flags: VkDeviceCreateFlags,
  pub queueCreateInfoCount: uint32_t,
  ///
  /// * **Len:** queueCreateInfoCount
  pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
  ///
  /// * **Optional:** true
  pub enabledLayerCount: uint32_t,
  /// Ordered list of layer names to be enabled
  /// * **Len:** enabledLayerCount,null-terminated
  pub ppEnabledLayerNames: *const u8,
  ///
  /// * **Optional:** true
  pub enabledExtensionCount: uint32_t,
  ///
  /// * **Len:** enabledExtensionCount,null-terminated
  pub ppEnabledExtensionNames: *const u8,
  ///
  /// * **Optional:** true
  pub pEnabledFeatures: *const VkPhysicalDeviceFeatures,
}
impl Default for VkDeviceCreateInfo {
  fn default() -> Self {
    unsafe { zeroed() }
  }
}

/// [VkDeviceQueueCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceQueueCreateInfo.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkDeviceQueueCreateInfo {
  ///
  /// * Values: [`VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO`]
  pub sType: VkStructureType,
  ///
  /// * Optional: true
  pub pNext: *const void,
  ///
  /// * Optional: true
  pub flags: VkDeviceQueueCreateFlags,
  pub queueFamilyIndex: uint32_t,
  pub queueCount: uint32_t,
  ///
  /// * Len: queueCount
  pub pQueuePriorities: *const float,
}
impl Default for VkDeviceQueueCreateInfo {
  fn default() -> Self {
    unsafe { zeroed() }
  }
}

/// [VkExtensionProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExtensionProperties.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkExtensionProperties {
  /// extension name
  pub extensionName: [char; VK_MAX_EXTENSION_NAME_SIZE],
  /// version of the extension specification implemented
  pub specVersion: uint32_t,
}
impl Default for VkExtensionProperties {
  fn default() -> Self {
    unsafe { zeroed() }
  }
}
impl core::fmt::Debug for VkExtensionProperties {
  #[rustfmt::skip]
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.debug_struct("VkExtensionProperties")
      .field("extensionName", &str_from_null_terminated_byte_slice(&self.extensionName).unwrap_or("<invalid utf8>"))
      .field("specVersion", &self.specVersion)
      .finish()
  }
}

/// [VkLayerProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkLayerProperties.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkLayerProperties {
  /// layer name
  pub layerName: [char; VK_MAX_EXTENSION_NAME_SIZE],
  /// version of the layer specification implemented
  pub specVersion: VulkanVersion,
  /// build or release version of the layer's library
  pub implementationVersion: uint32_t,
  /// Free-form description of the layer
  pub description: [char; VK_MAX_DESCRIPTION_SIZE],
}
impl Default for VkLayerProperties {
  fn default() -> Self {
    unsafe { zeroed() }
  }
}
impl core::fmt::Debug for VkLayerProperties {
  #[rustfmt::skip]
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.debug_struct("VkLayerProperties")
      .field("layerName", &str_from_null_terminated_byte_slice(&self.layerName).unwrap_or("<invalid utf8>"))
      .field("specVersion", &self.specVersion)
      .field("implementationVersion", &self.implementationVersion)
      .field("description", &str_from_null_terminated_byte_slice(&self.description).unwrap_or("<invalid utf8>"))
      .finish()
  }
}
