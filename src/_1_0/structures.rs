#![allow(non_snake_case)]

//! Structure types to interface with Vulkan.

// TODO: double check all Default impls

use crate::prelude::*;

use crate::str_from_null_terminated_byte_slice;

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
    Self { sType: VK_STRUCTURE_TYPE_MEMORY_BARRIER, ..unsafe { zeroed() } }
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
  /// Reserved for future use.
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
  pub ppEnabledLayerNames: *const *const u8,
  ///
  /// * **Optional:** true
  pub enabledExtensionCount: uint32_t,
  /// Extension names to be enabled
  /// * **Len:** enabledExtensionCount,null-terminated
  pub ppEnabledExtensionNames: *const *const u8,
}
impl Default for VkInstanceCreateInfo {
  fn default() -> Self {
    Self { sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO, ..unsafe { zeroed() } }
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
impl VkExtensionProperties {
  pub fn extension_name_str(&self) -> &str {
    str_from_null_terminated_byte_slice(&self.extensionName).unwrap_or("<invalid utf8>")
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
impl VkLayerProperties {
  pub fn layer_name_str(&self) -> &str {
    str_from_null_terminated_byte_slice(&self.layerName).unwrap_or("<invalid utf8>")
  }
}

/// [VkSubmitInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubmitInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkSubmitInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_SUBMIT_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
  /// * **Optional:** true
  waitSemaphoreCount: uint32_t,
  ///
  /// * **Len:** waitSemaphoreCount
  pWaitSemaphores: *const VkSemaphore,
  ///
  /// * **Len:** waitSemaphoreCount
  pWaitDstStageMask: *const VkPipelineStageFlags,
  ///
  /// * **Optional:** true
  commandBufferCount: uint32_t,
  ///
  /// * **Len:** commandBufferCount
  pCommandBuffers: *const VkCommandBuffer,
  ///
  /// * **Optional:** true
  signalSemaphoreCount: uint32_t,
  ///
  /// * **Len:** signalSemaphoreCount
  pSignalSemaphores: *const VkSemaphore,
}
impl Default for VkSubmitInfo {
  fn default() -> Self {
    Self { sType: VK_STRUCTURE_TYPE_SUBMIT_INFO, ..unsafe { zeroed() } }
  }
}

/// [VkMappedMemoryRange](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMappedMemoryRange.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkMappedMemoryRange {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  /// Mapped memory object
  memory: VkDeviceMemory,
  /// Offset within the memory object where the range starts
  offset: VkDeviceSize,
  /// Size of the range within the memory object
  size: VkDeviceSize,
}

/// [VkMemoryAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryAllocateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkMemoryAllocateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  /// Size of memory allocation
  allocationSize: VkDeviceSize,
  /// Index of the of the memory type to allocate from
  memoryTypeIndex: uint32_t,
}

/// [VkMemoryRequirements](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryRequirements.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkMemoryRequirements {
  /// Specified in bytes
  size: VkDeviceSize,
  /// Specified in bytes
  alignment: VkDeviceSize,
  /// Bitmask of the allowed memory type indices into memoryTypes[] for this
  /// object
  memoryTypeBits: uint32_t,
}

/// [VkBindSparseInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindSparseInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkBindSparseInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_BIND_SPARSE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
  /// * **Optional:** true
  waitSemaphoreCount: uint32_t,
  ///
  /// * **Len:** waitSemaphoreCount
  pWaitSemaphores: *const VkSemaphore,
  ///
  /// * **Optional:** true
  bufferBindCount: uint32_t,
  ///
  /// * **Len:** bufferBindCount
  pBufferBinds: *const VkSparseBufferMemoryBindInfo,
  ///
  /// * **Optional:** true
  imageOpaqueBindCount: uint32_t,
  ///
  /// * **Len:** imageOpaqueBindCount
  pImageOpaqueBinds: *const VkSparseImageOpaqueMemoryBindInfo,
  ///
  /// * **Optional:** true
  imageBindCount: uint32_t,
  ///
  /// * **Len:** imageBindCount
  pImageBinds: *const VkSparseImageMemoryBindInfo,
  ///
  /// * **Optional:** true
  signalSemaphoreCount: uint32_t,
  ///
  /// * **Len:** signalSemaphoreCount
  pSignalSemaphores: *const VkSemaphore,
}

/// [VkSparseBufferMemoryBindInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseBufferMemoryBindInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkSparseBufferMemoryBindInfo {
  buffer: VkBuffer,
  bindCount: uint32_t,
  ///
  /// * **Len:** bindCount
  pBinds: *const VkSparseMemoryBind,
}

/// [VkSparseImageOpaqueMemoryBindInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageOpaqueMemoryBindInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkSparseImageOpaqueMemoryBindInfo {
  image: VkImage,
  bindCount: uint32_t,
  ///
  /// * **Len:** bindCount
  pBinds: *const VkSparseMemoryBind,
}

/// [VkSparseMemoryBind](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseMemoryBind.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkSparseMemoryBind {
  /// Specified in bytes
  resourceOffset: VkDeviceSize,
  /// Specified in bytes
  size: VkDeviceSize,
  ///
  /// * **Optional:** true
  memory: VkDeviceMemory,
  /// Specified in bytes
  memoryOffset: VkDeviceSize,
  ///
  /// * **Optional:** true
  flags: VkSparseMemoryBindFlags,
}

/// [VkSparseImageMemoryBindInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageMemoryBindInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkSparseImageMemoryBindInfo {
  image: VkImage,
  bindCount: uint32_t,
  ///
  /// * **Len:** bindCount
  pBinds: *const VkSparseImageMemoryBind,
}

/// [VkSparseImageMemoryBind](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageMemoryBind.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkSparseImageMemoryBind {
  subresource: VkImageSubresource,
  offset: VkOffset3D,
  extent: VkExtent3D,
  ///
  /// * **Optional:** true
  memory: VkDeviceMemory,
  /// Specified in bytes
  memoryOffset: VkDeviceSize,
  ///
  /// * **Optional:** true
  flags: VkSparseMemoryBindFlags,
}

/// [VkImageSubresource](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSubresource.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkImageSubresource {
  aspectMask: VkImageAspectFlags,
  mipLevel: uint32_t,
  arrayLayer: uint32_t,
}

/// [VkSparseImageFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageFormatProperties.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkSparseImageFormatProperties {
  ///
  /// * **Optional:** true
  aspectMask: VkImageAspectFlags,
  imageGranularity: VkExtent3D,
  ///
  /// * **Optional:** true
  flags: VkSparseImageFormatFlags,
}

/// [VkSparseImageMemoryRequirements](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageMemoryRequirements.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkSparseImageMemoryRequirements {
  formatProperties: VkSparseImageFormatProperties,
  imageMipTailFirstLod: uint32_t,
  /// Specified in bytes, must be a multiple of sparse block size in bytes /
  /// alignment
  imageMipTailSize: VkDeviceSize,
  /// Specified in bytes, must be a multiple of sparse block size in bytes /
  /// alignment
  imageMipTailOffset: VkDeviceSize,
  /// Specified in bytes, must be a multiple of sparse block size in bytes /
  /// alignment
  imageMipTailStride: VkDeviceSize,
}

/// [VkFenceCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkFenceCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_FENCE_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  /// Fence creation flags
  /// * **Optional:** true
  flags: VkFenceCreateFlags,
}

/// [VkSemaphoreCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkSemaphoreCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  /// Semaphore creation flags
  /// * **Optional:** true
  flags: VkSemaphoreCreateFlags,
}

/// [VkEventCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkEventCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkEventCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_EVENT_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  /// Event creation flags
  /// * **Optional:** true
  flags: VkEventCreateFlags,
}

/// [VkQueryPoolCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkQueryPoolCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
  /// * **Optional:** true
  flags: VkQueryPoolCreateFlags,
  queryType: VkQueryType,
  queryCount: uint32_t,
  /// Optional
  /// * **Optional:** true
  /// * **No Auto-validity:** true
  pipelineStatistics: VkQueryPipelineStatisticFlags,
}

