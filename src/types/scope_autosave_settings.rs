use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains autosave settings for an autosave settings scope
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScopeAutosaveSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if photo autosave is enabled
  autosave_photos: bool,
  /// True, if video autosave is enabled
  autosave_videos: bool,
  /// The maximum size of a video file to be autosaved, in bytes; 512 KB  4000 MB
  max_video_file_size: i64,
}

impl RObject for ScopeAutosaveSettings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "scopeAutosaveSettings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ScopeAutosaveSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDScopeAutosaveSettingsBuilder {
    let inner = ScopeAutosaveSettings {
      td_name: "scopeAutosaveSettings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDScopeAutosaveSettingsBuilder { inner }
  }

  pub fn autosave_photos(&self) -> bool {
    self.autosave_photos
  }

  pub fn autosave_videos(&self) -> bool {
    self.autosave_videos
  }

  pub fn max_video_file_size(&self) -> i64 {
    self.max_video_file_size
  }
}

#[doc(hidden)]
pub struct RTDScopeAutosaveSettingsBuilder {
  inner: ScopeAutosaveSettings,
}

impl RTDScopeAutosaveSettingsBuilder {
  pub fn build(&self) -> ScopeAutosaveSettings {
    self.inner.clone()
  }

  pub fn autosave_photos(&mut self, autosave_photos: bool) -> &mut Self {
    self.inner.autosave_photos = autosave_photos;
    self
  }

  pub fn autosave_videos(&mut self, autosave_videos: bool) -> &mut Self {
    self.inner.autosave_videos = autosave_videos;
    self
  }

  pub fn max_video_file_size(&mut self, max_video_file_size: i64) -> &mut Self {
    self.inner.max_video_file_size = max_video_file_size;
    self
  }
}

impl AsRef<ScopeAutosaveSettings> for ScopeAutosaveSettings {
  fn as_ref(&self) -> &ScopeAutosaveSettings {
    self
  }
}

impl AsRef<ScopeAutosaveSettings> for RTDScopeAutosaveSettingsBuilder {
  fn as_ref(&self) -> &ScopeAutosaveSettings {
    &self.inner
  }
}
