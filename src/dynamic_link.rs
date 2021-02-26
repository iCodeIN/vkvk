//! The module with dynamic link declarations. Demo usage only!

use super::*;

#[link(name = "vulkan-1")]
extern "system" {
  /// [vkCreateInstance](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateInstance.html)
  pub fn vkCreateInstance(pCreateInfo: &VkInstanceCreateInfo, pAllocator: Option<&VkAllocationCallbacks>, pInstance: &mut VkInstance) -> VkResult;

  /// [vkDestroyInstance](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyInstance.html)
  pub fn vkDestroyInstance(instance: VkInstance, pAllocator: Option<&VkAllocationCallbacks>);

  /// [vkEnumeratePhysicalDevices](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDevices.html)
  pub fn vkEnumeratePhysicalDevices(instance: VkInstance, pPhysicalDeviceCount: &mut uint32_t, pPhysicalDevices: *mut VkPhysicalDevice) -> VkResult;

  /// [vkGetPhysicalDeviceFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFeatures.html)
  pub fn vkGetPhysicalDeviceFeatures(physicalDevice: VkPhysicalDevice, pFeatures: &mut VkPhysicalDeviceFeatures);

  /// [vkGetPhysicalDeviceFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFormatProperties.html)
  pub fn vkGetPhysicalDeviceFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: &mut VkFormatProperties);

  /// [vkGetPhysicalDeviceImageFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties.html)
  pub fn vkGetPhysicalDeviceImageFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, type_: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, pImageFormatProperties: &mut VkImageFormatProperties) -> VkResult;

  /// [vkGetPhysicalDeviceProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceProperties.html)
  pub fn vkGetPhysicalDeviceProperties(physicalDevice: VkPhysicalDevice, pProperties: &mut VkPhysicalDeviceProperties);

  /// [vkGetPhysicalDeviceQueueFamilyProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties.html)
  pub fn vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: &mut uint32_t, pQueueFamilyProperties: *mut VkQueueFamilyProperties);

  /// [vkGetPhysicalDeviceMemoryProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMemoryProperties.html)
  pub fn vkGetPhysicalDeviceMemoryProperties(physicalDevice: VkPhysicalDevice, pMemoryProperties: &mut VkPhysicalDeviceMemoryProperties);

  /// [vkGetInstanceProcAddr](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetInstanceProcAddr.html)
  pub fn vkGetInstanceProcAddr(instance: VkInstance, pName: *const char) -> PFN_vkVoidFunction;

  /// [vkGetDeviceProcAddr](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceProcAddr.html)
  pub fn vkGetDeviceProcAddr(device: VkDevice, pName: *const char) -> PFN_vkVoidFunction;
}
