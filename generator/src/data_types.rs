use super::*;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct TypeAlias {
  pub new: String,
  pub old: String,
}
impl core::fmt::Display for TypeAlias {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    writeln!(f, "/// [{new}](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/{new}.html)", new = self.new)?;
    writeln!(f, "pub type {old} = {new};", new = self.new, old = self.old)
  }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct FnPtrAlias {
  pub name: String,
  /// (name, ty)
  pub args: Vec<(String, String)>,
  pub return_type: String,
}
impl core::fmt::Display for FnPtrAlias {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    writeln!(f, "/// Nullable form of [{pfn_name}](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/{pfn_name}.html)", pfn_name = self.name)?;
    writeln!(f, "pub type {pfn_name} = Option<{name}_t>;", pfn_name = self.name, name = &self.name[4..])?;
    writeln!(f, "/// Non-nullable form of [{pfn_name}](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/{pfn_name}.html)", pfn_name = self.name)?;
    let mut s = format!("pub type {name}_t = unsafe extern \"system\" fn(", name = &self.name[4..]);
    for arg in self.args.iter() {
      s.push_str(&format!("{arg_name}: {arg_ty}, ", arg_name = arg.0, arg_ty = arg.1));
    }
    if self.args.len() > 0 {
      s.pop();
      s.pop();
    }
    s.push(')');
    if self.return_type != "void" {
      s.push_str(&format!(
        " -> {}",
        match self.return_type.as_str() {
          "void*" => "*mut c_void",
          other => other,
        }
      ));
    }
    s.push(';');
    writeln!(f, "{}", s)
  }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct Handle {
  pub name: String,
  pub objtypeenum: String,
  pub parent: Option<String>,
}
impl core::fmt::Display for Handle {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    writeln!(f, "/// (Handle) [{name}](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/{name}.html)", name = self.name)?;
    writeln!(f, "///")?;
    writeln!(f, "/// [`VkObjectType`] value: [`{objtypeenum}`]", objtypeenum = self.objtypeenum)?;
    writeln!(f, "#[repr(transparent)]")?;
    writeln!(f, "pub struct {name}(*mut c_void);", name = self.name)?;
    writeln!(f, "impl Copy for {name} {{}} impl Clone for {name}{{ fn clone(&self) -> Self {{ *self }} }}", name = self.name)?;
    Ok(())
  }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct NonDispatchableHandle {
  pub name: String,
  pub objtypeenum: String,
  pub parent: Option<String>,
}
impl core::fmt::Display for NonDispatchableHandle {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    writeln!(f, "/// (Non-dispatchable Handle) [{name}](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/{name}.html)", name = self.name)?;
    writeln!(f, "///")?;
    writeln!(f, "/// [`VkObjectType`] value: [`{objtypeenum}`]", objtypeenum = self.objtypeenum)?;
    writeln!(f, "#[repr(transparent)]")?;
    writeln!(f, "pub struct {name}(*mut c_void);", name = self.name)?;
    writeln!(f, "impl Copy for {name} {{}} impl Clone for {name}{{ fn clone(&self) -> Self {{ *self }} }}", name = self.name)?;
    Ok(())
  }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct Enumeration {
  pub name: String,
  pub entries: Vec<EnumerationEntry>,
}
impl core::fmt::Display for Enumeration {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    writeln!(f, "/// (Enumeration) [{name}](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/{name}.html)", name = self.name)?;
    writeln!(f, "#[repr(transparent)]")?;
    writeln!(f, "pub struct {name}(pub i32);", name = self.name)?;
    writeln!(f, "impl Copy for {name} {{}} impl Clone for {name}{{ fn clone(&self) -> Self {{ *self }} }}", name = self.name)?;
    for entry in self.entries.iter() {
      if let Some(comment) = entry.comment.as_ref() {
        writeln!(f, "/// {comment}", comment = comment)?;
      }
      match (entry.value.as_ref(), entry.alias.as_ref()) {
        (Some(value), None) => {
          writeln!(f, "pub const {name}: {ty} = {ty}({value});", name = entry.name, value = value, ty = self.name)?;
        }
        (None, Some(alias)) => {
          writeln!(f, "pub const {name}: {ty} = {alias};", name = entry.name, ty = self.name, alias = alias)?;
        }
        _ => {
          panic!("Can't Display Entry: {:?}", entry)
        }
      }
    }
    Ok(())
  }
}

