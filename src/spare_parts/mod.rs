use super::*;

pub use core::ffi::c_void;

// Note(Lokathor): Do not make these types pub!
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

/// requires="X11/Xlib.h"
pub type Display = *mut c_void;
/// requires="X11/Xlib.h"
pub type VisualID = chlorine::c_ulong;
/// requires="X11/Xlib.h"
pub type Window = chlorine::c_ulong;
/// requires="X11/extensions/Xrandr.h"
pub type RROutput = chlorine::c_ulong;

/// requires="wayland-client.h"
pub type wl_display = c_void;
/// requires="wayland-client.h"
pub type wl_surface = c_void;

/// requires="windows.h"
pub type HINSTANCE = *mut c_void;
/// requires="windows.h"
pub type HWND = *mut c_void;
/// requires="windows.h"
pub type HMONITOR = *mut c_void;
/// requires="windows.h"
pub type HANDLE = *mut c_void;
/// requires="windows.h"
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SECURITY_ATTRIBUTES {
  pub nLength: u32,
  pub lpSecurityDescriptor: *mut void,
  pub bInheritHandle: u32,
}
/// requires="windows.h"
pub type DWORD = u32;
/// requires="windows.h"
pub type LPCWSTR = *mut u16;

/// requires="xcb/xcb.h"
pub type xcb_connection_t = c_void;
/// requires="xcb/xcb.h"
pub type xcb_visualid_t = u32;
/// requires="xcb/xcb.h"
pub type xcb_window_t = u32;

/// requires="directfb.h"
pub type IDirectFB = c_void;
/// requires="directfb.h"
pub type IDirectFBSurface = c_void;

/// requires="zircon/types.h"
pub type zx_handle_t = uint32_t;

#[cfg(feature = "google_games_platform")]
/// requires="ggp_c/vulkan_types.h"
type GgpStreamDescriptor = TODO;
#[cfg(feature = "google_games_platform")]
/// requires="ggp_c/vulkan_types.h"
type GgpFrameToken = TODO;

/// Android Native Window
#[repr(transparent)]
pub struct ANativeWindow(c_void);
/// Android Hardware Buffer
#[repr(transparent)]
pub struct AHardwareBuffer(c_void);
/// Core Animation Metal Layer
#[repr(transparent)]
pub struct CAMetalLayer(c_void);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkBool32(pub uint32_t);
pub const VK_TRUE: VkBool32 = VkBool32(1);
pub const VK_FALSE: VkBool32 = VkBool32(0);
impl From<bool> for VkBool32 {
  fn from(b: bool) -> Self {
    Self(b as _)
  }
}
impl From<VkBool32> for bool {
  fn from(vk_b: VkBool32) -> Self {
    vk_b.0 != 0
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkSampleMask(pub uint32_t);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkDeviceSize(pub uint64_t);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VkDeviceAddress(pub uint64_t);

pub const VK_LUID_SIZE: usize = 8;
pub const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;
pub const VK_MAX_DESCRIPTION_SIZE: usize = 256;
pub const VK_QUEUE_FAMILY_EXTERNAL: u32 = (!0) - 1;
pub const VK_QUEUE_FAMILY_FOREIGN_EXT: u32 = (!0) - 2;
pub const VK_MAX_DEVICE_GROUP_SIZE: usize = 32;
pub const VK_MAX_DRIVER_NAME_SIZE: usize = 256;
pub const VK_MAX_DRIVER_INFO_SIZE: usize = 256;
pub const VK_SHADER_UNUSED_KHR: u32 = !0;

/// Alias for [`VK_QUEUE_FAMILY_EXTERNAL`]
pub const VK_QUEUE_FAMILY_EXTERNAL_KHR: u32 = VK_QUEUE_FAMILY_EXTERNAL;
/// Alias for [`VK_LUID_SIZE`]
pub const VK_LUID_SIZE_KHR: usize = VK_LUID_SIZE;
/// Alias for [`VK_MAX_DEVICE_GROUP_SIZE`]
pub const VK_MAX_DEVICE_GROUP_SIZE_KHR: usize = VK_MAX_DEVICE_GROUP_SIZE;
/// Alias for [`VK_MAX_DRIVER_NAME_SIZE`]
pub const VK_MAX_DRIVER_NAME_SIZE_KHR: usize = VK_MAX_DRIVER_NAME_SIZE;
/// Alias for [`VK_MAX_DRIVER_INFO_SIZE`]
pub const VK_MAX_DRIVER_INFO_SIZE_KHR: usize = VK_MAX_DRIVER_INFO_SIZE;
/// Alias for [`VK_SHADER_UNUSED_KHR`]
pub const VK_SHADER_UNUSED_NV: u32 = VK_SHADER_UNUSED_KHR;
