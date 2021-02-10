use super::*;

use core::ptr::{null, null_mut};

#[repr(C)]
pub struct PreInstanceFns {
  vkGetInstanceProcAddr_p: vkGetInstanceProcAddr_t,
  vkEnumerateInstanceVersion_p: Option<vkEnumerateInstanceVersion_t>,
  vkEnumerateInstanceExtensionProperties_p: vkEnumerateInstanceExtensionProperties_t,
  vkEnumerateInstanceLayerProperties_p: vkEnumerateInstanceLayerProperties_t,
  vkCreateInstance_p: vkCreateInstance_t,
}

impl PreInstanceFns {
  /// Gets the pre-instance functions out of the `vkGetInstanceProcAddr`
  /// function given.
  ///
  /// See [vkGetInstanceProcAddr](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetInstanceProcAddr.html)
  ///
  /// ## Failure
  /// * If any function *other than* `vkEnumerateInstanceVersion` fail to load,
  ///   this will fail. The `Err` will be the name of the first required
  ///   function that failed to load.
  pub unsafe fn new(vkGetInstanceProcAddr_p: vkGetInstanceProcAddr_t) -> Result<Self, &'static str> {
    use core::mem::transmute as t;
    let vkEnumerateInstanceVersion_p = t(vkGetInstanceProcAddr_p(VkInstance::null(), b"vkEnumerateInstanceVersion\0".as_ptr()));
    let vkEnumerateInstanceExtensionProperties_p = t(vkGetInstanceProcAddr_p(VkInstance::null(), b"vkEnumerateInstanceExtensionProperties\0".as_ptr()).ok_or("vkEnumerateInstanceExtensionProperties")?);
    let vkEnumerateInstanceLayerProperties_p = t(vkGetInstanceProcAddr_p(VkInstance::null(), b"vkEnumerateInstanceLayerProperties\0".as_ptr()).ok_or("vkEnumerateInstanceLayerProperties")?);
    let vkCreateInstance_p = t(vkGetInstanceProcAddr_p(VkInstance::null(), b"vkCreateInstance\0".as_ptr()).ok_or("vkCreateInstance")?);
    Ok(Self { vkGetInstanceProcAddr_p, vkEnumerateInstanceVersion_p, vkEnumerateInstanceExtensionProperties_p, vkEnumerateInstanceLayerProperties_p, vkCreateInstance_p })
  }

  /// Checks the maximum Instance version you'll be able to create.
  ///
  /// See [vkEnumerateInstanceVersion](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceVersion.html)
  pub fn enumerate_instance_version(&self) -> Result<VulkanVersion, VkResult> {
    match self.vkEnumerateInstanceVersion_p {
      None => Ok(VulkanVersion::_1_0),
      Some(vkEIV) => {
        let mut version = VulkanVersion::default();
        let vk_result = vkEIV(&mut version);
        if vk_result == VK_SUCCESS {
          Ok(version)
        } else {
          Err(vk_result)
        }
      }
    }
  }

  /// Gets the available layers.
  ///
  /// See [vkEnumerateInstanceLayerProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceLayerProperties.html)
  pub fn enumerate_instance_layer_properties(&self) -> Result<Vec<VkLayerProperties>, VkResult> {
    let mut layer_count = 0;
    let get_count_result = unsafe { (self.vkEnumerateInstanceLayerProperties_p)(&mut layer_count, null_mut()) };
    if get_count_result != VK_SUCCESS {
      return Err(get_count_result);
    }
    let mut v = Vec::with_capacity(layer_count as usize);
    let get_properties_result = unsafe { (self.vkEnumerateInstanceLayerProperties_p)(&mut layer_count, v.as_mut_ptr()) };
    if get_properties_result != VK_SUCCESS {
      Err(get_properties_result)
    } else {
      unsafe { v.set_len(layer_count as usize) };
      Ok(v)
    }
  }

  /// Gets the available extensions for a given layer.
  ///
  /// * `layer_name`: Either `None` (for global extension info) or a slice
  ///   pointing to null-terminated utf-8 data that names a layer you want the
  ///   extension properties of.
  ///
  /// A reference to the `layerName` field of a `VkLayerProperties` from
  /// [enumerate_instance_layer_properties][eilp] will work as a valid input to
  /// this function.
  ///
  /// [eilp]: [Self::enumerate_instance_layer_properties]
  ///
  /// See
  /// [vkEnumerateInstanceExtensionProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceExtensionProperties.html)
  ///
  /// ## Panics
  /// * This will panic if `layer_name` is `Some(name)` but the `name` doesn't
  ///   contain a zero.
  pub fn enumerate_instance_extension_properties(&self, layer_name: Option<&[u8]>) -> Result<Vec<VkExtensionProperties>, VkResult> {
    let name_ptr = match layer_name {
      None => null(),
      Some(name) => {
        assert!(name.contains(&0));
        name.as_ptr()
      }
    };
    let mut extensions_count = 0;
    let get_count_result = unsafe { (self.vkEnumerateInstanceExtensionProperties_p)(name_ptr, &mut extensions_count, null_mut()) };
    if get_count_result != VK_SUCCESS {
      return Err(get_count_result);
    }
    let mut v = Vec::with_capacity(extensions_count as usize);
    let get_properties_result = unsafe { (self.vkEnumerateInstanceExtensionProperties_p)(name_ptr, &mut extensions_count, v.as_mut_ptr()) };
    if get_properties_result != VK_SUCCESS {
      Err(get_properties_result)
    } else {
      unsafe { v.set_len(extensions_count as usize) };
      Ok(v)
    }
  }

  /// Creates a Vulkan Instance.
  ///
  /// See [vkCreateInstance](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateInstance.html)
  pub unsafe fn create_instance(&self, create_info: &VkInstanceCreateInfo, allocator: Option<&VkAllocationCallbacks>) -> Result<VkInstance, VkResult> {
    let mut instance = VkInstance::null();
    let create_result = (self.vkCreateInstance_p)(create_info, allocator, &mut instance);
    if create_result == VK_SUCCESS {
      Ok(instance)
    } else {
      Err(create_result)
    }
  }
}
