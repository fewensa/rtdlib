#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TdType {
  pub(crate) origin_name: String,
  pub(crate) clz_name: String,
  pub(crate) is_private: bool,
  pub(crate) clz_is_trait: bool,
  pub(crate) clz_description: Option<String>,
  pub(crate) has_subclasses: bool,
  pub(crate) sub_classes: Option<Vec<TdClz>>,
  pub(crate) clz_super: Option<String>,
  pub(crate) fields: Vec<TdTypeField>,
  pub(crate) fields_size: usize,
  pub(crate) has_trait_field: bool,
  pub(crate) set_trait_typetag: bool,
  pub(crate) impl_trait_typetag: bool,
  pub(crate) has_builder_struct: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TdTypeField {
  pub(crate) name: String,
  pub(crate) description: String,
  pub(crate) is_trait: bool,
  pub(crate) class: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TdTypeCommon {
  pub(crate) origin_names: Vec<String>,
  pub(crate) clzs: Vec<TdClz>,
  pub(crate) len: usize,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TdTypeSupplement {
  pub(crate) objects: Vec<TdTypeClassMapper>,
  pub(crate) functions: Vec<TdTypeClassMapper>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TdTypeClassMapper {
  pub(crate) origin_name: String,
  pub(crate) clz_name: String,
  pub(crate) is_trait: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TdClz {
  pub(crate) name: String,
  pub(crate) is_trait: bool
}
