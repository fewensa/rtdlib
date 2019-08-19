
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A user with information about joining/leaving a chat. 
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMember {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatMember
  /// User identifier of the chat member.
  user_id: Option<i32>,
  /// Identifier of a user that invited/promoted/banned this member in the chat; 0 if unknown.
  inviter_user_id: Option<i32>,
  /// Point in time (Unix timestamp) when the user joined a chat.
  joined_chat_date: Option<i32>,
  /// Status of the member in the chat.
  status: Option<Box<ChatMemberStatus>>,
  /// If the user is a bot, information about the bot; may be null. Can be null even for a bot if the bot is not a chat member.
  bot_info: Option<BotInfo>,
  
}


impl Clone for ChatMember {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for ChatMember {}
impl RObject for ChatMember {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMember" }
  fn td_type(&self) -> RTDType { RTDType::ChatMember }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl ChatMember {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatMember".to_string(),
      user_id: None,
      inviter_user_id: None,
      joined_chat_date: None,
      status: None,
      bot_info: None,
      
    }
  }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn inviter_user_id(&self) -> Option<i32> { self.inviter_user_id.clone() }
  #[doc(hidden)] pub fn _set_inviter_user_id(&mut self, inviter_user_id: i32) -> &mut Self { self.inviter_user_id = Some(inviter_user_id); self }
  
  pub fn joined_chat_date(&self) -> Option<i32> { self.joined_chat_date.clone() }
  #[doc(hidden)] pub fn _set_joined_chat_date(&mut self, joined_chat_date: i32) -> &mut Self { self.joined_chat_date = Some(joined_chat_date); self }
  
  pub fn status(&self) -> Option<Box<ChatMemberStatus>> { self.status.clone() }
  #[doc(hidden)] pub fn _set_status(&mut self, status: Box<ChatMemberStatus>) -> &mut Self { self.status = Some(status); self }
  
  pub fn bot_info(&self) -> Option<BotInfo> { self.bot_info.clone() }
  #[doc(hidden)] pub fn _set_bot_info(&mut self, bot_info: BotInfo) -> &mut Self { self.bot_info = Some(bot_info); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



