//! Handles are pointer-like values to objects that *can* accept commands.
//!
//! Because they're pointers, handles are either 32-bit or 64-bit depending on
//! the platform (the same size as `usize`).

use super::*;

macro_rules! define_handle {
  ($(#[$m:meta])* $handle:ident) => {
    #[derive(Debug, PartialEq, Eq)]
    #[cfg_attr(feature="derive_hash", derive(Hash))]
    #[repr(transparent)]
    $(#[$m])*
    pub struct $handle(*mut c_void);
    impl Copy for $handle { }
    impl Clone for $handle {
      fn clone(&self) -> Self {
        *self
      }
    }
    impl Default for $handle {
      fn default() -> Self {
        Self::null()
      }
    }
    impl $handle {
      pub const fn null() -> Self {
        Self(core::ptr::null_mut())
      }
      pub fn is_null(&self) -> bool {
        self.0.is_null()
      }
    }
  };
}