/// [VkBufferCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkBufferCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO`]
  sType: VkStructureType,
  ///
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
  ///
  /// * **Optional:** true
  queueFamilyIndexCount: uint32_t,
  ///
  /// * **No Auto-validity:** true
  /// * **Len:** queueFamilyIndexCount
  pQueueFamilyIndices: *const uint32_t,
}

/// [VkBufferViewCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferViewCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkBufferViewCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
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

/// [VkImageCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkImageCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO`]
  sType: VkStructureType,
  ///
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

/// [VkSubresourceLayout](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubresourceLayout.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkSubresourceLayout {
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

/// [VkComponentMapping](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkComponentMapping.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkComponentMapping {
  r: VkComponentSwizzle,
  g: VkComponentSwizzle,
  b: VkComponentSwizzle,
  a: VkComponentSwizzle,
}

/// [VkImageViewCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkImageViewCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
  /// * **Optional:** true
  flags: VkImageViewCreateFlags,
  image: VkImage,
  viewType: VkImageViewType,
  format: VkFormat,
  components: VkComponentMapping,
  subresourceRange: VkImageSubresourceRange,
}

/// [VkShaderModuleCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderModuleCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkShaderModuleCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
  /// * **Optional:** true
  flags: VkShaderModuleCreateFlags,
  /// Specified in bytes
  codeSize: size_t,
  /// Binary code of size `codeSize`
  /// * **Len:** `codeSize / 4`
  pCode: *const uint32_t,
}

/// [VkPipelineCacheCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCacheCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkPipelineCacheCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
  /// * **Optional:** true
  flags: VkPipelineCacheCreateFlags,
  /// Size of initial data to populate cache, in bytes
  /// * **Optional:** true
  initialDataSize: size_t,
  /// Initial data to populate cache
  /// * **Len:** initialDataSize
  pInitialData: *const c_void,
}

/// [VkComputePipelineCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkComputePipelineCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkComputePipelineCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  /// Pipeline creation flags
  /// * **Optional:** true
  flags: VkPipelineCreateFlags,
  stage: VkPipelineShaderStageCreateInfo,
  /// Interface layout of the pipeline
  layout: VkPipelineLayout,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is nonzero, it
  /// specifies the handle of the base pipeline this is a derivative of
  /// * **Optional:** true
  /// * **No Auto-validity:** true
  basePipelineHandle: VkPipeline,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is not -1, it
  /// specifies an index into pCreateInfos of the base pipeline this is a
  /// derivative of
  basePipelineIndex: int32_t,
}

/// [VkGraphicsPipelineCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGraphicsPipelineCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkGraphicsPipelineCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  /// Pipeline creation flags
  /// * **Optional:** true
  flags: VkPipelineCreateFlags,
  stageCount: uint32_t,
  /// One entry for each active shader stage
  /// * **Len:** stageCount
  pStages: *const VkPipelineShaderStageCreateInfo,
  ///
  /// * **Optional:** true
  /// * **No Auto-validity:** true
  pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
  ///
  /// * **Optional:** true
  /// * **No Auto-validity:** true
  pInputAssemblyState: *const VkPipelineInputAssemblyStateCreateInfo,
  ///
  /// * **Optional:** true
  /// * **No Auto-validity:** true
  pTessellationState: *const VkPipelineTessellationStateCreateInfo,
  ///
  /// * **Optional:** true
  /// * **No Auto-validity:** true
  pViewportState: *const VkPipelineViewportStateCreateInfo,
  pRasterizationState: *const VkPipelineRasterizationStateCreateInfo,
  ///
  /// * **Optional:** true
  /// * **No Auto-validity:** true
  pMultisampleState: *const VkPipelineMultisampleStateCreateInfo,
  ///
  /// * **Optional:** true
  /// * **No Auto-validity:** true
  pDepthStencilState: *const VkPipelineDepthStencilStateCreateInfo,
  ///
  /// * **Optional:** true
  /// * **No Auto-validity:** true
  pColorBlendState: *const VkPipelineColorBlendStateCreateInfo,
  ///
  /// * **Optional:** true
  pDynamicState: *const VkPipelineDynamicStateCreateInfo,
  /// Interface layout of the pipeline
  layout: VkPipelineLayout,
  renderPass: VkRenderPass,
  subpass: uint32_t,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is nonzero, it
  /// specifies the handle of the base pipeline this is a derivative of
  /// * **Optional:** true
  /// * **No Auto-validity:** true
  basePipelineHandle: VkPipeline,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is not -1, it
  /// specifies an index into pCreateInfos of the base pipeline this is a
  /// derivative of
  basePipelineIndex: int32_t,
}

