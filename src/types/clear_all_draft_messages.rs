
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Clears draft messages in all chats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearAllDraftMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // clearAllDraftMessages
  /// If true, local draft messages in secret chats will not be cleared.
  exclude_secret_chats: Option<bool>,
  
}



impl Object for ClearAllDraftMessages {}
impl RObject for ClearAllDraftMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "clearAllDraftMessages" }
  fn td_type(&self) -> RTDType { RTDType::ClearAllDraftMessages }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ClearAllDraftMessages {}


impl ClearAllDraftMessages {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "clearAllDraftMessages".to_string(),
      exclude_secret_chats: None,
      
    }
  }
  
  pub fn exclude_secret_chats(&self) -> Option<bool> { self.exclude_secret_chats.clone() }
  #[doc(hidden)] pub fn _set_exclude_secret_chats(&mut self, exclude_secret_chats: bool) -> &mut Self { self.exclude_secret_chats = Some(exclude_secret_chats); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