#[derive(Debug, Default, PartialEq, PartialOrd, Ord, Eq)]
pub struct EnumerationEntry {
  pub name: String,
  pub value: Option<String>,
  pub alias: Option<String>,
  pub comment: Option<String>,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct Bitmask {
  pub name: String,
  pub entries: Vec<BitmaskEntry>,
}
impl core::fmt::Display for Bitmask {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    writeln!(f, "/// (Bitmask) [{name}](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/{name}.html)", name = self.name)?;
    writeln!(f, "#[repr(transparent)]")?;
    writeln!(f, "pub struct {name}(pub u32);", name = self.name)?;
    writeln!(f, "pub type {sort_name}s = {name};", sort_name = &self.name[..(self.name.len() - 4)], name = self.name)?;
    writeln!(f, "impl Copy for {name} {{}} impl Clone for {name}{{ fn clone(&self) -> Self {{ *self }} }}", name = self.name)?;
    writeln!(f, "impl VkFlags for {name} {{", name = self.name)?;
    writeln!(f, "  fn from_u32(u: u32) -> Self {{ Self(u) }}")?;
    writeln!(f, "  fn to_u32(self) -> u32 {{ self.0 }}")?;
    writeln!(f, "}}")?;
    for entry in self.entries.iter() {
      if let Some(comment) = entry.comment.as_ref() {
        writeln!(f, "/// {comment}", comment = comment)?;
      }
      match (entry.value.as_ref(), entry.alias.as_ref(), entry.bit_position.as_ref()) {
        (Some(value), None, None) => {
          writeln!(f, "pub const {name}: {ty} = {ty}({value});", name = entry.name, value = value, ty = self.name)?;
        }
        (None, Some(alias), None) => {
          writeln!(f, "pub const {name}: {ty} = {alias};", name = entry.name, alias = alias, ty = self.name)?;
        }
        (None, None, Some(bit_position)) => {
          writeln!(f, "pub const {name}: {ty} = {ty}(1<<{bit_position});", name = entry.name, bit_position = bit_position, ty = self.name)?;
        }
        _ => {
          panic!("Can't Display Entry: {:?}", entry)
        }
      }
    }
    Ok(())
  }
}

#[derive(Debug, Default, PartialEq, PartialOrd, Ord, Eq)]
pub struct BitmaskEntry {
  pub name: String,
  pub bit_position: Option<String>,
  pub value: Option<String>,
  pub alias: Option<String>,
  pub comment: Option<String>,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct Struct {
  pub name: String,
  pub structextends: Option<String>,
  pub fields: Vec<StructField>,
}
#[cfg(FALSE)]
impl core::fmt::Display for Struct {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    writeln!(f, "/// [{name}](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/{name}.html)", name = self.name)?;
    if let Some(struct_extends) = self.structextends.as_ref() {
      writeln!(f, "///")?;
      write!(f, "/// Struct Extends: ")?;
      for (i, ex) in struct_extends.split(',').enumerate() {
        if i > 0 {
          write!(f, ", ")?;
        }
        write!(f, "[`{ex}`]", ex = ex)?;
      }
      writeln!(f)?;
    }
    writeln!(f, "#[repr(C)]")?;
    writeln!(f, "pub struct {name} {{", name = self.name)?;
    for field in self.fields.iter() {
      write!(f, "{}", field)?;
    }
    writeln!(f, "}}")?;
    writeln!(f, "impl Copy for {name} {{}} impl Clone for {name}{{ fn clone(&self) -> Self {{ *self }} }}", name = self.name)?;
    Ok(())
  }
}
impl core::fmt::Display for Struct {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    writeln!(f, "structure! {{")?;
    writeln!(f, "  /// [{name}](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/{name}.html)", name = self.name)?;
    if let Some(struct_extends) = self.structextends.as_ref() {
      writeln!(f, "  ///")?;
      write!(f, "  /// Struct Extends: ")?;
      for (i, ex) in struct_extends.split(',').enumerate() {
        if i > 0 {
          write!(f, ", ")?;
        }
        write!(f, "[`{ex}`]", ex = ex)?;
      }
      writeln!(f)?;
    }
    writeln!(f, "  {name} {{", name = self.name)?;
    for field in self.fields.iter() {
      write!(f, "{}", field)?;
    }
    writeln!(f, "  }}")?;
    writeln!(f, "}}")?;
    Ok(())
  }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct Union {
  pub name: String,
  pub fields: Vec<StructField>,
}
impl core::fmt::Display for Union {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    writeln!(f, "unionize! {{")?;
    writeln!(f, "  /// [{name}](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/{name}.html)", name = self.name)?;
    writeln!(f, "  {name} {{", name = self.name)?;
    for field in self.fields.iter() {
      write!(f, "{}", field)?;
    }
    writeln!(f, "  }}")?;
    writeln!(f, "}}")?;
    Ok(())
  }
}