/// [VkPipelineColorBlendAttachmentState](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineColorBlendAttachmentState.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkPipelineColorBlendAttachmentState {
  blendEnable: VkBool32,
  srcColorBlendFactor: VkBlendFactor,
  dstColorBlendFactor: VkBlendFactor,
  colorBlendOp: VkBlendOp,
  srcAlphaBlendFactor: VkBlendFactor,
  dstAlphaBlendFactor: VkBlendFactor,
  alphaBlendOp: VkBlendOp,
  ///
  /// * **Optional:** true
  colorWriteMask: VkColorComponentFlags,
}

/// [VkPipelineColorBlendStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineColorBlendStateCreateInfo.html)
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct VkPipelineColorBlendStateCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
  /// * **Optional:** true
  flags: VkPipelineColorBlendStateCreateFlags,
  logicOpEnable: VkBool32,
  ///
  /// * **No Auto-validity:** true
  logicOp: VkLogicOp,
  /// # of pAttachments
  /// * **Optional:** true
  attachmentCount: uint32_t,
  ///
  /// * **Len:** attachmentCount
  pAttachments: *const VkPipelineColorBlendAttachmentState,
  blendConstants: [float; 4],
}

/// [VkPipelineDepthStencilStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDepthStencilStateCreateInfo.html)
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct VkPipelineDepthStencilStateCreateInfo {
  ///
  /// * **Values:**
  ///   [`VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
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

/// [VkPipelineDynamicStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDynamicStateCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkPipelineDynamicStateCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
  /// * **Optional:** true
  flags: VkPipelineDynamicStateCreateFlags,
  ///
  /// * **Optional:** true
  dynamicStateCount: uint32_t,
  ///
  /// * **Len:** dynamicStateCount
  pDynamicStates: *const VkDynamicState,
}

/// [VkPipelineInputAssemblyStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineInputAssemblyStateCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
  ///
  /// * **Values:**
  ///   [`VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
  /// * **Optional:** true
  flags: VkPipelineInputAssemblyStateCreateFlags,
  topology: VkPrimitiveTopology,
  primitiveRestartEnable: VkBool32,
}

/// [VkPipelineMultisampleStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineMultisampleStateCreateInfo.html)
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct VkPipelineMultisampleStateCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
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
  /// * **Len:** latexmath:[\lceil{\mathit{rasterizationSamples} \over
  ///   32}\rceil]
  /// * **Alt Len:** (rasterizationSamples + 31) / 32
  pSampleMask: *const VkSampleMask,
  alphaToCoverageEnable: VkBool32,
  alphaToOneEnable: VkBool32,
}

/// [VkPipelineRasterizationStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationStateCreateInfo.html)
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct VkPipelineRasterizationStateCreateInfo {
  ///
  /// * **Values:**
  ///   [`VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
  /// * **Optional:** true
  flags: VkPipelineRasterizationStateCreateFlags,
  depthClampEnable: VkBool32,
  rasterizerDiscardEnable: VkBool32,
  /// optional (GL45)
  polygonMode: VkPolygonMode,
  ///
  /// * **Optional:** true
  cullMode: VkCullModeFlags,
  frontFace: VkFrontFace,
  depthBiasEnable: VkBool32,
  depthBiasConstantFactor: float,
  depthBiasClamp: float,
  depthBiasSlopeFactor: float,
  lineWidth: float,
}

