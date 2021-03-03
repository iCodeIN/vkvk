use fermium::prelude::*;

use vkvk::prelude::*;

//use core::convert::TryInto;

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

    let void_ptr = SDL_Vulkan_GetVkGetInstanceProcAddr();
    assert!(!void_ptr.is_null());

    let pifr = PreInstanceFnsRusty(PreInstanceFns::load_from(core::mem::transmute(void_ptr)).unwrap());

    let extension_properties = pifr.enumerate_instance_extension_properties(None).unwrap();
    for extension_property in extension_properties.iter() {
      println!("Layer(-)>Extension({ext_name})> {ext:#?}", ext_name = extension_property.extension_name_str(), ext = extension_property);
    }

    let instance_layer_properties = pifr.enumerate_instance_layer_properties().unwrap();
    for instance_layer_property in instance_layer_properties.iter() {
      println!("Layer({layer_name})> {layer:#?}", layer_name = instance_layer_property.layer_name_str(), layer = instance_layer_property);

      let extension_properties = pifr.enumerate_instance_extension_properties(Some(&instance_layer_property.layerName)).unwrap();
      for extension_property in extension_properties.iter() {
        println!(
          "Layer({layer_name})>Extension({ext_name})> {ext:#?}",
          layer_name = instance_layer_property.layer_name_str(),
          ext_name = extension_property.extension_name_str(),
          ext = extension_property
        );
      }
    }

    SDL_Quit();
  }
}
