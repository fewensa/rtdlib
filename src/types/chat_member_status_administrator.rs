
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user is a member of a chat and has some additional privileges. In basic groups, administrators can edit and delete messages sent by others, add new members, and ban unprivileged members. In supergroups and channels, there are more detailed options for administrator privileges. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberStatusAdministrator {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatMemberStatusAdministrator
  /// True, if the current user can edit the administrator privileges for the called user.
  can_be_edited: Option<bool>,
  /// True, if the administrator can change the chat title, photo, and other settings.
  can_change_info: Option<bool>,
  /// True, if the administrator can create channel posts; applicable to channels only.
  can_post_messages: Option<bool>,
  /// True, if the administrator can edit messages of other users and pin messages; applicable to channels only.
  can_edit_messages: Option<bool>,
  /// True, if the administrator can delete messages of other users.
  can_delete_messages: Option<bool>,
  /// True, if the administrator can invite new users to the chat.
  can_invite_users: Option<bool>,
  /// True, if the administrator can restrict, ban, or unban chat members.
  can_restrict_members: Option<bool>,
  /// True, if the administrator can pin messages; applicable to groups only.
  can_pin_messages: Option<bool>,
  /// True, if the administrator can add new administrators with a subset of his own privileges or demote administrators that were directly or indirectly promoted by him.
  can_promote_members: Option<bool>,
  
}



impl Object for ChatMemberStatusAdministrator {}
impl RObject for ChatMemberStatusAdministrator {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMemberStatusAdministrator" }
  fn td_type(&self) -> RTDType { RTDType::ChatMemberStatusAdministrator }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatMemberStatus for ChatMemberStatusAdministrator {}


impl ChatMemberStatusAdministrator {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatMemberStatusAdministrator".to_string(),
      can_be_edited: None,
      can_change_info: None,
      can_post_messages: None,
      can_edit_messages: None,
      can_delete_messages: None,
      can_invite_users: None,
      can_restrict_members: None,
      can_pin_messages: None,
      can_promote_members: None,
      
    }
  }
  
  pub fn can_be_edited(&self) -> Option<bool> { self.can_be_edited.clone() }
  #[doc(hidden)] pub fn _set_can_be_edited(&mut self, can_be_edited: bool) -> &mut Self { self.can_be_edited = Some(can_be_edited); self }
  
  pub fn can_change_info(&self) -> Option<bool> { self.can_change_info.clone() }
  #[doc(hidden)] pub fn _set_can_change_info(&mut self, can_change_info: bool) -> &mut Self { self.can_change_info = Some(can_change_info); self }
  
  pub fn can_post_messages(&self) -> Option<bool> { self.can_post_messages.clone() }
  #[doc(hidden)] pub fn _set_can_post_messages(&mut self, can_post_messages: bool) -> &mut Self { self.can_post_messages = Some(can_post_messages); self }
  
  pub fn can_edit_messages(&self) -> Option<bool> { self.can_edit_messages.clone() }
  #[doc(hidden)] pub fn _set_can_edit_messages(&mut self, can_edit_messages: bool) -> &mut Self { self.can_edit_messages = Some(can_edit_messages); self }
  
  pub fn can_delete_messages(&self) -> Option<bool> { self.can_delete_messages.clone() }
  #[doc(hidden)] pub fn _set_can_delete_messages(&mut self, can_delete_messages: bool) -> &mut Self { self.can_delete_messages = Some(can_delete_messages); self }
  
  pub fn can_invite_users(&self) -> Option<bool> { self.can_invite_users.clone() }
  #[doc(hidden)] pub fn _set_can_invite_users(&mut self, can_invite_users: bool) -> &mut Self { self.can_invite_users = Some(can_invite_users); self }
  
  pub fn can_restrict_members(&self) -> Option<bool> { self.can_restrict_members.clone() }
  #[doc(hidden)] pub fn _set_can_restrict_members(&mut self, can_restrict_members: bool) -> &mut Self { self.can_restrict_members = Some(can_restrict_members); self }
  
  pub fn can_pin_messages(&self) -> Option<bool> { self.can_pin_messages.clone() }
  #[doc(hidden)] pub fn _set_can_pin_messages(&mut self, can_pin_messages: bool) -> &mut Self { self.can_pin_messages = Some(can_pin_messages); self }
  
  pub fn can_promote_members(&self) -> Option<bool> { self.can_promote_members.clone() }
  #[doc(hidden)] pub fn _set_can_promote_members(&mut self, can_promote_members: bool) -> &mut Self { self.can_promote_members = Some(can_promote_members); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



