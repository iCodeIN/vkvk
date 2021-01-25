use super::*;

#[derive(Debug)]
pub struct TypeAlias {
  pub new: String,
  pub old: String,
}

#[derive(Debug)]
pub struct Enumeration {
  pub name: String,
  pub entries: Vec<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct Bitmask {
  pub name: String,
  pub entries: Vec<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct Handle {
  pub name: String,
  pub objtypeenum: String,
  pub parent: Option<String>,
}

#[derive(Debug)]
pub struct NonDispatchableHandle {
  pub name: String,
  pub objtypeenum: String,
  pub parent: Option<String>,
}

#[derive(Debug)]
pub struct FnPtrAlias {
  pub name: String,
  /// (name, ty)
  pub args: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct Struct {
  pub name: String,
  pub structextends: Option<String>,
  pub fields: Vec<StructField>,
}

#[derive(Debug)]
pub struct Union {
  pub name: String,
  pub fields: Vec<StructField>,
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

#[derive(Debug)]
pub enum VulkanTypeDefinition {
  TypeAlias(TypeAlias),
  Enumeration(Enumeration),
  Bitmask(Bitmask),
  Handle(Handle),
  NonDispatchableHandle(NonDispatchableHandle),
  FnPtrAlias(FnPtrAlias),
  Struct(Struct),
  Union(Union),
}
