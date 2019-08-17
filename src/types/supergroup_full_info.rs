
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains full information about a supergroup or channel. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupergroupFullInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // supergroupFullInfo
  /// Supergroup or channel description.
  description: Option<String>,
  /// Number of members in the supergroup or channel; 0 if unknown.
  member_count: Option<i32>,
  /// Number of privileged users in the supergroup or channel; 0 if unknown.
  administrator_count: Option<i32>,
  /// Number of restricted users in the supergroup; 0 if unknown.
  restricted_count: Option<i32>,
  /// Number of users banned from chat; 0 if unknown.
  banned_count: Option<i32>,
  /// True, if members of the chat can be retrieved.
  can_get_members: Option<bool>,
  /// True, if the chat can be made public.
  can_set_username: Option<bool>,
  /// True, if the supergroup sticker set can be changed.
  can_set_sticker_set: Option<bool>,
  /// True, if the channel statistics is available through getChatStatisticsUrl.
  can_view_statistics: Option<bool>,
  /// True, if new chat members will have access to old messages. In public supergroups and both public and private channels, old messages are always available, so this option affects only private supergroups. The value of this field is only available for chat administrators.
  is_all_history_available: Option<bool>,
  /// Identifier of the supergroup sticker set; 0 if none.
  sticker_set_id: Option<String>,
  /// Invite link for this chat.
  invite_link: Option<String>,
  /// Identifier of the basic group from which supergroup was upgraded; 0 if none.
  upgraded_from_basic_group_id: Option<i32>,
  /// Identifier of the last message in the basic group from which supergroup was upgraded; 0 if none.
  upgraded_from_max_message_id: Option<i64>,
  
}



impl Object for SupergroupFullInfo {}
impl RObject for SupergroupFullInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "supergroupFullInfo" }
  fn td_type(&self) -> RTDType { RTDType::SupergroupFullInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl SupergroupFullInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "supergroupFullInfo".to_string(),
      description: None,
      member_count: None,
      administrator_count: None,
      restricted_count: None,
      banned_count: None,
      can_get_members: None,
      can_set_username: None,
      can_set_sticker_set: None,
      can_view_statistics: None,
      is_all_history_available: None,
      sticker_set_id: None,
      invite_link: None,
      upgraded_from_basic_group_id: None,
      upgraded_from_max_message_id: None,
      
    }
  }
  
  pub fn description(&self) -> Option<String> { self.description.clone() }
  #[doc(hidden)] pub fn _set_description(&mut self, description: String) -> &mut Self { self.description = Some(description); self }
  
  pub fn member_count(&self) -> Option<i32> { self.member_count.clone() }
  #[doc(hidden)] pub fn _set_member_count(&mut self, member_count: i32) -> &mut Self { self.member_count = Some(member_count); self }
  
  pub fn administrator_count(&self) -> Option<i32> { self.administrator_count.clone() }
  #[doc(hidden)] pub fn _set_administrator_count(&mut self, administrator_count: i32) -> &mut Self { self.administrator_count = Some(administrator_count); self }
  
  pub fn restricted_count(&self) -> Option<i32> { self.restricted_count.clone() }
  #[doc(hidden)] pub fn _set_restricted_count(&mut self, restricted_count: i32) -> &mut Self { self.restricted_count = Some(restricted_count); self }
  
  pub fn banned_count(&self) -> Option<i32> { self.banned_count.clone() }
  #[doc(hidden)] pub fn _set_banned_count(&mut self, banned_count: i32) -> &mut Self { self.banned_count = Some(banned_count); self }
  
  pub fn can_get_members(&self) -> Option<bool> { self.can_get_members.clone() }
  #[doc(hidden)] pub fn _set_can_get_members(&mut self, can_get_members: bool) -> &mut Self { self.can_get_members = Some(can_get_members); self }
  
  pub fn can_set_username(&self) -> Option<bool> { self.can_set_username.clone() }
  #[doc(hidden)] pub fn _set_can_set_username(&mut self, can_set_username: bool) -> &mut Self { self.can_set_username = Some(can_set_username); self }
  
  pub fn can_set_sticker_set(&self) -> Option<bool> { self.can_set_sticker_set.clone() }
  #[doc(hidden)] pub fn _set_can_set_sticker_set(&mut self, can_set_sticker_set: bool) -> &mut Self { self.can_set_sticker_set = Some(can_set_sticker_set); self }
  
  pub fn can_view_statistics(&self) -> Option<bool> { self.can_view_statistics.clone() }
  #[doc(hidden)] pub fn _set_can_view_statistics(&mut self, can_view_statistics: bool) -> &mut Self { self.can_view_statistics = Some(can_view_statistics); self }
  
  pub fn is_all_history_available(&self) -> Option<bool> { self.is_all_history_available.clone() }
  #[doc(hidden)] pub fn _set_is_all_history_available(&mut self, is_all_history_available: bool) -> &mut Self { self.is_all_history_available = Some(is_all_history_available); self }
  
  pub fn sticker_set_id(&self) -> Option<String> { self.sticker_set_id.clone() }
  #[doc(hidden)] pub fn _set_sticker_set_id(&mut self, sticker_set_id: String) -> &mut Self { self.sticker_set_id = Some(sticker_set_id); self }
  
  pub fn invite_link(&self) -> Option<String> { self.invite_link.clone() }
  #[doc(hidden)] pub fn _set_invite_link(&mut self, invite_link: String) -> &mut Self { self.invite_link = Some(invite_link); self }
  
  pub fn upgraded_from_basic_group_id(&self) -> Option<i32> { self.upgraded_from_basic_group_id.clone() }
  #[doc(hidden)] pub fn _set_upgraded_from_basic_group_id(&mut self, upgraded_from_basic_group_id: i32) -> &mut Self { self.upgraded_from_basic_group_id = Some(upgraded_from_basic_group_id); self }
  
  pub fn upgraded_from_max_message_id(&self) -> Option<i64> { self.upgraded_from_max_message_id.clone() }
  #[doc(hidden)] pub fn _set_upgraded_from_max_message_id(&mut self, upgraded_from_max_message_id: i64) -> &mut Self { self.upgraded_from_max_message_id = Some(upgraded_from_max_message_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