/// [VkPipelineShaderStageCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineShaderStageCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkPipelineShaderStageCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
  /// * **Optional:** true
  flags: VkPipelineShaderStageCreateFlags,
  /// Shader stage
  stage: VkShaderStageFlagBits,
  /// Module containing entry point
  module: VkShaderModule,
  /// Null-terminated entry point name
  /// * **Len:** null-terminated
  pName: *const u8,
  ///
  /// * **Optional:** true
  pSpecializationInfo: *const VkSpecializationInfo,
}

/// [VkPipelineTessellationStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineTessellationStateCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkPipelineTessellationStateCreateInfo {
  ///
  /// * **Values:**
  ///   [`VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
  /// * **Optional:** true
  flags: VkPipelineTessellationStateCreateFlags,
  patchControlPoints: uint32_t,
}

/// [VkPipelineVertexInputStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineVertexInputStateCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkPipelineVertexInputStateCreateInfo {
  ///
  /// * **Values:**
  ///   [`VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
  /// * **Optional:** true
  flags: VkPipelineVertexInputStateCreateFlags,
  /// number of bindings
  /// * **Optional:** true
  vertexBindingDescriptionCount: uint32_t,
  ///
  /// * **Len:** vertexBindingDescriptionCount
  pVertexBindingDescriptions: *const VkVertexInputBindingDescription,
  /// number of attributes
  /// * **Optional:** true
  vertexAttributeDescriptionCount: uint32_t,
  ///
  /// * **Len:** vertexAttributeDescriptionCount
  pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription,
}

/// [VkPipelineViewportStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportStateCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkPipelineViewportStateCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
  /// * **Optional:** true
  flags: VkPipelineViewportStateCreateFlags,
  ///
  /// * **Optional:** true
  viewportCount: uint32_t,
  ///
  /// * **Optional:** true
  /// * **No Auto-validity:** true
  /// * **Len:** viewportCount
  pViewports: *const VkViewport,
  ///
  /// * **Optional:** true
  scissorCount: uint32_t,
  ///
  /// * **Optional:** true
  /// * **No Auto-validity:** true
  /// * **Len:** scissorCount
  pScissors: *const VkRect2D,
}

/// [VkSpecializationInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSpecializationInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkSpecializationInfo {
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

/// [VkSpecializationMapEntry](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSpecializationMapEntry.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkSpecializationMapEntry {
  /// The SpecConstant ID specified in the BIL
  constantID: uint32_t,
  /// Offset of the value in the data block
  offset: uint32_t,
  /// Size in bytes of the SpecConstant
  /// * **No Auto-validity:** true
  size: size_t,
}

/// [VkStencilOpState](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStencilOpState.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkStencilOpState {
  failOp: VkStencilOp,
  passOp: VkStencilOp,
  depthFailOp: VkStencilOp,
  compareOp: VkCompareOp,
  compareMask: uint32_t,
  writeMask: uint32_t,
  reference: uint32_t,
}

/// [VkVertexInputAttributeDescription](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputAttributeDescription.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkVertexInputAttributeDescription {
  /// location of the shader vertex attrib
  location: uint32_t,
  /// Vertex buffer binding id
  binding: uint32_t,
  /// format of source data
  format: VkFormat,
  /// Offset of first element in bytes from base of vertex
  offset: uint32_t,
}

/// [VkVertexInputBindingDescription](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputBindingDescription.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkVertexInputBindingDescription {
  /// Vertex buffer binding id
  binding: uint32_t,
  /// Distance between vertices in bytes (0 = no advancement)
  stride: uint32_t,
  /// The rate at which the vertex data is consumed
  inputRate: VkVertexInputRate,
}

/// [VkViewport](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViewport.html)
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct VkViewport {
  x: float,
  y: float,
  width: float,
  height: float,
  minDepth: float,
  maxDepth: float,
}

/// [VkPipelineLayoutCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineLayoutCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkPipelineLayoutCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
  /// * **Optional:** true
  flags: VkPipelineLayoutCreateFlags,
  /// Number of descriptor sets interfaced by the pipeline
  /// * **Optional:** true
  setLayoutCount: uint32_t,
  /// Array of setCount number of descriptor set layout objects defining the
  /// layout of the
  /// * **Len:** setLayoutCount
  pSetLayouts: *const VkDescriptorSetLayout,
  /// Number of push-constant ranges used by the pipeline
  /// * **Optional:** true
  pushConstantRangeCount: uint32_t,
  /// Array of pushConstantRangeCount number of ranges used by various shader
  /// stages
  /// * **Len:** pushConstantRangeCount
  pPushConstantRanges: *const VkPushConstantRange,
}

