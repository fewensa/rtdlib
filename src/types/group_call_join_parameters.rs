use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes parameters used to join a group call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallJoinParameters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Audio channel synchronization source identifier; received from tgcalls
  audio_source_id: i64,
  /// Group call join payload; received from tgcalls
  payload: String,
  /// Pass true to join the call with muted microphone
  is_muted: bool,
  /// Pass true if the user's video is enabled
  is_my_video_enabled: bool,
}

impl RObject for GroupCallJoinParameters {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "groupCallJoinParameters"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GroupCallJoinParameters {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGroupCallJoinParametersBuilder {
    let inner = GroupCallJoinParameters {
      td_name: "groupCallJoinParameters".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGroupCallJoinParametersBuilder { inner }
  }

  pub fn audio_source_id(&self) -> i64 {
    self.audio_source_id
  }

  pub fn payload(&self) -> &String {
    &self.payload
  }

  pub fn is_muted(&self) -> bool {
    self.is_muted
  }

  pub fn is_my_video_enabled(&self) -> bool {
    self.is_my_video_enabled
  }
}

#[doc(hidden)]
pub struct RTDGroupCallJoinParametersBuilder {
  inner: GroupCallJoinParameters,
}

impl RTDGroupCallJoinParametersBuilder {
  pub fn build(&self) -> GroupCallJoinParameters {
    self.inner.clone()
  }

  pub fn audio_source_id(&mut self, audio_source_id: i64) -> &mut Self {
    self.inner.audio_source_id = audio_source_id;
    self
  }

  pub fn payload<T: AsRef<str>>(&mut self, payload: T) -> &mut Self {
    self.inner.payload = payload.as_ref().to_string();
    self
  }

  pub fn is_muted(&mut self, is_muted: bool) -> &mut Self {
    self.inner.is_muted = is_muted;
    self
  }

  pub fn is_my_video_enabled(&mut self, is_my_video_enabled: bool) -> &mut Self {
    self.inner.is_my_video_enabled = is_my_video_enabled;
    self
  }
}

impl AsRef<GroupCallJoinParameters> for GroupCallJoinParameters {
  fn as_ref(&self) -> &GroupCallJoinParameters {
    self
  }
}

impl AsRef<GroupCallJoinParameters> for RTDGroupCallJoinParametersBuilder {
  fn as_ref(&self) -> &GroupCallJoinParameters {
    &self.inner
  }
}
