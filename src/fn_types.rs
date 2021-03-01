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

pub(crate) type vkAllocationFunction_t = unsafe extern "system" fn(pUserData: *mut void, size: size_t, alignment: size_t, allocationScope: VkSystemAllocationScope) -> *mut void;

pub(crate) type vkFreeFunction_t = unsafe extern "system" fn(pUserData: *mut void, pMemory: *mut void);

pub(crate) type vkInternalAllocationNotification_t = unsafe extern "system" fn(pUserData: *mut void, size: size_t, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);

pub(crate) type vkInternalFreeNotification_t = unsafe extern "system" fn(pUserData: *mut void, size: size_t, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);

pub(crate) type vkReallocationFunction_t = unsafe extern "system" fn(pUserData: *mut void, pOriginal: *mut void, size: size_t, alignment: size_t, allocationScope: VkSystemAllocationScope) -> *mut void;

pub(crate) type vkVoidFunction_t = unsafe extern "system" fn();

pub(crate) type vkCreateInstance_t = unsafe extern "system" fn(pCreateInfo: &VkInstanceCreateInfo, pAllocator: Option<&VkAllocationCallbacks>, pInstance: &mut VkInstance) -> VkResult;

pub(crate) type vkDestroyInstance_t = unsafe extern "system" fn(instance: VkInstance, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkEnumeratePhysicalDevices_t = unsafe extern "system" fn(instance: VkInstance, pPhysicalDeviceCount: &mut uint32_t, pPhysicalDevices: *mut VkPhysicalDevice) -> VkResult;

pub(crate) type vkGetPhysicalDeviceFeatures_t = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pFeatures: &mut VkPhysicalDeviceFeatures);

pub(crate) type vkGetPhysicalDeviceFormatProperties_t = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: &mut VkFormatProperties);

pub(crate) type vkGetPhysicalDeviceImageFormatProperties_t = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, type_: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, pImageFormatProperties: &mut VkImageFormatProperties) -> VkResult;

pub(crate) type vkGetPhysicalDeviceProperties_t = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pProperties: &mut VkPhysicalDeviceProperties);

pub(crate) type vkGetPhysicalDeviceQueueFamilyProperties_t = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: &mut uint32_t, pQueueFamilyProperties: *mut VkQueueFamilyProperties);

pub(crate) type vkGetPhysicalDeviceMemoryProperties_t = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pMemoryProperties: &mut VkPhysicalDeviceMemoryProperties);

pub(crate) type vkCreateDevice_t = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pCreateInfo: &VkDeviceCreateInfo, pAllocator: Option<&VkAllocationCallbacks>, pDevice: &mut VkDevice) -> VkResult;

pub(crate) type vkDestroyDevice_t = unsafe extern "system" fn(device: VkDevice, pAllocator: Option<&VkAllocationCallbacks>);

pub(crate) type vkEnumerateInstanceExtensionProperties_t = unsafe extern "system" fn(pLayerName: *const char, pPropertyCount: &mut uint32_t, pProperties: *mut VkExtensionProperties) -> VkResult;

pub(crate) type vkEnumerateDeviceExtensionProperties_t = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pLayerName: *const char, pPropertyCount: *mut uint32_t, pProperties: *mut VkExtensionProperties) -> VkResult;

pub(crate) type vkEnumerateInstanceLayerProperties_t = unsafe extern "system" fn(pPropertyCount: &mut uint32_t, pProperties: *mut VkLayerProperties) -> VkResult;

pub(crate) type vkEnumerateDeviceLayerProperties_t = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut uint32_t, pProperties: *mut VkLayerProperties) -> VkResult;