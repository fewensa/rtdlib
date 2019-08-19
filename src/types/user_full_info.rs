
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains full information about a user (except the full list of profile photos). 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFullInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // userFullInfo
  /// True, if the user is blacklisted by the current user.
  is_blocked: Option<bool>,
  /// True, if the user can be called.
  can_be_called: Option<bool>,
  /// True, if the user can't be called due to their privacy settings.
  has_private_calls: Option<bool>,
  /// A short user bio.
  bio: Option<String>,
  /// For bots, the text that is included with the link when users share the bot.
  share_text: Option<String>,
  /// Number of group chats where both the other user and the current user are a member; 0 for the current user.
  group_in_common_count: Option<i32>,
  /// If the user is a bot, information about the bot; may be null.
  bot_info: Option<BotInfo>,
  
}



impl Object for UserFullInfo {}
impl RObject for UserFullInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userFullInfo" }
  fn td_type(&self) -> RTDType { RTDType::UserFullInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl UserFullInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "userFullInfo".to_string(),
      is_blocked: None,
      can_be_called: None,
      has_private_calls: None,
      bio: None,
      share_text: None,
      group_in_common_count: None,
      bot_info: None,
      
    }
  }
  
  pub fn is_blocked(&self) -> Option<bool> { self.is_blocked.clone() }
  #[doc(hidden)] pub fn _set_is_blocked(&mut self, is_blocked: bool) -> &mut Self { self.is_blocked = Some(is_blocked); self }
  
  pub fn can_be_called(&self) -> Option<bool> { self.can_be_called.clone() }
  #[doc(hidden)] pub fn _set_can_be_called(&mut self, can_be_called: bool) -> &mut Self { self.can_be_called = Some(can_be_called); self }
  
  pub fn has_private_calls(&self) -> Option<bool> { self.has_private_calls.clone() }
  #[doc(hidden)] pub fn _set_has_private_calls(&mut self, has_private_calls: bool) -> &mut Self { self.has_private_calls = Some(has_private_calls); self }
  
  pub fn bio(&self) -> Option<String> { self.bio.clone() }
  #[doc(hidden)] pub fn _set_bio(&mut self, bio: String) -> &mut Self { self.bio = Some(bio); self }
  
  pub fn share_text(&self) -> Option<String> { self.share_text.clone() }
  #[doc(hidden)] pub fn _set_share_text(&mut self, share_text: String) -> &mut Self { self.share_text = Some(share_text); self }
  
  pub fn group_in_common_count(&self) -> Option<i32> { self.group_in_common_count.clone() }
  #[doc(hidden)] pub fn _set_group_in_common_count(&mut self, group_in_common_count: i32) -> &mut Self { self.group_in_common_count = Some(group_in_common_count); self }
  
  pub fn bot_info(&self) -> Option<BotInfo> { self.bot_info.clone() }
  #[doc(hidden)] pub fn _set_bot_info(&mut self, bot_info: BotInfo) -> &mut Self { self.bot_info = Some(bot_info); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



