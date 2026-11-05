use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of audio files
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Audios {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Approximate total number of audio files found
  total_count: i64,
  /// List of audio files
  audios: Vec<Audio>,
}

impl RObject for Audios {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "audios"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl Audios {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAudiosBuilder {
    let inner = Audios {
      td_name: "audios".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAudiosBuilder { inner }
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn audios(&self) -> &Vec<Audio> {
    &self.audios
  }
}

#[doc(hidden)]
pub struct RTDAudiosBuilder {
  inner: Audios,
}

impl RTDAudiosBuilder {
  pub fn build(&self) -> Audios {
    self.inner.clone()
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn audios(&mut self, audios: Vec<Audio>) -> &mut Self {
    self.inner.audios = audios;
    self
  }
}

impl AsRef<Audios> for Audios {
  fn as_ref(&self) -> &Audios {
    self
  }
}

impl AsRef<Audios> for RTDAudiosBuilder {
  fn as_ref(&self) -> &Audios {
    &self.inner
  }
}
