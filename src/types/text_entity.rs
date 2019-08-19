
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a part of the text that needs to be formatted in some unusual way. 
#[derive(Debug, Serialize, Deserialize)]
pub struct TextEntity {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // textEntity
  /// Offset of the entity in UTF-16 code points.
  offset: Option<i32>,
  /// Length of the entity, in UTF-16 code points.
  length: Option<i32>,
  /// Type of the entity.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<Box<TextEntityType>>,
  
}


impl Clone for TextEntity {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for TextEntity {}
impl RObject for TextEntity {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntity" }
  fn td_type(&self) -> RTDType { RTDType::TextEntity }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl TextEntity {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "textEntity".to_string(),
      offset: None,
      length: None,
      type_: None,
      
    }
  }
  
  pub fn offset(&self) -> Option<i32> { self.offset.clone() }
  #[doc(hidden)] pub fn _set_offset(&mut self, offset: i32) -> &mut Self { self.offset = Some(offset); self }
  
  pub fn length(&self) -> Option<i32> { self.length.clone() }
  #[doc(hidden)] pub fn _set_length(&mut self, length: i32) -> &mut Self { self.length = Some(length); self }
  
  pub fn type_(&self) -> Option<Box<TextEntityType>> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: Box<TextEntityType>) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