#[derive(Debug, Default, PartialEq, PartialOrd, Ord, Eq)]
pub struct StructField {
  pub name: String,
  pub type_: String,
  pub values: Option<String>,
  pub optional: Option<String>,
  pub no_auto_validity: Option<String>,
  pub len: Option<String>,
  pub alt_len: Option<String>,
  pub extern_sync: Option<String>,
  pub selection: Option<String>,
  pub selector: Option<String>,
  pub is_ptr: bool,
  pub is_const: bool,
  pub array_count: Option<String>,
  pub comment: Option<String>,
}
#[cfg(FALSE)]
impl core::fmt::Display for StructField {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(comment) = self.comment.as_ref() {
      writeln!(f, "  /// {comment}", comment = comment)?;
    }
    if let Some(values) = self.values.as_ref() {
      writeln!(f, "  /// * **Values:** [`{values}`]", values = values)?;
    }
    if let Some(optional) = self.optional.as_ref() {
      writeln!(f, "  /// * **Optional:** {optional}", optional = optional)?;
    }
    if let Some(no_auto_validity) = self.no_auto_validity.as_ref() {
      writeln!(f, "  /// * **No Auto-validity:** {no_auto_validity}", no_auto_validity = no_auto_validity)?;
    }
    if let Some(len) = self.len.as_ref() {
      writeln!(f, "  /// * **Len:** {len}", len = len)?;
    }
    if let Some(alt_len) = self.alt_len.as_ref() {
      writeln!(f, "  /// * **Alt Len:** {alt_len}", alt_len = alt_len)?;
    }
    if let Some(extern_sync) = self.extern_sync.as_ref() {
      writeln!(f, "  /// * **Extern Sync:** {extern_sync}", extern_sync = extern_sync)?;
    }
    if let Some(selection) = self.selection.as_ref() {
      writeln!(f, "  /// * **Selection:** {selection}", selection = selection)?;
    }
    if let Some(selector) = self.selector.as_ref() {
      writeln!(f, "  /// * **Selector:** {selector}", selector = selector)?;
    }
    let name = if self.name == "type" { "type_" } else { self.name.as_str() };
    let ptr_prefix = match (self.is_ptr, self.is_const) {
      (true, true) => "*const ",
      (true, false) => "*mut ",
      (false, true) => unimplemented!(),
      (false, false) => "",
    };
    let ty = match self.array_count.as_ref() {
      None => format!("{ptr_prefix}{base_ty}", ptr_prefix = ptr_prefix, base_ty = self.type_),
      Some(array_count) => {
        if array_count.starts_with('V') {
          format!("{ptr_prefix}[{base_ty}; {count} as usize]", ptr_prefix = ptr_prefix, base_ty = self.type_, count = array_count)
        } else if array_count.starts_with(':') {
          format!("{ptr_prefix}[{base_ty}; {count}]", ptr_prefix = ptr_prefix, base_ty = self.type_, count = &array_count[1..])
        } else if array_count.starts_with('[') {
          if array_count.contains("][") {
            format!("{ptr_prefix}[[{base_ty}; {count1}]; {count2}]", ptr_prefix = ptr_prefix, base_ty = self.type_, count2 = &array_count[1..2], count1 = &array_count[4..5])
          } else {
            format!("{ptr_prefix}[{base_ty}; {count}]", ptr_prefix = ptr_prefix, base_ty = self.type_, count = &array_count[1..2])
          }
        } else {
          format!("{ptr_prefix}[{base_ty}; {count}]", ptr_prefix = ptr_prefix, base_ty = self.type_, count = array_count)
        }
      }
    };
    writeln!(f, "  pub {name}: {ty},", name = name, ty = ty)
  }
}
impl core::fmt::Display for StructField {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(comment) = self.comment.as_ref() {
      writeln!(f, "    /// {comment}", comment = comment)?;
    }
    if let Some(values) = self.values.as_ref() {
      writeln!(f, "    /// * **Values:** [`{values}`]", values = values)?;
    }
    if let Some(optional) = self.optional.as_ref() {
      writeln!(f, "    /// * **Optional:** {optional}", optional = optional)?;
    }
    if let Some(no_auto_validity) = self.no_auto_validity.as_ref() {
      writeln!(f, "    /// * **No Auto-validity:** {no_auto_validity}", no_auto_validity = no_auto_validity)?;
    }
    if let Some(len) = self.len.as_ref() {
      writeln!(f, "    /// * **Len:** {len}", len = len)?;
    }
    if let Some(alt_len) = self.alt_len.as_ref() {
      writeln!(f, "    /// * **Alt Len:** {alt_len}", alt_len = alt_len)?;
    }
    if let Some(extern_sync) = self.extern_sync.as_ref() {
      writeln!(f, "    /// * **Extern Sync:** {extern_sync}", extern_sync = extern_sync)?;
    }
    if let Some(selection) = self.selection.as_ref() {
      writeln!(f, "    /// * **Selection:** {selection}", selection = selection)?;
    }
    if let Some(selector) = self.selector.as_ref() {
      writeln!(f, "    /// * **Selector:** {selector}", selector = selector)?;
    }
    let name = if self.name == "type" { "type_" } else { self.name.as_str() };
    let ptr_prefix = match (self.is_ptr, self.is_const) {
      (true, true) => "*const ",
      (true, false) => "*mut ",
      (false, true) => unimplemented!(),
      (false, false) => "",
    };
    let ty = match self.array_count.as_ref() {
      None => format!("{ptr_prefix}{base_ty}", ptr_prefix = ptr_prefix, base_ty = self.type_),
      Some(array_count) => {
        if array_count.starts_with('V') {
          format!("{ptr_prefix}[{base_ty}; {count} as usize]", ptr_prefix = ptr_prefix, base_ty = self.type_, count = array_count)
        } else if array_count.starts_with(':') {
          format!("{ptr_prefix}[{base_ty}; {count}]", ptr_prefix = ptr_prefix, base_ty = self.type_, count = &array_count[1..])
        } else if array_count.starts_with('[') {
          if array_count.contains("][") {
            format!("{ptr_prefix}[[{base_ty}; {count1}]; {count2}]", ptr_prefix = ptr_prefix, base_ty = self.type_, count2 = &array_count[1..2], count1 = &array_count[4..5])
          } else {
            format!("{ptr_prefix}[{base_ty}; {count}]", ptr_prefix = ptr_prefix, base_ty = self.type_, count = &array_count[1..2])
          }
        } else {
          format!("{ptr_prefix}[{base_ty}; {count}]", ptr_prefix = ptr_prefix, base_ty = self.type_, count = array_count)
        }
      }
    };
    writeln!(f, "    {name}: {ty},", name = name, ty = ty)
  }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct FunctionPrototype {
  pub success_codes: Vec<String>,
  pub error_codes: Vec<String>,
  pub queues: Vec<String>,
  pub render_passes: Vec<String>,
  pub cmd_buffer_levels: Vec<String>,
  pub pipelines: Vec<String>,
  pub comment: Option<String>,
  pub fn_name: String,
  pub return_type: String,
  pub params: Vec<ProtoParam>,
  pub implicit_extern_sync_params: Option<String>,
}
impl core::fmt::Display for FunctionPrototype {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    writeln!(f, "/// Nullable pointer to [{name}](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/{name}.html)", name = self.fn_name)?;
    writeln!(f, "pub type PFN_{name} = Option<{name}_t>;", name = self.fn_name)?;
    writeln!(f, "/// Non-nullable pointer to [{name}](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/{name}.html)", name = self.fn_name)?;
    let mut args = String::new();
    for arg in self.params.iter() {
      let arg_name = arg.get_name();
      if arg.fixed_len > 0 {
        args.push_str(&format!("{arg_name}: *const [{arg_ty}; {fixed}], ", arg_name = arg_name, arg_ty = arg.param_type, fixed = arg.fixed_len));
      } else if arg.star_count > 0 {
        let mutability = if arg.const_count == arg.star_count { "const" } else { "mut" };
        let arg_ty = if arg.param_type == "void" { "c_void" } else { &arg.param_type };
        args.push_str(&format!("{arg_name}: *{mutability} {arg_ty}, ", arg_name = arg_name, arg_ty = arg_ty, mutability = mutability));
      } else {
        args.push_str(&format!("{arg_name}: {arg_ty}, ", arg_name = arg_name, arg_ty = arg.param_type));
      }
    }
    if self.params.len() > 0 {
      args.pop();
      args.pop();
    }
    let ret = if self.return_type != "void" { format!(" -> {}", self.return_type) } else { String::new() };
    writeln!(f, "pub type {name}_t = unsafe extern \"system\" fn({args}){ret};", name = self.fn_name, args = args, ret = ret)?;
    Ok(())
  }
}

