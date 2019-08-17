
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A chat photo was changed. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChatPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateChatPhoto
  /// Chat identifier.
  chat_id: Option<i64>,
  /// The new chat photo; may be null.
  photo: Option<ChatPhoto>,
  
}



impl Object for UpdateChatPhoto {}
impl RObject for UpdateChatPhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatPhoto" }
  fn td_type(&self) -> RTDType { RTDType::UpdateChatPhoto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateChatPhoto {}


impl UpdateChatPhoto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateChatPhoto".to_string(),
      chat_id: None,
      photo: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn photo(&self) -> Option<ChatPhoto> { self.photo.clone() }
  #[doc(hidden)] pub fn _set_photo(&mut self, photo: ChatPhoto) -> &mut Self { self.photo = Some(photo); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