/// [VkPushConstantRange](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPushConstantRange.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkPushConstantRange {
  /// Which stages use the range
  stageFlags: VkShaderStageFlags,
  /// Start of the range, in bytes
  offset: uint32_t,
  /// Size of the range, in bytes
  size: uint32_t,
}

/// [VkSamplerCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerCreateInfo.html)
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct VkSamplerCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
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
  ///
  /// * **No Auto-validity:** true
  compareOp: VkCompareOp,
  minLod: float,
  maxLod: float,
  ///
  /// * **No Auto-validity:** true
  borderColor: VkBorderColor,
  unnormalizedCoordinates: VkBool32,
}

/// [VkCopyDescriptorSet](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyDescriptorSet.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkCopyDescriptorSet {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET`]
  sType: VkStructureType,
  ///
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
  /// Number of descriptors to write (determines the size of the array pointed
  /// by pDescriptors)
  descriptorCount: uint32_t,
}

/// [VkDescriptorBufferInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorBufferInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkDescriptorBufferInfo {
  /// Buffer used for this descriptor slot.
  /// * **Optional:** true
  buffer: VkBuffer,
  /// Base offset from buffer start in bytes to update in the descriptor set.
  offset: VkDeviceSize,
  /// Size in bytes of the buffer resource for this descriptor update.
  range: VkDeviceSize,
}

/// [VkDescriptorImageInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorImageInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkDescriptorImageInfo {
  /// Sampler to write to the descriptor in case it is a SAMPLER or
  /// COMBINED_IMAGE_SAMPLER descriptor. Ignored otherwise.
  /// * **No Auto-validity:** true
  sampler: VkSampler,
  /// Image view to write to the descriptor in case it is a SAMPLED_IMAGE,
  /// STORAGE_IMAGE, COMBINED_IMAGE_SAMPLER, or INPUT_ATTACHMENT descriptor.
  /// Ignored otherwise.
  /// * **No Auto-validity:** true
  imageView: VkImageView,
  /// Layout the image is expected to be in when accessed using this descriptor
  /// (only used if imageView is not VK_NULL_HANDLE).
  /// * **No Auto-validity:** true
  imageLayout: VkImageLayout,
}

/// [VkDescriptorPoolCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorPoolCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkDescriptorPoolCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
  /// * **Optional:** true
  flags: VkDescriptorPoolCreateFlags,
  maxSets: uint32_t,
  poolSizeCount: uint32_t,
  ///
  /// * **Len:** poolSizeCount
  pPoolSizes: *const VkDescriptorPoolSize,
}

/// [VkDescriptorPoolSize](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorPoolSize.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkDescriptorPoolSize {
  type_: VkDescriptorType,
  descriptorCount: uint32_t,
}

/// [VkDescriptorSetAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetAllocateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkDescriptorSetAllocateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  descriptorPool: VkDescriptorPool,
  descriptorSetCount: uint32_t,
  ///
  /// * **Len:** descriptorSetCount
  pSetLayouts: *const VkDescriptorSetLayout,
}

/// [VkDescriptorSetLayoutBinding](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutBinding.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkDescriptorSetLayoutBinding {
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
  /// Immutable samplers (used if descriptor type is SAMPLER or
  /// COMBINED_IMAGE_SAMPLER, is either NULL or contains count number of
  /// elements)
  /// * **Optional:** true
  /// * **No Auto-validity:** true
  /// * **Len:** descriptorCount
  pImmutableSamplers: *const VkSampler,
}

