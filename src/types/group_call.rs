
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a group call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Group call identifier
  id: i64,
  /// True, if the call is active
  is_active: bool,
  /// True, if the call is joined
  is_joined: bool,
  /// True, if user was kicked from the call because of network loss and the call needs to be rejoined
  need_rejoin: bool,
  /// True, if the current user can unmute themself
  can_unmute_self: bool,
  /// True, if the current user can manage the group call
  can_be_managed: bool,
  /// Number of participants in the group call
  participant_count: i64,
  /// True, if all group call participants are loaded
  loaded_all_participants: bool,
  /// Recently speaking users in the group call
  recent_speakers: Vec<GroupCallRecentSpeaker>,
  /// True, if only group call administrators can unmute new participants
  mute_new_participants: bool,
  /// True, if group call administrators can enable or disable mute_new_participants setting
  allowed_change_mute_new_participants: bool,
  /// Call duration; for ended calls only
  duration: i64,
  
}

impl RObject for GroupCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "groupCall" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl GroupCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGroupCallBuilder {
    let mut inner = GroupCall::default();
    inner.td_name = "groupCall".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGroupCallBuilder { inner }
  }

  pub fn id(&self) -> i64 { self.id }

  pub fn is_active(&self) -> bool { self.is_active }

  pub fn is_joined(&self) -> bool { self.is_joined }

  pub fn need_rejoin(&self) -> bool { self.need_rejoin }

  pub fn can_unmute_self(&self) -> bool { self.can_unmute_self }

  pub fn can_be_managed(&self) -> bool { self.can_be_managed }

  pub fn participant_count(&self) -> i64 { self.participant_count }

  pub fn loaded_all_participants(&self) -> bool { self.loaded_all_participants }

  pub fn recent_speakers(&self) -> &Vec<GroupCallRecentSpeaker> { &self.recent_speakers }

  pub fn mute_new_participants(&self) -> bool { self.mute_new_participants }

  pub fn allowed_change_mute_new_participants(&self) -> bool { self.allowed_change_mute_new_participants }

  pub fn duration(&self) -> i64 { self.duration }

}

#[doc(hidden)]
pub struct RTDGroupCallBuilder {
  inner: GroupCall
}

impl RTDGroupCallBuilder {
  pub fn build(&self) -> GroupCall { self.inner.clone() }

   
  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn is_active(&mut self, is_active: bool) -> &mut Self {
    self.inner.is_active = is_active;
    self
  }

   
  pub fn is_joined(&mut self, is_joined: bool) -> &mut Self {
    self.inner.is_joined = is_joined;
    self
  }

   
  pub fn need_rejoin(&mut self, need_rejoin: bool) -> &mut Self {
    self.inner.need_rejoin = need_rejoin;
    self
  }

   
  pub fn can_unmute_self(&mut self, can_unmute_self: bool) -> &mut Self {
    self.inner.can_unmute_self = can_unmute_self;
    self
  }

   
  pub fn can_be_managed(&mut self, can_be_managed: bool) -> &mut Self {
    self.inner.can_be_managed = can_be_managed;
    self
  }

   
  pub fn participant_count(&mut self, participant_count: i64) -> &mut Self {
    self.inner.participant_count = participant_count;
    self
  }

   
  pub fn loaded_all_participants(&mut self, loaded_all_participants: bool) -> &mut Self {
    self.inner.loaded_all_participants = loaded_all_participants;
    self
  }

   
  pub fn recent_speakers(&mut self, recent_speakers: Vec<GroupCallRecentSpeaker>) -> &mut Self {
    self.inner.recent_speakers = recent_speakers;
    self
  }

   
  pub fn mute_new_participants(&mut self, mute_new_participants: bool) -> &mut Self {
    self.inner.mute_new_participants = mute_new_participants;
    self
  }

   
  pub fn allowed_change_mute_new_participants(&mut self, allowed_change_mute_new_participants: bool) -> &mut Self {
    self.inner.allowed_change_mute_new_participants = allowed_change_mute_new_participants;
    self
  }

   
  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

}

impl AsRef<GroupCall> for GroupCall {
  fn as_ref(&self) -> &GroupCall { self }
}

impl AsRef<GroupCall> for RTDGroupCallBuilder {
  fn as_ref(&self) -> &GroupCall { &self.inner }
}



