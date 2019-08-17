
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a sticker. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inlineQueryResultSticker
  /// Unique identifier of the query result.
  id: Option<String>,
  /// Sticker.
  sticker: Option<Sticker>,
  
}



impl Object for InlineQueryResultSticker {}
impl RObject for InlineQueryResultSticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultSticker" }
  fn td_type(&self) -> RTDType { RTDType::InlineQueryResultSticker }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InlineQueryResult for InlineQueryResultSticker {}


impl InlineQueryResultSticker {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inlineQueryResultSticker".to_string(),
      id: None,
      sticker: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn sticker(&self) -> Option<Sticker> { self.sticker.clone() }
  #[doc(hidden)] pub fn _set_sticker(&mut self, sticker: Sticker) -> &mut Self { self.sticker = Some(sticker); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



