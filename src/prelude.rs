//! Lets you grab the entire crate as a single, flat API.

pub use crate::{
  fn_managers::*,
  fn_managers_rusty::*,
  _1_0::{
    enumerations::*, flag_bits::*, fn_types::*, handles::*, non_dispatchable_handles::*, structures::*, unions::*, vk_platform::*, vk_version::*, *,
  },
};

#[cfg(feature = "_1_1")]
pub use crate::_1_1::*;
