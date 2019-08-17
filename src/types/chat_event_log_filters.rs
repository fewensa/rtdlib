
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a set of filters used to obtain a chat event log. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEventLogFilters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatEventLogFilters
  /// True, if message edits should be returned.
  message_edits: Option<bool>,
  /// True, if message deletions should be returned.
  message_deletions: Option<bool>,
  /// True, if pin/unpin events should be returned.
  message_pins: Option<bool>,
  /// True, if members joining events should be returned.
  member_joins: Option<bool>,
  /// True, if members leaving events should be returned.
  member_leaves: Option<bool>,
  /// True, if invited member events should be returned.
  member_invites: Option<bool>,
  /// True, if member promotion/demotion events should be returned.
  member_promotions: Option<bool>,
  /// True, if member restricted/unrestricted/banned/unbanned events should be returned.
  member_restrictions: Option<bool>,
  /// True, if changes in chat information should be returned.
  info_changes: Option<bool>,
  /// True, if changes in chat settings should be returned.
  setting_changes: Option<bool>,
  
}



impl Object for ChatEventLogFilters {}
impl RObject for ChatEventLogFilters {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventLogFilters" }
  fn td_type(&self) -> RTDType { RTDType::ChatEventLogFilters }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl ChatEventLogFilters {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatEventLogFilters".to_string(),
      message_edits: None,
      message_deletions: None,
      message_pins: None,
      member_joins: None,
      member_leaves: None,
      member_invites: None,
      member_promotions: None,
      member_restrictions: None,
      info_changes: None,
      setting_changes: None,
      
    }
  }
  
  pub fn message_edits(&self) -> Option<bool> { self.message_edits.clone() }
  #[doc(hidden)] pub fn _set_message_edits(&mut self, message_edits: bool) -> &mut Self { self.message_edits = Some(message_edits); self }
  
  pub fn message_deletions(&self) -> Option<bool> { self.message_deletions.clone() }
  #[doc(hidden)] pub fn _set_message_deletions(&mut self, message_deletions: bool) -> &mut Self { self.message_deletions = Some(message_deletions); self }
  
  pub fn message_pins(&self) -> Option<bool> { self.message_pins.clone() }
  #[doc(hidden)] pub fn _set_message_pins(&mut self, message_pins: bool) -> &mut Self { self.message_pins = Some(message_pins); self }
  
  pub fn member_joins(&self) -> Option<bool> { self.member_joins.clone() }
  #[doc(hidden)] pub fn _set_member_joins(&mut self, member_joins: bool) -> &mut Self { self.member_joins = Some(member_joins); self }
  
  pub fn member_leaves(&self) -> Option<bool> { self.member_leaves.clone() }
  #[doc(hidden)] pub fn _set_member_leaves(&mut self, member_leaves: bool) -> &mut Self { self.member_leaves = Some(member_leaves); self }
  
  pub fn member_invites(&self) -> Option<bool> { self.member_invites.clone() }
  #[doc(hidden)] pub fn _set_member_invites(&mut self, member_invites: bool) -> &mut Self { self.member_invites = Some(member_invites); self }
  
  pub fn member_promotions(&self) -> Option<bool> { self.member_promotions.clone() }
  #[doc(hidden)] pub fn _set_member_promotions(&mut self, member_promotions: bool) -> &mut Self { self.member_promotions = Some(member_promotions); self }
  
  pub fn member_restrictions(&self) -> Option<bool> { self.member_restrictions.clone() }
  #[doc(hidden)] pub fn _set_member_restrictions(&mut self, member_restrictions: bool) -> &mut Self { self.member_restrictions = Some(member_restrictions); self }
  
  pub fn info_changes(&self) -> Option<bool> { self.info_changes.clone() }
  #[doc(hidden)] pub fn _set_info_changes(&mut self, info_changes: bool) -> &mut Self { self.info_changes = Some(info_changes); self }
  
  pub fn setting_changes(&self) -> Option<bool> { self.setting_changes.clone() }
  #[doc(hidden)] pub fn _set_setting_changes(&mut self, setting_changes: bool) -> &mut Self { self.setting_changes = Some(setting_changes); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



