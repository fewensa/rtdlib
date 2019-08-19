
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A text with some entities. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattedText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // formattedText
  /// The text.
  text: Option<String>,
  /// Entities contained in the text.
  entities: Option<Vec<TextEntity>>,
  
}



impl Object for FormattedText {}
impl RObject for FormattedText {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "formattedText" }
  fn td_type(&self) -> RTDType { RTDType::FormattedText }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl FormattedText {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "formattedText".to_string(),
      text: None,
      entities: None,
      
    }
  }
  
  pub fn text(&self) -> Option<String> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: String) -> &mut Self { self.text = Some(text); self }
  
  pub fn entities(&self) -> Option<Vec<TextEntity>> { self.entities.clone() }
  #[doc(hidden)] pub fn _set_entities(&mut self, entities: Vec<TextEntity>) -> &mut Self { self.entities = Some(entities); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



