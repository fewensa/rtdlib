
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An updated chat photo. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageChatChangePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageChatChangePhoto
  /// New chat photo.
  photo: Option<Photo>,
  
}



impl Object for MessageChatChangePhoto {}
impl RObject for MessageChatChangePhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageChatChangePhoto" }
  fn td_type(&self) -> RTDType { RTDType::MessageChatChangePhoto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageChatChangePhoto {}


impl MessageChatChangePhoto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageChatChangePhoto".to_string(),
      photo: None,
      
    }
  }
  
  pub fn photo(&self) -> Option<Photo> { self.photo.clone() }
  #[doc(hidden)] pub fn _set_photo(&mut self, photo: Photo) -> &mut Self { self.photo = Some(photo); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



