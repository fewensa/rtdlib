
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a list of text entities. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEntities {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // textEntities
  /// List of text entities.
  entities: Option<Vec<TextEntity>>,
  
}



impl Object for TextEntities {}
impl RObject for TextEntities {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntities" }
  fn td_type(&self) -> RTDType { RTDType::TextEntities }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl TextEntities {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "textEntities".to_string(),
      entities: None,
      
    }
  }
  
  pub fn entities(&self) -> Option<Vec<TextEntity>> { self.entities.clone() }
  #[doc(hidden)] pub fn _set_entities(&mut self, entities: Vec<TextEntity>) -> &mut Self { self.entities = Some(entities); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



