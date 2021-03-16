#![no_std]
#![allow(non_camel_case_types)]

//! Don't use this crate. Use [erupt](https://docs.rs/erupt) if you need Vulkan bindings.

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
mod macros;

/// Makes a `&str` from all bytes before the first `0` within the byte slice.
///
/// If the slice has no `0` then the entire slice is used.
pub fn str_from_null_terminated_byte_slice(bytes: &[u8]) -> Result<&str, core::str::Utf8Error> {
  let terminal_position = bytes.iter().copied().position(|u| u == 0).unwrap_or(bytes.len());
  core::str::from_utf8(&bytes[..terminal_position])
}

pub mod prelude;

pub mod fn_managers;

pub mod fn_managers_rusty;

pub mod _1_0;

#[cfg(feature = "_1_1")]
pub mod _1_1;
