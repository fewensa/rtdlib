
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns all entities (mentions, hashtags, cashtags, bot commands, URLs, and email addresses) contained in the text. This is an offline method. Can be called before authorization. Can be called synchronously.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTextEntities {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getTextEntities
  /// The text in which to look for entites.
  text: Option<String>,
  
}



impl Object for GetTextEntities {}
impl RObject for GetTextEntities {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getTextEntities" }
  fn td_type(&self) -> RTDType { RTDType::GetTextEntities }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetTextEntities {}


impl GetTextEntities {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getTextEntities".to_string(),
      text: None,
      
    }
  }
  
  pub fn text(&self) -> Option<String> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: String) -> &mut Self { self.text = Some(text); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



