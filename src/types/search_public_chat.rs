
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Searches a public chat by its username. Currently only private chats, supergroups and channels can be public. Returns the chat if found; otherwise an error is returned.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchPublicChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchPublicChat
  /// Username to be resolved.
  username: Option<String>,
  
}



impl Object for SearchPublicChat {}
impl RObject for SearchPublicChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchPublicChat" }
  fn td_type(&self) -> RTDType { RTDType::SearchPublicChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SearchPublicChat {}


impl SearchPublicChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchPublicChat".to_string(),
      username: None,
      
    }
  }
  
  pub fn username(&self) -> Option<String> { self.username.clone() }
  #[doc(hidden)] pub fn _set_username(&mut self, username: String) -> &mut Self { self.username = Some(username); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



