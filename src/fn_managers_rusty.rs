use crate::prelude::*;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use core::{
  convert::TryInto,
  ptr::{null, null_mut},
};

pub struct PreInstanceFnsRusty(pub PreInstanceFns);

impl core::ops::Deref for PreInstanceFnsRusty {
  type Target = PreInstanceFns;
  fn deref(&self) -> &PreInstanceFns {
    &self.0
  }
}

impl PreInstanceFnsRusty {
  /// Gets info about the available layers.
  #[cfg(feature = "alloc")]
  pub fn enumerate_instance_layer_properties(&self) -> Result<Vec<VkLayerProperties>, VkResult> {
    unsafe {
      let mut tmp = 0;
      let get_capacity_result = self.EnumerateInstanceLayerProperties(&mut tmp, null_mut());
      if get_capacity_result != VK_SUCCESS {
        return Err(get_capacity_result);
      }
      let mut v = Vec::with_capacity(tmp.try_into().unwrap());
      let get_data_result = self.EnumerateInstanceLayerProperties(&mut tmp, v.as_mut_ptr());
      if get_data_result != VK_SUCCESS {
        return Err(get_data_result);
      } else {
        v.set_len(tmp.try_into().unwrap());
        Ok(v)
      }
    }
  }

  /// Gets info about the extensions available per layer.
  ///
  /// Extensions are specific to a given layer, or to the "base" instance.
  /// * Use `Some(layer_name)` to get extension info for the named layer (layer
  ///   names come from [enumerate_instance_layer_properties][1])
  /// * Use `None` to get extension info about the base instance.
  ///
  /// [1]: Self::enumerate_instance_layer_properties
  #[cfg(feature = "alloc")]
  pub fn enumerate_instance_extension_properties(
    &self, layer_name: Option<&[char; VK_MAX_EXTENSION_NAME_SIZE]>,
  ) -> Result<Vec<VkExtensionProperties>, VkResult> {
    let name_ptr = match layer_name {
      None => null(),
      Some(r) => {
        assert!(r.contains(&0));
        r.as_ptr()
      }
    };
    unsafe {
      let mut tmp = 0;
      let get_capacity_result = self.EnumerateInstanceExtensionProperties(name_ptr, &mut tmp, null_mut());
      if get_capacity_result != VK_SUCCESS {
        return Err(get_capacity_result);
      }
      let mut v = Vec::with_capacity(tmp.try_into().unwrap());
      let get_data_result = self.EnumerateInstanceExtensionProperties(name_ptr, &mut tmp, v.as_mut_ptr());
      if get_data_result != VK_SUCCESS {
        return Err(get_data_result);
      } else {
        v.set_len(tmp.try_into().unwrap());
        Ok(v)
      }
    }
  }

  #[cfg(feature = "alloc")]
  pub fn create_instance_simple(
    &self, name: &str, api: VulkanVersion, layers: &[VkLayerProperties], extensions: &[VkExtensionProperties],
  ) -> Result<VkInstance, VkResult> {
    let name_null: Vec<u8> = name.as_bytes().iter().copied().chain(Some(0)).collect();
    let application_info = VkApplicationInfo {
      pApplicationName: name_null.as_ptr().cast(),
      applicationVersion: 1,
      engineVersion: 1,
      apiVersion: api,
      ..Default::default()
    };
    let requested_layers: Vec<*const u8> = layers
      .iter()
      .map(|la| {
        assert!(la.layerName.contains(&0));
        la.layerName.as_ptr()
      })
      .collect();
    let requested_extensions: Vec<*const u8> = extensions
      .iter()
      .map(|ex| {
        assert!(ex.extensionName.contains(&0));
        ex.extensionName.as_ptr()
      })
      .collect();
    let create_info = VkInstanceCreateInfo {
      pApplicationInfo: &application_info,
      enabledLayerCount: requested_layers.len() as _,
      ppEnabledLayerNames: requested_layers.as_ptr(),
      enabledExtensionCount: requested_extensions.len() as _,
      ppEnabledExtensionNames: requested_extensions.as_ptr(),
      ..Default::default()
    };
    let mut instance = VkInstance::null();
    let create_instance_result = unsafe { self.CreateInstance(&create_info, None, &mut instance) };
    if create_instance_result == VK_SUCCESS {
      Ok(instance)
    } else {
      Err(create_instance_result)
    }
  }
}

pub struct InstanceFnsRusty(pub InstanceFns);

impl core::ops::Deref for InstanceFnsRusty {
  type Target = InstanceFns;
  fn deref(&self) -> &InstanceFns {
    &self.0
  }
}

impl InstanceFnsRusty {
  //
}
