
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a recently speaking participant in a group call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallRecentSpeaker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Group call participant identifier
  participant_id: MessageSender,
  /// True, is the user has spoken recently
  is_speaking: bool,
  
}

impl RObject for GroupCallRecentSpeaker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "groupCallRecentSpeaker" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl GroupCallRecentSpeaker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGroupCallRecentSpeakerBuilder {
    let mut inner = GroupCallRecentSpeaker::default();
    inner.td_name = "groupCallRecentSpeaker".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGroupCallRecentSpeakerBuilder { inner }
  }

  pub fn participant_id(&self) -> &MessageSender { &self.participant_id }

  pub fn is_speaking(&self) -> bool { self.is_speaking }

}

#[doc(hidden)]
pub struct RTDGroupCallRecentSpeakerBuilder {
  inner: GroupCallRecentSpeaker
}

impl RTDGroupCallRecentSpeakerBuilder {
  pub fn build(&self) -> GroupCallRecentSpeaker { self.inner.clone() }

   
  pub fn participant_id<T: AsRef<MessageSender>>(&mut self, participant_id: T) -> &mut Self {
    self.inner.participant_id = participant_id.as_ref().clone();
    self
  }

   
  pub fn is_speaking(&mut self, is_speaking: bool) -> &mut Self {
    self.inner.is_speaking = is_speaking;
    self
  }

}

impl AsRef<GroupCallRecentSpeaker> for GroupCallRecentSpeaker {
  fn as_ref(&self) -> &GroupCallRecentSpeaker { self }
}

impl AsRef<GroupCallRecentSpeaker> for RTDGroupCallRecentSpeakerBuilder {
  fn as_ref(&self) -> &GroupCallRecentSpeaker { &self.inner }
}



