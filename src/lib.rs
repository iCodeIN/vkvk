#![no_std]
#![allow(bad_style)]
#![allow(dead_code)]
#![allow(unused)]

use core::ffi::c_void;

mod version;
pub use version::*;

mod bitmask;
pub use bitmask::*;

mod handle;
pub use handle::*;

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
type c_int = i32;
