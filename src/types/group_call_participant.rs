
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Represents a group call participant
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallParticipant {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user
  user_id: i64,
  /// User's synchronization source
  source: i64,
  /// True, if the participant is speaking as set by setGroupCallParticipantIsSpeaking
  is_speaking: bool,
  /// True, if the current user can mute the participant for all other group call participants
  can_be_muted_for_all_users: bool,
  /// True, if the current user can allow the participant to unmute themself or unmute the participant (if the participant is the current user)
  can_be_unmuted_for_all_users: bool,
  /// True, if the current user can mute the participant only for self
  can_be_muted_for_current_user: bool,
  /// True, if the current user can unmute the participant for self
  can_be_unmuted_for_current_user: bool,
  /// True, if the participant is muted for all users
  is_muted_for_all_users: bool,
  /// True, if the participant is muted for the current user
  is_muted_for_current_user: bool,
  /// True, if the participant is muted for all users, but can unmute themself
  can_unmute_self: bool,
  /// Participant's volume level; 1-20000 in hundreds of percents
  volume_level: i64,
  /// User's order in the group call participant list. The bigger is order, the higher is user in the list. If order is 0, the user must be removed from the participant list
  order: isize,
  
}

impl RObject for GroupCallParticipant {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "groupCallParticipant" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl GroupCallParticipant {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGroupCallParticipantBuilder {
    let mut inner = GroupCallParticipant::default();
    inner.td_name = "groupCallParticipant".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGroupCallParticipantBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn source(&self) -> i64 { self.source }

  pub fn is_speaking(&self) -> bool { self.is_speaking }

  pub fn can_be_muted_for_all_users(&self) -> bool { self.can_be_muted_for_all_users }

  pub fn can_be_unmuted_for_all_users(&self) -> bool { self.can_be_unmuted_for_all_users }

  pub fn can_be_muted_for_current_user(&self) -> bool { self.can_be_muted_for_current_user }

  pub fn can_be_unmuted_for_current_user(&self) -> bool { self.can_be_unmuted_for_current_user }

  pub fn is_muted_for_all_users(&self) -> bool { self.is_muted_for_all_users }

  pub fn is_muted_for_current_user(&self) -> bool { self.is_muted_for_current_user }

  pub fn can_unmute_self(&self) -> bool { self.can_unmute_self }

  pub fn volume_level(&self) -> i64 { self.volume_level }

  pub fn order(&self) -> isize { self.order }

}

#[doc(hidden)]
pub struct RTDGroupCallParticipantBuilder {
  inner: GroupCallParticipant
}

impl RTDGroupCallParticipantBuilder {
  pub fn build(&self) -> GroupCallParticipant { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn source(&mut self, source: i64) -> &mut Self {
    self.inner.source = source;
    self
  }

   
  pub fn is_speaking(&mut self, is_speaking: bool) -> &mut Self {
    self.inner.is_speaking = is_speaking;
    self
  }

   
  pub fn can_be_muted_for_all_users(&mut self, can_be_muted_for_all_users: bool) -> &mut Self {
    self.inner.can_be_muted_for_all_users = can_be_muted_for_all_users;
    self
  }

   
  pub fn can_be_unmuted_for_all_users(&mut self, can_be_unmuted_for_all_users: bool) -> &mut Self {
    self.inner.can_be_unmuted_for_all_users = can_be_unmuted_for_all_users;
    self
  }

   
  pub fn can_be_muted_for_current_user(&mut self, can_be_muted_for_current_user: bool) -> &mut Self {
    self.inner.can_be_muted_for_current_user = can_be_muted_for_current_user;
    self
  }

   
  pub fn can_be_unmuted_for_current_user(&mut self, can_be_unmuted_for_current_user: bool) -> &mut Self {
    self.inner.can_be_unmuted_for_current_user = can_be_unmuted_for_current_user;
    self
  }

   
  pub fn is_muted_for_all_users(&mut self, is_muted_for_all_users: bool) -> &mut Self {
    self.inner.is_muted_for_all_users = is_muted_for_all_users;
    self
  }

   
  pub fn is_muted_for_current_user(&mut self, is_muted_for_current_user: bool) -> &mut Self {
    self.inner.is_muted_for_current_user = is_muted_for_current_user;
    self
  }

   
  pub fn can_unmute_self(&mut self, can_unmute_self: bool) -> &mut Self {
    self.inner.can_unmute_self = can_unmute_self;
    self
  }

   
  pub fn volume_level(&mut self, volume_level: i64) -> &mut Self {
    self.inner.volume_level = volume_level;
    self
  }

   
  pub fn order(&mut self, order: isize) -> &mut Self {
    self.inner.order = order;
    self
  }

}

impl AsRef<GroupCallParticipant> for GroupCallParticipant {
  fn as_ref(&self) -> &GroupCallParticipant { self }
}

impl AsRef<GroupCallParticipant> for RTDGroupCallParticipantBuilder {
  fn as_ref(&self) -> &GroupCallParticipant { &self.inner }
}



