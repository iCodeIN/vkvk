// Note(Lokathor): All the macro_rules! here should be `#[macro_export]` and
// `#[doc(hidden)]`, which lets us use them anywhere in our crate without
// actually having them show up in the generated docs.

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
    #[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
    #[cfg_attr(not(feature = "impl_enumeration_precise_debug"), derive(Debug))]
    #[repr(transparent)]
    $(#[$e_meta])*
    pub struct $enumeration(pub int);
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

#[macro_export]
#[doc(hidden)]
macro_rules! flag_bits {
  (
    $(#[$s_meta:meta])*
    $bits_name:ident = $flags_name:ident {
      $($(#[$c_meta:meta])* $i:ident = $val:expr),*
      $(,)?
    }
  ) => {
    pub type $flags_name = $bits_name;
    #[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
    #[cfg_attr(not(feature = "impl_flag_bits_precise_debug"), derive(Debug))]
    #[repr(transparent)]
    $(#[$s_meta])*
    pub struct $bits_name(pub u32);
    #[cfg(feature = "impl_flag_bits_precise_debug")]
    impl core::fmt::Debug for $bits_name {
      fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.0 != 0 {
          let mut printed_yet = false;
          f.write_str(concat!(stringify!($bits_name)," {"))?;
          $(if self.0 & $val > 0 {
            if printed_yet {
              f.write_str(", ")?;
            }
            f.write_str(stringify!($i))?;
            #[allow(unused_assignments)]
            {
              printed_yet = true;
            }
          })*
          f.write_str(" }")
        } else {
          f.write_str(concat!(stringify!($bits_name)," { none }"))
        }
      }
    }
    impl core::ops::BitAnd for $bits_name {
      type Output = Self;
      fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
      }
    }
    impl core::ops::BitAndAssign for $bits_name {
      fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
      }
    }
    impl core::ops::BitOr for $bits_name {
      type Output = Self;
      fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
      }
    }
    impl core::ops::BitOrAssign for $bits_name {
      fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
      }
    }
    impl core::ops::BitXor for $bits_name {
      type Output = Self;
      fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
      }
    }
    impl core::ops::BitXorAssign for $bits_name {
      fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
      }
    }
    impl core::ops::Not for $bits_name {
      type Output = Self;
      fn not(self) -> Self::Output {
        Self(!self.0)
      }
    }
    $( #[allow(unused_parens)] $(#[$c_meta])* pub const $i: $bits_name = $bits_name($val); )*
  };
}
