use super::*;

macro_rules! fn_ptr {
  (
    $pfn:ident;
    $fn_t:ident;
    $the_type:ty;
  ) => {
    pub type $fn_t = $the_type;
    pub type $pfn = Option<$fn_t>;
  };
}

fn_ptr!(
  PFN_vkInternalAllocationNotification;
  vkInternalAllocationNotification_t;
  unsafe extern "system" fn(pUserData: *mut void, size: size_t, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);
);

fn_ptr!(
  PFN_vkInternalFreeNotification;
  vkInternalFreeNotification_t;
  unsafe extern "system" fn(pUserData: *mut void, size: size_t, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);
);

fn_ptr!(
  PFN_vkReallocationFunction;
  vkReallocationFunction_t;
  unsafe extern "system" fn(pUserData: *mut void, pOriginal: *mut void, size: size_t, alignment: size_t, allocationScope: VkSystemAllocationScope) -> *mut void;
);

fn_ptr!(
  PFN_vkAllocationFunction;
  vkAllocationFunction_t;
  unsafe extern "system" fn(pUserData: *mut void, size: size_t, alignment: size_t, allocationScope: VkSystemAllocationScope) -> *mut void;
);

fn_ptr!(
  PFN_vkFreeFunction;
  vkFreeFunction_t;
  unsafe extern "system" fn(pUserData: *mut void, pMemory: *mut void);
);

fn_ptr!(
  PFN_vkVoidFunction;
  vkVoidFunction_t;
  unsafe extern "system" fn();
);

fn_ptr!(
  PFN_vkDebugReportCallbackEXT;
  vkDebugReportCallbackEXT_t;
  unsafe extern "system" fn(flags: VkDebugReportFlagsEXT, objectType: VkDebugReportObjectTypeEXT, object: uint64_t, location: size_t, messageCode: int32_t, pLayerPrefix: *const  char, pMessage: *const  char, pUserData: *mut void) -> VkBool32;
);

fn_ptr!(
  PFN_vkDebugUtilsMessengerCallbackEXT;
  vkDebugUtilsMessengerCallbackEXT_t;
  unsafe extern "system" fn(messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT, messageTypes: VkDebugUtilsMessageTypeFlagsEXT, pCallbackData: *const  VkDebugUtilsMessengerCallbackDataEXT, pUserData: *mut void) -> VkBool32;
);

fn_ptr!(
  PFN_vkDeviceMemoryReportCallbackEXT;
  vkDeviceMemoryReportCallbackEXT_t;
  unsafe extern "system" fn(pCallbackData: *const VkDeviceMemoryReportCallbackDataEXT, pUserData: *mut void);
);