#[derive(Debug, Default, PartialEq, PartialOrd, Ord, Eq)]
pub struct ProtoParam {
  pub optional: Option<Vec<String>>,
  pub extern_sync: Option<Vec<String>>,
  pub len: Option<String>,
  pub no_auto_validity: Option<Vec<String>>,
  pub const_count: usize,
  pub star_count: usize,
  pub param_type: String,
  pub param_name: String,
  pub fixed_len: usize,
}
impl ProtoParam {
  pub fn get_name(&self) -> String {
    if self.param_name == "type" {
      format!("type_")
    } else {
      format!("{}", self.param_name)
    }
  }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum VulkanDefinition {
  Handle(Handle),
  NonDispatchableHandle(NonDispatchableHandle),
  Enumeration(Enumeration),
  Bitmask(Bitmask),
  Struct(Struct),
  Union(Union),
  TypeAlias(TypeAlias),
  FnPtrAlias(FnPtrAlias),
  FunctionPrototype(FunctionPrototype),
}
impl core::fmt::Display for VulkanDefinition {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match self {
      VulkanDefinition::TypeAlias(x) => write!(f, "{}", x),
      VulkanDefinition::Enumeration(x) => write!(f, "{}", x),
      VulkanDefinition::Bitmask(x) => write!(f, "{}", x),
      VulkanDefinition::Handle(x) => write!(f, "{}", x),
      VulkanDefinition::NonDispatchableHandle(x) => write!(f, "{}", x),
      VulkanDefinition::FnPtrAlias(x) => write!(f, "{}", x),
      VulkanDefinition::Struct(x) => write!(f, "{}", x),
      VulkanDefinition::Union(x) => write!(f, "{}", x),
      VulkanDefinition::FunctionPrototype(x) => write!(f, "{}", x),
    }
  }
}

#[derive(Debug, Default)]
pub struct Registry {
  pub definitions: Vec<VulkanDefinition>,
}
