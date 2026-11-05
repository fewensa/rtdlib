use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains identifiers of group call participants
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallParticipants {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Total number of group call participants
  total_count: i64,
  /// Identifiers of the participants
  participant_ids: Vec<MessageSender>,
}

impl RObject for GroupCallParticipants {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "groupCallParticipants"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GroupCallParticipants {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGroupCallParticipantsBuilder {
    let inner = GroupCallParticipants {
      td_name: "groupCallParticipants".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGroupCallParticipantsBuilder { inner }
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn participant_ids(&self) -> &Vec<MessageSender> {
    &self.participant_ids
  }
}

#[doc(hidden)]
pub struct RTDGroupCallParticipantsBuilder {
  inner: GroupCallParticipants,
}

impl RTDGroupCallParticipantsBuilder {
  pub fn build(&self) -> GroupCallParticipants {
    self.inner.clone()
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn participant_ids(&mut self, participant_ids: Vec<MessageSender>) -> &mut Self {
    self.inner.participant_ids = participant_ids;
    self
  }
}

impl AsRef<GroupCallParticipants> for GroupCallParticipants {
  fn as_ref(&self) -> &GroupCallParticipants {
    self
  }
}

impl AsRef<GroupCallParticipants> for RTDGroupCallParticipantsBuilder {
  fn as_ref(&self) -> &GroupCallParticipants {
    &self.inner
  }
}