/// [VkWriteDescriptorSet](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWriteDescriptorSet.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkWriteDescriptorSet {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  /// Destination descriptor set
  /// * **No Auto-validity:** true
  dstSet: VkDescriptorSet,
  /// Binding within the destination descriptor set to write
  dstBinding: uint32_t,
  /// Array element within the destination binding to write
  dstArrayElement: uint32_t,
  /// Number of descriptors to write (determines the size of the array pointed
  /// by pDescriptors)
  descriptorCount: uint32_t,
  /// Descriptor type to write (determines which members of the array pointed by
  /// pDescriptors are going to be used)
  descriptorType: VkDescriptorType,
  /// Sampler, image view, and layout for SAMPLER, COMBINED_IMAGE_SAMPLER,
  /// {SAMPLED,STORAGE}_IMAGE, and INPUT_ATTACHMENT descriptor types.
  /// * **No Auto-validity:** true
  /// * **Len:** descriptorCount
  pImageInfo: *const VkDescriptorImageInfo,
  /// Raw buffer, size, and offset for {UNIFORM,STORAGE}_BUFFER{_DYNAMIC}
  /// descriptor types.
  /// * **No Auto-validity:** true
  /// * **Len:** descriptorCount
  pBufferInfo: *const VkDescriptorBufferInfo,
  /// Buffer view to write to the descriptor for {UNIFORM,STORAGE}_TEXEL_BUFFER
  /// descriptor types.
  /// * **No Auto-validity:** true
  /// * **Len:** descriptorCount
  pTexelBufferView: *const VkBufferView,
}

/// [VkDescriptorSetLayoutCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkDescriptorSetLayoutCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
  /// * **Optional:** true
  flags: VkDescriptorSetLayoutCreateFlags,
  /// Number of bindings in the descriptor set layout
  /// * **Optional:** true
  bindingCount: uint32_t,
  /// Array of descriptor set layout bindings
  /// * **Len:** bindingCount
  pBindings: *const VkDescriptorSetLayoutBinding,
}

/// [VkAttachmentDescription](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentDescription.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkAttachmentDescription {
  ///
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

/// [VkAttachmentReference](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentReference.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkAttachmentReference {
  attachment: uint32_t,
  layout: VkImageLayout,
}

/// [VkFramebufferCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebufferCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkFramebufferCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
  /// * **Optional:** true
  flags: VkFramebufferCreateFlags,
  renderPass: VkRenderPass,
  ///
  /// * **Optional:** true
  attachmentCount: uint32_t,
  ///
  /// * **No Auto-validity:** true
  /// * **Len:** attachmentCount
  pAttachments: *const VkImageView,
  width: uint32_t,
  height: uint32_t,
  layers: uint32_t,
}

/// [VkRenderPassCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkRenderPassCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  ///
  /// * **Optional:** true
  flags: VkRenderPassCreateFlags,
  ///
  /// * **Optional:** true
  attachmentCount: uint32_t,
  ///
  /// * **Len:** attachmentCount
  pAttachments: *const VkAttachmentDescription,
  subpassCount: uint32_t,
  ///
  /// * **Len:** subpassCount
  pSubpasses: *const VkSubpassDescription,
  ///
  /// * **Optional:** true
  dependencyCount: uint32_t,
  ///
  /// * **Len:** dependencyCount
  pDependencies: *const VkSubpassDependency,
}

/// [VkSubpassDependency](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDependency.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkSubpassDependency {
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
  ///
  /// * **Optional:** true
  dependencyFlags: VkDependencyFlags,
}

/// [VkSubpassDescription](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDescription.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkSubpassDescription {
  ///
  /// * **Optional:** true
  flags: VkSubpassDescriptionFlags,
  /// Must be VK_PIPELINE_BIND_POINT_GRAPHICS for now
  pipelineBindPoint: VkPipelineBindPoint,
  ///
  /// * **Optional:** true
  inputAttachmentCount: uint32_t,
  ///
  /// * **Len:** inputAttachmentCount
  pInputAttachments: *const VkAttachmentReference,
  ///
  /// * **Optional:** true
  colorAttachmentCount: uint32_t,
  ///
  /// * **Len:** colorAttachmentCount
  pColorAttachments: *const VkAttachmentReference,
  ///
  /// * **Optional:** true
  /// * **Len:** colorAttachmentCount
  pResolveAttachments: *const VkAttachmentReference,
  ///
  /// * **Optional:** true
  pDepthStencilAttachment: *const VkAttachmentReference,
  ///
  /// * **Optional:** true
  preserveAttachmentCount: uint32_t,
  ///
  /// * **Len:** preserveAttachmentCount
  pPreserveAttachments: *const uint32_t,
}

