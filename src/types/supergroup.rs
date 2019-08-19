
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a supergroup or channel with zero or more members (subscribers in the case of channels). From the point of view of the system, a channel is a special kind of a supergroup: only administrators can post and see the list of members, and posts from all administrators use the name and photo of the channel instead of individual names and profile photos. Unlike supergroups, channels can have an unlimited number of subscribers. 
#[derive(Debug, Serialize, Deserialize)]
pub struct Supergroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // supergroup
  /// Supergroup or channel identifier.
  id: Option<i32>,
  /// Username of the supergroup or channel; empty for private supergroups or channels.
  username: Option<String>,
  /// Point in time (Unix timestamp) when the current user joined, or the point in time when the supergroup or channel was created, in case the user is not a member.
  date: Option<i32>,
  /// Status of the current user in the supergroup or channel.
  status: Option<Box<ChatMemberStatus>>,
  /// Member count; 0 if unknown. Currently it is guaranteed to be known only if the supergroup or channel was found through SearchPublicChats.
  member_count: Option<i32>,
  /// True, if any member of the supergroup can invite other members. This field has no meaning for channels.
  anyone_can_invite: Option<bool>,
  /// True, if messages sent to the channel should contain information about the sender. This field is only applicable to channels.
  sign_messages: Option<bool>,
  /// True, if the supergroup is a channel.
  is_channel: Option<bool>,
  /// True, if the supergroup or channel is verified.
  is_verified: Option<bool>,
  /// If non-empty, contains the reason why access to this supergroup or channel must be restricted. Format of the string is "{type}: {description}". {type} Contains the type of the restriction and at least one of the suffixes "-all", "-ios", "-android", or "-wp", which describe the platforms on which access should be restricted. (For example, "terms-ios-android". {description} contains a human-readable description of the restriction, which can be shown to the user.)
  restriction_reason: Option<String>,
  
}


impl Clone for Supergroup {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for Supergroup {}
impl RObject for Supergroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "supergroup" }
  fn td_type(&self) -> RTDType { RTDType::Supergroup }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Supergroup {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "supergroup".to_string(),
      id: None,
      username: None,
      date: None,
      status: None,
      member_count: None,
      anyone_can_invite: None,
      sign_messages: None,
      is_channel: None,
      is_verified: None,
      restriction_reason: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i32> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i32) -> &mut Self { self.id = Some(id); self }
  
  pub fn username(&self) -> Option<String> { self.username.clone() }
  #[doc(hidden)] pub fn _set_username(&mut self, username: String) -> &mut Self { self.username = Some(username); self }
  
  pub fn date(&self) -> Option<i32> { self.date.clone() }
  #[doc(hidden)] pub fn _set_date(&mut self, date: i32) -> &mut Self { self.date = Some(date); self }
  
  pub fn status(&self) -> Option<Box<ChatMemberStatus>> { self.status.clone() }
  #[doc(hidden)] pub fn _set_status(&mut self, status: Box<ChatMemberStatus>) -> &mut Self { self.status = Some(status); self }
  
  pub fn member_count(&self) -> Option<i32> { self.member_count.clone() }
  #[doc(hidden)] pub fn _set_member_count(&mut self, member_count: i32) -> &mut Self { self.member_count = Some(member_count); self }
  
  pub fn anyone_can_invite(&self) -> Option<bool> { self.anyone_can_invite.clone() }
  #[doc(hidden)] pub fn _set_anyone_can_invite(&mut self, anyone_can_invite: bool) -> &mut Self { self.anyone_can_invite = Some(anyone_can_invite); self }
  
  pub fn sign_messages(&self) -> Option<bool> { self.sign_messages.clone() }
  #[doc(hidden)] pub fn _set_sign_messages(&mut self, sign_messages: bool) -> &mut Self { self.sign_messages = Some(sign_messages); self }
  
  pub fn is_channel(&self) -> Option<bool> { self.is_channel.clone() }
  #[doc(hidden)] pub fn _set_is_channel(&mut self, is_channel: bool) -> &mut Self { self.is_channel = Some(is_channel); self }
  
  pub fn is_verified(&self) -> Option<bool> { self.is_verified.clone() }
  #[doc(hidden)] pub fn _set_is_verified(&mut self, is_verified: bool) -> &mut Self { self.is_verified = Some(is_verified); self }
  
  pub fn restriction_reason(&self) -> Option<String> { self.restriction_reason.clone() }
  #[doc(hidden)] pub fn _set_restriction_reason(&mut self, restriction_reason: String) -> &mut Self { self.restriction_reason = Some(restriction_reason); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



