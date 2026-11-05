use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of advertisements to be shown while a video from a message is watched
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VideoMessageAdvertisements {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of advertisements
  advertisements: Vec<VideoMessageAdvertisement>,
  /// Delay before the first advertisement is shown, in seconds
  start_delay: i64,
  /// Delay between consecutive advertisements, in seconds
  between_delay: i64,
}

impl RObject for VideoMessageAdvertisements {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "videoMessageAdvertisements"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl VideoMessageAdvertisements {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDVideoMessageAdvertisementsBuilder {
    let inner = VideoMessageAdvertisements {
      td_name: "videoMessageAdvertisements".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDVideoMessageAdvertisementsBuilder { inner }
  }

  pub fn advertisements(&self) -> &Vec<VideoMessageAdvertisement> {
    &self.advertisements
  }

  pub fn start_delay(&self) -> i64 {
    self.start_delay
  }

  pub fn between_delay(&self) -> i64 {
    self.between_delay
  }
}

#[doc(hidden)]
pub struct RTDVideoMessageAdvertisementsBuilder {
  inner: VideoMessageAdvertisements,
}

impl RTDVideoMessageAdvertisementsBuilder {
  pub fn build(&self) -> VideoMessageAdvertisements {
    self.inner.clone()
  }

  pub fn advertisements(&mut self, advertisements: Vec<VideoMessageAdvertisement>) -> &mut Self {
    self.inner.advertisements = advertisements;
    self
  }

  pub fn start_delay(&mut self, start_delay: i64) -> &mut Self {
    self.inner.start_delay = start_delay;
    self
  }

  pub fn between_delay(&mut self, between_delay: i64) -> &mut Self {
    self.inner.between_delay = between_delay;
    self
  }
}

impl AsRef<VideoMessageAdvertisements> for VideoMessageAdvertisements {
  fn as_ref(&self) -> &VideoMessageAdvertisements {
    self
  }
}

impl AsRef<VideoMessageAdvertisements> for RTDVideoMessageAdvertisementsBuilder {
  fn as_ref(&self) -> &VideoMessageAdvertisements {
    &self.inner
  }
}