/// [VkCommandPoolCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandPoolCreateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkCommandPoolCreateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  /// Command pool creation flags
  /// * **Optional:** true
  flags: VkCommandPoolCreateFlags,
  queueFamilyIndex: uint32_t,
}

/// [VkCommandBufferAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferAllocateInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkCommandBufferAllocateInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  commandPool: VkCommandPool,
  level: VkCommandBufferLevel,
  commandBufferCount: uint32_t,
}

/// [VkCommandBufferBeginInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferBeginInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkCommandBufferBeginInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO`]
  sType: VkStructureType,
  ///
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

/// [VkCommandBufferInheritanceInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferInheritanceInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkCommandBufferInheritanceInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO`]
  sType: VkStructureType,
  ///
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
  /// Whether this secondary command buffer may be executed during an occlusion
  /// query
  occlusionQueryEnable: VkBool32,
  /// Query flags used by this secondary command buffer, if executed during an
  /// occlusion query
  /// * **Optional:** true
  /// * **No Auto-validity:** true
  queryFlags: VkQueryControlFlags,
  /// Pipeline statistics that may be counted for this secondary command buffer
  /// * **Optional:** true
  /// * **No Auto-validity:** true
  pipelineStatistics: VkQueryPipelineStatisticFlags,
}

/// [VkBufferCopy](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCopy.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkBufferCopy {
  /// Specified in bytes
  srcOffset: VkDeviceSize,
  /// Specified in bytes
  dstOffset: VkDeviceSize,
  /// Specified in bytes
  /// * **No Auto-validity:** true
  size: VkDeviceSize,
}

/// [VkBufferImageCopy](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferImageCopy.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkBufferImageCopy {
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

/// [VkClearAttachment](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearAttachment.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClearAttachment {
  aspectMask: VkImageAspectFlags,
  colorAttachment: uint32_t,
  clearValue: VkClearValue,
}

/// [VkClearDepthStencilValue](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearDepthStencilValue.html)
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct VkClearDepthStencilValue {
  depth: float,
  stencil: uint32_t,
}

/// [VkClearRect](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearRect.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkClearRect {
  rect: VkRect2D,
  baseArrayLayer: uint32_t,
  layerCount: uint32_t,
}

/// [VkImageBlit](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageBlit.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkImageBlit {
  srcSubresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  srcOffsets: [VkOffset3D; 2],
  dstSubresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  dstOffsets: [VkOffset3D; 2],
}

/// [VkImageCopy](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageCopy.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkImageCopy {
  srcSubresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  srcOffset: VkOffset3D,
  dstSubresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  dstOffset: VkOffset3D,
  /// Specified in pixels for both compressed and uncompressed images
  extent: VkExtent3D,
}

/// [VkImageResolve](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageResolve.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkImageResolve {
  srcSubresource: VkImageSubresourceLayers,
  srcOffset: VkOffset3D,
  dstSubresource: VkImageSubresourceLayers,
  dstOffset: VkOffset3D,
  extent: VkExtent3D,
}

/// [VkImageSubresourceLayers](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSubresourceLayers.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkImageSubresourceLayers {
  aspectMask: VkImageAspectFlags,
  mipLevel: uint32_t,
  baseArrayLayer: uint32_t,
  layerCount: uint32_t,
}

/// [VkRenderPassBeginInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassBeginInfo.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VkRenderPassBeginInfo {
  ///
  /// * **Values:** [`VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO`]
  sType: VkStructureType,
  ///
  /// * **Optional:** true
  pNext: *const c_void,
  renderPass: VkRenderPass,
  framebuffer: VkFramebuffer,
  renderArea: VkRect2D,
  ///
  /// * **Optional:** true
  clearValueCount: uint32_t,
  ///
  /// * **Len:** clearValueCount
  pClearValues: *const VkClearValue,
}
