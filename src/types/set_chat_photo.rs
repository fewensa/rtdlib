
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes the photo of a chat. Supported only for basic groups, supergroups and channels. Requires administrator rights in basic groups and the appropriate administrator rights in supergroups and channels. The photo will not be changed before request to the server has been completed.
#[derive(Debug, Serialize, Deserialize)]
pub struct SetChatPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setChatPhoto
  /// Chat identifier.
  chat_id: Option<i64>,
  /// New chat photo. You can use a zero InputFileId to delete the chat photo. Files that are accessible only by HTTP URL are not acceptable.
  photo: Option<Box<InputFile>>,
  
}


impl Clone for SetChatPhoto {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for SetChatPhoto {}
impl RObject for SetChatPhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setChatPhoto" }
  fn td_type(&self) -> RTDType { RTDType::SetChatPhoto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetChatPhoto {}


impl SetChatPhoto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setChatPhoto".to_string(),
      chat_id: None,
      photo: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn photo(&self) -> Option<Box<InputFile>> { self.photo.clone() }
  #[doc(hidden)] pub fn _set_photo(&mut self, photo: Box<InputFile>) -> &mut Self { self.photo = Some(photo); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



