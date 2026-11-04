use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;

/// token argument
#[derive(Debug, Clone, TypedBuilder, Serialize, Deserialize)]
pub struct TLTokenArgType {
  pub(crate) sign_name: String,
  pub(crate) sign_type: String,
  #[builder(default)]
  pub(crate) description: Option<String>,
  pub(crate) components: Vec<TLTokenComponentType>,
}

/// token component type
#[derive(Debug, Clone, TypedBuilder, Serialize, Deserialize)]
pub struct TLTokenComponentType {
  sign_type: String,
  components: Vec<TLTokenComponentType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TLTokenGroupType {
  Struct,
  Trait,
  Function,
}

/// tl schema token group
#[derive(Debug, Clone, TypedBuilder, Serialize, Deserialize)]
pub struct TLTokenGroup {
  /// all description, include trait/struct description and argument description
  pub(crate) description_all: Option<HashMap<String, String>>,
  /// this group description
  pub(crate) description: Option<String>,
  /// trait/struct name
  pub(crate) name: String,
  /// trait/struct argument map
  pub(crate) arguments: Vec<TLTokenArgType>,
  /// token group type
  pub(crate) type_: TLTokenGroupType,
  /// when type is struct, blood is super type
  ///      type is trait, blood is none
  ///      type is function, blood is return type
  pub(crate) blood: Option<String>,
  /// when type is struct, this may be true to
  /// indicate this can be returned be a function
  pub(crate) is_return_type: bool
}

impl TLTokenGroup {
  pub fn description_all (&self) -> Option<HashMap<String, String>>{ self.description_all.clone() }
  pub fn description     (&self) -> Option<String>                 { self.description    .clone() }
  pub fn name            (&self) -> String                         { self.name           .clone() }
  pub fn arguments       (&self) -> Vec<TLTokenArgType>            { self.arguments      .clone() }
  pub fn type_           (&self) -> TLTokenGroupType               { self.type_          .clone() }
  pub fn blood           (&self) -> Option<String>                 { self.blood          .clone() }
  pub fn is_return_type  (&self) -> bool                           { self.is_return_type .clone() }
}

impl TLTokenArgType {
  pub fn sign_name   (&self) -> String                     { self.sign_name  .clone() }
  pub fn sign_type   (&self) -> String                     { self.sign_type  .clone() }
  pub fn description (&self) -> Option<String>             { self.description.clone() }
  pub fn components  (&self) -> Vec<TLTokenComponentType>  { self.components .clone() }
}

impl TLTokenComponentType {
  pub fn sign_type   (&self) -> String                     { self.sign_type  .clone() }
  pub fn components  (&self) -> Vec<TLTokenComponentType>  { self.components .clone() }
}





