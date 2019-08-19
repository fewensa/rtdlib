
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about a chat invite link. 
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatInviteLinkInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatInviteLinkInfo
  /// Chat identifier of the invite link; 0 if the user is not a member of this chat.
  chat_id: Option<i64>,
  /// Contains information about the type of the chat.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<Box<ChatType>>,
  /// Title of the chat.
  title: Option<String>,
  /// Chat photo; may be null.
  photo: Option<ChatPhoto>,
  /// Number of members.
  member_count: Option<i32>,
  /// User identifiers of some chat members that may be known to the current user.
  member_user_ids: Option<Vec<i32>>,
  /// True, if the chat is a public supergroup or channel with a username.
  is_public: Option<bool>,
  
}


impl Clone for ChatInviteLinkInfo {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for ChatInviteLinkInfo {}
impl RObject for ChatInviteLinkInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatInviteLinkInfo" }
  fn td_type(&self) -> RTDType { RTDType::ChatInviteLinkInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl ChatInviteLinkInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatInviteLinkInfo".to_string(),
      chat_id: None,
      type_: None,
      title: None,
      photo: None,
      member_count: None,
      member_user_ids: None,
      is_public: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn type_(&self) -> Option<Box<ChatType>> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: Box<ChatType>) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn photo(&self) -> Option<ChatPhoto> { self.photo.clone() }
  #[doc(hidden)] pub fn _set_photo(&mut self, photo: ChatPhoto) -> &mut Self { self.photo = Some(photo); self }
  
  pub fn member_count(&self) -> Option<i32> { self.member_count.clone() }
  #[doc(hidden)] pub fn _set_member_count(&mut self, member_count: i32) -> &mut Self { self.member_count = Some(member_count); self }
  
  pub fn member_user_ids(&self) -> Option<Vec<i32>> { self.member_user_ids.clone() }
  #[doc(hidden)] pub fn _set_member_user_ids(&mut self, member_user_ids: Vec<i32>) -> &mut Self { self.member_user_ids = Some(member_user_ids); self }
  
  pub fn is_public(&self) -> Option<bool> { self.is_public.clone() }
  #[doc(hidden)] pub fn _set_is_public(&mut self, is_public: bool) -> &mut Self { self.is_public = Some(is_public); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



