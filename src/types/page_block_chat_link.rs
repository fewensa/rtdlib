
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A link to a chat. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBlockChatLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockChatLink
  /// Chat title.
  title: Option<String>,
  /// Chat photo; may be null.
  photo: Option<ChatPhoto>,
  /// Chat username, by which all other information about the chat should be resolved.
  username: Option<String>,
  
}



impl Object for PageBlockChatLink {}
impl RObject for PageBlockChatLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockChatLink" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockChatLink }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockChatLink {}


impl PageBlockChatLink {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockChatLink".to_string(),
      title: None,
      photo: None,
      username: None,
      
    }
  }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn photo(&self) -> Option<ChatPhoto> { self.photo.clone() }
  #[doc(hidden)] pub fn _set_photo(&mut self, photo: ChatPhoto) -> &mut Self { self.photo = Some(photo); self }
  
  pub fn username(&self) -> Option<String> { self.username.clone() }
  #[doc(hidden)] pub fn _set_username(&mut self, username: String) -> &mut Self { self.username = Some(username); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



