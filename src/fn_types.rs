#![allow(dead_code)]

use super::*;

pub type PFN_vkAllocationFunction = Option<vkAllocationFunction_t>;

pub type PFN_vkFreeFunction = Option<vkFreeFunction_t>;

pub type PFN_vkInternalAllocationNotification = Option<vkInternalAllocationNotification_t>;

pub type PFN_vkInternalFreeNotification = Option<vkInternalFreeNotification_t>;

pub type PFN_vkReallocationFunction = Option<vkReallocationFunction_t>;

pub type PFN_vkVoidFunction = Option<vkVoidFunction_t>;

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

pub(crate) type vkGetInstanceProcAddr_t = unsafe extern "system" fn(instance: VkInstance, pName: *const char) -> PFN_vkVoidFunction;

pub(crate) type vkGetDeviceProcAddr_t = unsafe extern "system" fn(device: VkDevice, pName: *const char) -> PFN_vkVoidFunction;
