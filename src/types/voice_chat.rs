
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a voice chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VoiceChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Group call identifier of an active voice chat; 0 if none. Full informationa about the voice chat can be received through the method getGroupCall
  group_call_id: i64,
  /// True, if the voice chat has participants
  has_participants: bool,
  /// Default group call participant identifier to join the voice chat; may be null
  default_participant_id: Option<MessageSender>,
  
}

impl RObject for VoiceChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "voiceChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl VoiceChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDVoiceChatBuilder {
    let mut inner = VoiceChat::default();
    inner.td_name = "voiceChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDVoiceChatBuilder { inner }
  }

  pub fn group_call_id(&self) -> i64 { self.group_call_id }

  pub fn has_participants(&self) -> bool { self.has_participants }

  pub fn default_participant_id(&self) -> &Option<MessageSender> { &self.default_participant_id }

}

#[doc(hidden)]
pub struct RTDVoiceChatBuilder {
  inner: VoiceChat
}

impl RTDVoiceChatBuilder {
  pub fn build(&self) -> VoiceChat { self.inner.clone() }

   
  pub fn group_call_id(&mut self, group_call_id: i64) -> &mut Self {
    self.inner.group_call_id = group_call_id;
    self
  }

   
  pub fn has_participants(&mut self, has_participants: bool) -> &mut Self {
    self.inner.has_participants = has_participants;
    self
  }

   
  pub fn default_participant_id<T: AsRef<MessageSender>>(&mut self, default_participant_id: T) -> &mut Self {
    self.inner.default_participant_id = Some(default_participant_id.as_ref().clone());
    self
  }

}

impl AsRef<VoiceChat> for VoiceChat {
  fn as_ref(&self) -> &VoiceChat { self }
}

impl AsRef<VoiceChat> for RTDVoiceChatBuilder {
  fn as_ref(&self) -> &VoiceChat { &self.inner }
}



