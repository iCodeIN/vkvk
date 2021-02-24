use super::*;

macro_rules! define_non_dispatchable_handle {
  ($(#[$m:meta])* $handle:ident) => {
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    $(#[$m])*
    pub struct $handle(u64);
    impl Copy for $handle { }
    impl Clone for $handle {
      fn clone(&self) -> Self {
        *self
      }
    }
    impl Default for $handle {
      fn default() -> Self {
        Self(0)
      }
    }
  };
}
