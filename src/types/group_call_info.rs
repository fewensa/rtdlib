use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a just created or just joined group call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the group call
  group_call_id: i64,
  /// Join response payload for tgcalls; empty if the call isn't joined
  join_payload: String,
}

impl RObject for GroupCallInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "groupCallInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GroupCallInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGroupCallInfoBuilder {
    let inner = GroupCallInfo {
      td_name: "groupCallInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGroupCallInfoBuilder { inner }
  }

  pub fn group_call_id(&self) -> i64 {
    self.group_call_id
  }

  pub fn join_payload(&self) -> &String {
    &self.join_payload
  }
}

#[doc(hidden)]
pub struct RTDGroupCallInfoBuilder {
  inner: GroupCallInfo,
}

impl RTDGroupCallInfoBuilder {
  pub fn build(&self) -> GroupCallInfo {
    self.inner.clone()
  }

  pub fn group_call_id(&mut self, group_call_id: i64) -> &mut Self {
    self.inner.group_call_id = group_call_id;
    self
  }

  pub fn join_payload<T: AsRef<str>>(&mut self, join_payload: T) -> &mut Self {
    self.inner.join_payload = join_payload.as_ref().to_string();
    self
  }
}

impl AsRef<GroupCallInfo> for GroupCallInfo {
  fn as_ref(&self) -> &GroupCallInfo {
    self
  }
}

impl AsRef<GroupCallInfo> for RTDGroupCallInfoBuilder {
  fn as_ref(&self) -> &GroupCallInfo {
    &self.inner
  }
}
