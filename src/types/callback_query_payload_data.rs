
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The payload from a general callback button. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackQueryPayloadData {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // callbackQueryPayloadData
  /// Data that was attached to the callback button.
  data: Option<String>,
  
}



impl Object for CallbackQueryPayloadData {}
impl RObject for CallbackQueryPayloadData {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callbackQueryPayloadData" }
  fn td_type(&self) -> RTDType { RTDType::CallbackQueryPayloadData }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl CallbackQueryPayload for CallbackQueryPayloadData {}


impl CallbackQueryPayloadData {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "callbackQueryPayloadData".to_string(),
      data: None,
      
    }
  }
  
  pub fn data(&self) -> Option<String> { self.data.clone() }
  #[doc(hidden)] pub fn _set_data(&mut self, data: String) -> &mut Self { self.data = Some(data); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



