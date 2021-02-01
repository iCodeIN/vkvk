#![no_std]
#![allow(bad_style)]
#![allow(dead_code)]
#![allow(unused)]

use core::ffi::c_void;

mod enumeration;
pub use enumeration::*;

mod flag_bits;
pub use flag_bits::*;

mod fn_ptr;
pub use fn_ptr::*;

mod handle;
pub use handle::*;

mod structure;
pub use structure::*;

mod version;
pub use version::*;

type TODO = ();
/// requires="X11/Xlib.h"
type Display = TODO;
/// requires="X11/Xlib.h"
type VisualID = TODO;
/// requires="X11/Xlib.h"
type Window = TODO;
/// requires="X11/extensions/Xrandr..h"
type RROutput = TODO;
/// requires="wayland-client..h"
type wl_display = TODO;
/// requires="wayland-client..h"
type wl_surface = TODO;
/// requires="windows.h"
type HINSTANCE = TODO;
/// requires="windows.h"
type HWND = TODO;
/// requires="windows.h"
type HMONITOR = TODO;
/// requires="windows.h"
type HANDLE = TODO;
/// requires="windows.h"
type SECURITY_ATTRIBUTES = TODO;
/// requires="windows.h"
type DWORD = TODO;
/// requires="windows.h"
type LPCWSTR = TODO;
/// requires="xcb/xcb.h"
type xcb_connection_t = TODO;
/// requires="xcb/xcb.h"
type xcb_visualid_t = TODO;
/// requires="xcb/xcb.h"
type xcb_window_t = TODO;
/// requires="directfb.h"
type IDirectFB = TODO;
/// requires="directfb.h"
type IDirectFBSurface = TODO;
/// requires="zircon/types.h"
type zx_handle_t = TODO;
/// requires="ggp_c/vulkan_types.h"
type GgpStreamDescriptor = TODO;
/// requires="ggp_c/vulkan_types.h"
type GgpFrameToken = TODO;

#[repr(transparent)]
pub struct ANativeWindow(c_void);
#[repr(transparent)]
pub struct AHardwareBuffer(c_void);
#[repr(transparent)]
pub struct CAMetalLayer(c_void);

// TODO: traits and such for these newtypes

pub struct VkSampleMask(pub uint32_t);
pub struct VkBool32(pub uint32_t);
pub struct VkDeviceSize(pub uint64_t);
pub struct VkDeviceAddress(pub uint64_t);

type void = c_void;
type char = u8;
type float = f32;
type double = f64;
type uint8_t = u8;
type uint16_t = u16;
type uint32_t = u32;
type uint64_t = u64;
type int32_t = i32;
type int64_t = i64;
type size_t = usize;
type int = i32;

pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
pub const VK_UUID_SIZE: usize = 16;
pub const VK_LUID_SIZE: usize = 8;
pub const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;
pub const VK_MAX_DESCRIPTION_SIZE: usize = 256;
pub const VK_MAX_MEMORY_TYPES: usize = 32;
/// The maximum number of unique memory heaps, each of which supporting 1 or
/// more memory types.
pub const VK_MAX_MEMORY_HEAPS: usize = 16;
pub const VK_LOD_CLAMP_NONE: f32 = 1000.0;
pub const VK_REMAINING_MIP_LEVELS: u32 = !0;
pub const VK_REMAINING_ARRAY_LAYERS: u32 = !0;
pub const VK_WHOLE_SIZE: u64 = !0;
pub const VK_ATTACHMENT_UNUSED: u32 = !0;
pub const VK_TRUE: VkBool32 = VkBool32(1);
pub const VK_FALSE: VkBool32 = VkBool32(0);
pub const VK_QUEUE_FAMILY_IGNORED: u32 = !0;
pub const VK_QUEUE_FAMILY_EXTERNAL: u32 = (!0) - 1;
pub const VK_QUEUE_FAMILY_FOREIGN_EXT: u32 = (!0) - 2;
pub const VK_SUBPASS_EXTERNAL: u32 = !0;
pub const VK_MAX_DEVICE_GROUP_SIZE: usize = 32;
pub const VK_MAX_DRIVER_NAME_SIZE: usize = 256;
pub const VK_MAX_DRIVER_INFO_SIZE: usize = 256;
pub const VK_SHADER_UNUSED_KHR: u32 = !0;

/// ALias for [`VK_QUEUE_FAMILY_EXTERNAL`]
pub const VK_QUEUE_FAMILY_EXTERNAL_KHR: u32 = VK_QUEUE_FAMILY_EXTERNAL;
/// ALias for [`VK_LUID_SIZE`]
pub const VK_LUID_SIZE_KHR: usize = VK_LUID_SIZE;
/// ALias for [`VK_MAX_DEVICE_GROUP_SIZE`]
pub const VK_MAX_DEVICE_GROUP_SIZE_KHR: usize = VK_MAX_DEVICE_GROUP_SIZE;
/// ALias for [`VK_MAX_DRIVER_NAME_SIZE`]
pub const VK_MAX_DRIVER_NAME_SIZE_KHR: usize = VK_MAX_DRIVER_NAME_SIZE;
/// ALias for [`VK_MAX_DRIVER_INFO_SIZE`]
pub const VK_MAX_DRIVER_INFO_SIZE_KHR: usize = VK_MAX_DRIVER_INFO_SIZE;
/// ALias for [`VK_SHADER_UNUSED_KHR`]
pub const VK_SHADER_UNUSED_NV: u32 = VK_SHADER_UNUSED_KHR;
