// Note(Lokathor): All the macro_rules! here should be `#[macro_export]` and
// `#[doc(hidden)]`, which lets us use them anywhere in our crate without
// actually having them show up in the generated docs.

#[macro_export]
#[doc(hidden)]
macro_rules! vk_struct {
  (
    $(#[$s_meta:meta])*
    $s_name:ident {
      $($(#[$f_meta:meta])* $f_name:ident: $f_ty:ty),*
      $(,)?
    }
  ) => {
    $(#[$s_meta])*
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct $s_name {
      $($(#[$f_meta])* pub $f_name: $f_ty),*
    }
  };
}

#[macro_export]
#[doc(hidden)]
macro_rules! vk_enumeration {
  (
    $(#[$e_meta:meta])*
    $enumeration:ident {
      $($(#[$c_meta:meta])* $i:ident = $val:expr),*
      $(,)?
    }
  ) => {
    #[derive(PartialEq, Eq, Hash)]
    #[repr(transparent)]
    $(#[$e_meta])*
    pub struct $enumeration(int);
    impl Copy for $enumeration { }
    impl Clone for $enumeration {
      fn clone(&self) -> Self {
        *self
      }
    }
    $( $(#[$c_meta])* pub const $i: $enumeration = $enumeration($val); )*
    #[cfg(feature = "impl_enumeration_precise_debug")]
    impl core::fmt::Debug for $enumeration {
      fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
          $($val => f.write_str(stringify!($i)),)*
          other => write!(f, concat!(stringify!($enumeration),"({})"), other),
        }
      }
    }
  };
}