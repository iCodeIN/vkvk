use super::*;

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

  /// Gets info about the extensions available.
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

  //
}
