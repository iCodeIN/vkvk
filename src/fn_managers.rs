#![allow(non_snake_case)]

use super::*;

/// Holds functions from [vkGetInstanceProcAddr][vkGIPA] that **do not** require
/// a [`VkInstance`] to look up.
///
/// These functions are used to create your `VkInstance` in the first place.
///
/// [vkGIPA]: (https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetInstanceProcAddr.html)
#[derive(Clone, Copy)]
pub struct PreInstanceFns {
  vkGetInstanceProcAddr_p: vkGetInstanceProcAddr_t,
  // (1.1) vkEnumerateInstanceVersion
  vkEnumerateInstanceExtensionProperties_p: vkEnumerateInstanceExtensionProperties_t,
  vkEnumerateInstanceLayerProperties_p: vkEnumerateInstanceLayerProperties_t,
  vkCreateInstance_p: vkCreateInstance_t,
}

impl PreInstanceFns {
  pub unsafe fn load_from(vkGetInstanceProcAddr_p: vkGetInstanceProcAddr_t) -> Result<Self, &'static str> {
    use core::mem::transmute as t;
    // non-null-function
    type NNF = unsafe extern "system" fn();
    let vkCreateInstance_p = t::<NNF, _>(vkGetInstanceProcAddr_p(VkInstance::null(), b"vkCreateInstance\0".as_ptr()).ok_or("vkCreateInstance")?);
    let vkEnumerateInstanceExtensionProperties_p = t::<NNF, _>(vkGetInstanceProcAddr_p(VkInstance::null(), b"vkEnumerateInstanceExtensionProperties\0".as_ptr()).ok_or("vkEnumerateInstanceExtensionProperties")?);
    let vkEnumerateInstanceLayerProperties_p = t::<NNF, _>(vkGetInstanceProcAddr_p(VkInstance::null(), b"vkEnumerateInstanceLayerProperties\0".as_ptr()).ok_or("vkEnumerateInstanceLayerProperties")?);
    Ok(Self {
      vkGetInstanceProcAddr_p,
      // (1.1) vkEnumerateInstanceVersion_p
      vkEnumerateInstanceExtensionProperties_p,
      vkEnumerateInstanceLayerProperties_p,
      vkCreateInstance_p,
    })
  }

  /// [vkCreateInstance](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateInstance.html)
  pub unsafe fn CreateInstance(&self, pCreateInfo: &VkInstanceCreateInfo, pAllocator: Option<&VkAllocationCallbacks>, pInstance: &mut VkInstance) -> VkResult {
    (self.vkCreateInstance_p)(pCreateInfo, pAllocator, pInstance)
  }

  /// [vkEnumerateInstanceExtensionProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceExtensionProperties.html)
  pub unsafe fn EnumerateInstanceExtensionProperties(&self, pLayerName: *const char, pPropertyCount: &mut uint32_t, pProperties: *mut VkExtensionProperties) -> VkResult {
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
/// [vkGIPA]: (https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetInstanceProcAddr.html)
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
}

impl core::ops::Deref for InstanceFns {
  type Target = PreInstanceFns;
  fn deref(&self) -> &PreInstanceFns {
    &self.pre_instance_fns
  }
}

impl InstanceFns {
  pub unsafe fn load_from(_pre_instance_fns: PreInstanceFns, _instance: VkInstance) -> Result<Self, &'static str> {
    todo!()
  }

  /// [vkDestroyInstance](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyInstance.html)
  pub unsafe fn DestroyInstance(&self, instance: VkInstance, pAllocator: Option<&VkAllocationCallbacks>) {
    (self.vkDestroyInstance_p)(instance, pAllocator)
  }

  /// [vkEnumeratePhysicalDevices](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDevices.html)
  pub unsafe fn EnumeratePhysicalDevices(&self, instance: VkInstance, pPhysicalDeviceCount: &mut uint32_t, pPhysicalDevices: *mut VkPhysicalDevice) -> VkResult {
    (self.vkEnumeratePhysicalDevices_p)(instance, pPhysicalDeviceCount, pPhysicalDevices)
  }

  /// [vkGetPhysicalDeviceFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFeatures.html)
  pub unsafe fn GetPhysicalDeviceFeatures(&self, physicalDevice: VkPhysicalDevice, pFeatures: &mut VkPhysicalDeviceFeatures) {
    (self.vkGetPhysicalDeviceFeatures_p)(physicalDevice, pFeatures)
  }

  /// [vkGetPhysicalDeviceFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFormatProperties.html)
  pub unsafe fn GetPhysicalDeviceFormatProperties(&self, physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: &mut VkFormatProperties) {
    (self.vkGetPhysicalDeviceFormatProperties_p)(physicalDevice, format, pFormatProperties)
  }

  /// [vkGetPhysicalDeviceImageFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties.html)
  pub unsafe fn GetPhysicalDeviceImageFormatProperties(&self, physicalDevice: VkPhysicalDevice, format: VkFormat, type_: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, pImageFormatProperties: &mut VkImageFormatProperties) -> VkResult {
    (self.vkGetPhysicalDeviceImageFormatProperties_p)(physicalDevice, format, type_, tiling, usage, flags, pImageFormatProperties)
  }

  /// [vkGetPhysicalDeviceProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceProperties.html)
  pub unsafe fn GetPhysicalDeviceProperties(&self, physicalDevice: VkPhysicalDevice, pProperties: &mut VkPhysicalDeviceProperties) {
    (self.vkGetPhysicalDeviceProperties_p)(physicalDevice, pProperties)
  }

  /// [vkGetPhysicalDeviceQueueFamilyProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties.html)
  pub unsafe fn GetPhysicalDeviceQueueFamilyProperties(&self, physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: &mut uint32_t, pQueueFamilyProperties: *mut VkQueueFamilyProperties) {
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
  pub unsafe fn CreateDevice(&self, physicalDevice: VkPhysicalDevice, pCreateInfo: &VkDeviceCreateInfo, pAllocator: Option<&VkAllocationCallbacks>, pDevice: &mut VkDevice) -> VkResult {
    (self.vkCreateDevice_p)(physicalDevice, pCreateInfo, pAllocator, pDevice)
  }

  /// [vkDestroyDevice](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDevice.html)
  pub unsafe fn DestroyDevice(&self, device: VkDevice, pAllocator: Option<&VkAllocationCallbacks>) {
    (self.vkDestroyDevice_p)(device, pAllocator)
  }

  /// [vkEnumerateDeviceExtensionProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateDeviceExtensionProperties.html)
  pub unsafe fn EnumerateDeviceExtensionProperties(&self, physicalDevice: VkPhysicalDevice, pLayerName: *const char, pPropertyCount: *mut uint32_t, pProperties: *mut VkExtensionProperties) -> VkResult {
    (self.vkEnumerateDeviceExtensionProperties_p)(physicalDevice, pLayerName, pPropertyCount, pProperties)
  }

  /// [vkEnumerateDeviceLayerProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateDeviceLayerProperties.html)
  pub unsafe fn EnumerateDeviceLayerProperties(&self, physicalDevice: VkPhysicalDevice, pPropertyCount: *mut uint32_t, pProperties: *mut VkLayerProperties) -> VkResult {
    (self.vkEnumerateDeviceLayerProperties_p)(physicalDevice, pPropertyCount, pProperties)
  }
}

/// Holds functions that come from a [`vkGetDeviceProcAddr_t`] value.
#[derive(Clone, Copy)]
pub struct DeviceFns {
  //
}

impl DeviceFns {
  pub unsafe fn load_from(_instance_fns: InstanceFns, _device: VkDevice) -> Result<Self, &'static str> {
    todo!()
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
