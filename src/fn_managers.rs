#![allow(non_snake_case)]

use super::*;

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
