use super::*;

#[derive(Debug)]
pub struct TypeAlias {
  pub new: String,
  pub old: String,
}
impl core::fmt::Display for TypeAlias {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "pub type {new} = {old};", new = self.new, old = self.old)
  }
}

#[derive(Debug)]
pub struct Enumeration {
  pub name: String,
  pub entries: Vec<HashMap<String, String>>,
}
impl core::fmt::Display for Enumeration {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    // TODO
    write!(f, "#[repr(transparent)] pub struct {name}(pub i32);", name = self.name)
  }
}

#[derive(Debug)]
pub struct Bitmask {
  pub name: String,
  pub entries: Vec<HashMap<String, String>>,
}
impl core::fmt::Display for Bitmask {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    // TODO
    write!(f, "#[repr(transparent)] pub struct {name}(pub u32);", name = self.name)
  }
}

#[derive(Debug)]
pub struct Handle {
  pub name: String,
  pub objtypeenum: String,
  pub parent: Option<String>,
}
impl core::fmt::Display for Handle {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    // TODO
    write!(f, "#[repr(transparent)] pub struct {name}(*mut c_void);", name = self.name)
  }
}

#[derive(Debug)]
pub struct NonDispatchableHandle {
  pub name: String,
  pub objtypeenum: String,
  pub parent: Option<String>,
}
impl core::fmt::Display for NonDispatchableHandle {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    // TODO
    write!(f, "#[repr(transparent)] pub struct {name}(u64);", name = self.name)
  }
}

#[derive(Debug)]
pub struct FnPtrAlias {
  pub name: String,
  /// (name, ty)
  pub args: Vec<(String, String)>,
  pub return_type: String,
}
impl core::fmt::Display for FnPtrAlias {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let mut s = format!(
      "pub type {pfn_name} = Option<{name}_t>; pub type {name}_t = unsafe extern \"system\" fn(",
      pfn_name = self.name,
      name = &self.name[4..]
    );
    for arg in self.args.iter() {
      s.push_str(&format!("{arg_name}: {arg_ty}, ", arg_name = arg.0, arg_ty = arg.1));
    }
    s.push(')');
    if self.return_type != "void" {
      s.push_str(&format!(
        "-> {}",
        match self.return_type.as_str() {
          "void*" => "*mut c_void",
          other => other,
        }
      ));
    }
    s.push(';');
    write!(f, "{}", s)
  }
}

#[derive(Debug)]
pub struct Struct {
  pub name: String,
  pub structextends: Option<String>,
  pub fields: Vec<StructField>,
}
impl core::fmt::Display for Struct {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[derive(Debug)]
pub struct Union {
  pub name: String,
  pub fields: Vec<StructField>,
}
impl core::fmt::Display for Union {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[derive(Debug, Default)]
pub struct StructField {
  pub attrs: HashMap<String, String>,
  pub name: String,
  pub type_: String,
  pub is_ptr: bool,
  pub is_const: bool,
  pub array_count: Option<String>,
  pub comment: Option<String>,
}
impl core::fmt::Display for StructField {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[derive(Debug)]
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
    write!(f, "{:?}", self)
  }
}

#[derive(Debug, Default)]
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
impl core::fmt::Display for ProtoParam {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[derive(Debug)]
pub enum VulkanDefinition {
  TypeAlias(TypeAlias),
  Enumeration(Enumeration),
  Bitmask(Bitmask),
  Handle(Handle),
  NonDispatchableHandle(NonDispatchableHandle),
  FnPtrAlias(FnPtrAlias),
  Struct(Struct),
  Union(Union),
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
