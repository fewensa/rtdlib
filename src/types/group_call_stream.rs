use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an available stream in a video chat or a live story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallStream {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of an audio/video channel
  channel_id: i64,
  /// Scale of segment durations in the stream. The duration is 1000/(2**scale) milliseconds
  scale: i64,
  /// Point in time when the stream currently ends; Unix timestamp in milliseconds
  time_offset: i64,
}

impl RObject for GroupCallStream {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "groupCallStream"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GroupCallStream {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGroupCallStreamBuilder {
    let inner = GroupCallStream {
      td_name: "groupCallStream".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGroupCallStreamBuilder { inner }
  }

  pub fn channel_id(&self) -> i64 {
    self.channel_id
  }

  pub fn scale(&self) -> i64 {
    self.scale
  }

  pub fn time_offset(&self) -> i64 {
    self.time_offset
  }
}

#[doc(hidden)]
pub struct RTDGroupCallStreamBuilder {
  inner: GroupCallStream,
}

impl RTDGroupCallStreamBuilder {
  pub fn build(&self) -> GroupCallStream {
    self.inner.clone()
  }

  pub fn channel_id(&mut self, channel_id: i64) -> &mut Self {
    self.inner.channel_id = channel_id;
    self
  }

  pub fn scale(&mut self, scale: i64) -> &mut Self {
    self.inner.scale = scale;
    self
  }

  pub fn time_offset(&mut self, time_offset: i64) -> &mut Self {
    self.inner.time_offset = time_offset;
    self
  }
}

impl AsRef<GroupCallStream> for GroupCallStream {
  fn as_ref(&self) -> &GroupCallStream {
    self
  }
}

impl AsRef<GroupCallStream> for RTDGroupCallStreamBuilder {
  fn as_ref(&self) -> &GroupCallStream {
    &self.inner
  }
}
