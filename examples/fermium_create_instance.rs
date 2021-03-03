use fermium::prelude::*;

use vkvk::prelude::*;

//use core::convert::TryInto;

pub unsafe fn gather_null_terminated_bytes(mut p: *const u8) -> Vec<u8> {
  let mut v = vec![];
  while *p != 0 {
    v.push(*p);
    p = p.add(1);
  }
  v
}

pub fn min_alloc_lossy_into_string(bytes: Vec<u8>) -> String {
  match String::from_utf8(bytes) {
    Ok(s) => s,
    Err(e) => String::from_utf8_lossy(e.as_bytes()).into_owned(),
  }
}

fn main() {
  unsafe {
    assert!(SDL_Init(SDL_INIT_EVERYTHING) == 0);

    let win = SDL_CreateWindow(
      b"demo\0".as_ptr().cast(),
      SDL_WINDOWPOS_CENTERED,
      SDL_WINDOWPOS_CENTERED,
      800,
      600,
      (SDL_WINDOW_VULKAN | SDL_WINDOW_ALLOW_HIGHDPI).0,
    );
    assert!(!win.is_null());

    let required_extensions: Vec<String> = {
      let mut required_extensions_count = 0;
      assert_eq!(SDL_Vulkan_GetInstanceExtensions(win, &mut required_extensions_count, 0 as _), SDL_TRUE);
      let mut v: Vec<*const c_char> = Vec::with_capacity(required_extensions_count as usize);
      assert_eq!(SDL_Vulkan_GetInstanceExtensions(win, &mut required_extensions_count, v.as_mut_ptr()), SDL_TRUE);
      v.set_len(required_extensions_count as usize);
      v.iter().copied().map(|p| min_alloc_lossy_into_string(gather_null_terminated_bytes(p.cast()))).collect()
    };
    println!("Required Extensions: {:?}", required_extensions);

    let void_ptr = SDL_Vulkan_GetVkGetInstanceProcAddr();
    assert!(!void_ptr.is_null());

    let pifr = PreInstanceFnsRusty(PreInstanceFns::load_from(core::mem::transmute(void_ptr)).unwrap());

    let mut requested_layers = Vec::new();
    let mut requested_extensions = Vec::new();

    let extension_properties = pifr.enumerate_instance_extension_properties(None).unwrap();
    for extension_property in extension_properties.iter().copied() {
      println!("Instance Extension({ext_name})> {ext:#?}", ext_name = extension_property.extension_name_str(), ext = extension_property);
      if required_extensions.iter().find(|s| s.as_str() == extension_property.extension_name_str()).is_some() {
        requested_extensions.push(extension_property);
      }
    }

    let instance_layer_properties = pifr.enumerate_instance_layer_properties().unwrap();
    for instance_layer_property in instance_layer_properties.iter().copied() {
      println!("Layer({layer_name})> {layer:#?}", layer_name = instance_layer_property.layer_name_str(), layer = instance_layer_property);
      if cfg!(debug_assertions) && ["VK_LAYER_KHRONOS_validation"].iter().find(|s| **s == instance_layer_property.layer_name_str()).is_some() {
        requested_layers.push(instance_layer_property);
      }

      let extension_properties = pifr.enumerate_instance_extension_properties(Some(&instance_layer_property.layerName)).unwrap();
      for extension_property in extension_properties.iter() {
        println!("Layer Extension({ext_name})> {ext:#?}", ext_name = extension_property.extension_name_str(), ext = extension_property);
      }
    }

    let instance =
      pifr.create_instance_simple("FermiumCreateInstance", VulkanVersion::new(1, 0, 0), &requested_layers, &requested_extensions).unwrap();
    let surface = 0_u64;
    assert_eq!(SDL_Vulkan_CreateSurface(win, core::mem::transmute(instance), &mut core::mem::transmute(surface)), SDL_TRUE);

    let ifr = InstanceFnsRusty(InstanceFns::load_from(pifr.0, instance).unwrap());

    SDL_Quit();
  }
}
