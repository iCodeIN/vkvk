use fermium::prelude::*;

use vkvk::prelude::*;

use core::convert::TryInto;

fn main() {
  unsafe {
    assert!(SDL_Init(SDL_INIT_EVERYTHING) == 0);

    let win = SDL_CreateWindow(b"demo\0".as_ptr().cast(), SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, 800, 600, (SDL_WINDOW_VULKAN | SDL_WINDOW_ALLOW_HIGHDPI).0);
    assert!(!win.is_null());

    let void_ptr = SDL_Vulkan_GetVkGetInstanceProcAddr();
    assert!(!void_ptr.is_null());

    let pif = PreInstanceFns::load_from(core::mem::transmute(void_ptr)).unwrap();

    let mut instance_layer_properties_count = 0;
    assert_eq!(pif.EnumerateInstanceLayerProperties(&mut instance_layer_properties_count, 0 as _), VK_SUCCESS);
    println!("There are {} instance layer properties...", instance_layer_properties_count);
    let mut instance_layer_properties = Vec::with_capacity(instance_layer_properties_count.try_into().unwrap());
    assert_eq!(pif.EnumerateInstanceLayerProperties(&mut instance_layer_properties_count, instance_layer_properties.as_mut_ptr()), VK_SUCCESS);
    instance_layer_properties.set_len(instance_layer_properties_count.try_into().unwrap());
    for (n, instance_layer_property) in instance_layer_properties.iter().enumerate() {
      println!("Layer {}: {:#?}", n, instance_layer_property);
    }

    let mut extension_properties_count = 0;
    assert_eq!(pif.EnumerateInstanceExtensionProperties(0 as _, &mut extension_properties_count, 0 as _), VK_SUCCESS);
    println!("There are {} extension properties in the Base Layer...", extension_properties_count);
    let mut extension_properties = Vec::with_capacity(extension_properties_count.try_into().unwrap());
    assert_eq!(pif.EnumerateInstanceExtensionProperties(0 as _, &mut extension_properties_count, extension_properties.as_mut_ptr()), VK_SUCCESS);
    extension_properties.set_len(extension_properties_count.try_into().unwrap());
    for (n, extension_property) in extension_properties.iter().enumerate() {
      println!("Layer(base) Extension {}: {:#?}", n, extension_property);
    }

    SDL_Quit();
  }
}
