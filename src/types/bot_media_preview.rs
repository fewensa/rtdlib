use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes media previews of a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotMediaPreview {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Point in time (Unix timestamp) when the preview was added or changed last time
  date: i64,
  /// Content of the preview; may only be of the types storyContentPhoto, storyContentVideo, or storyContentUnsupported
  content: StoryContent,
}

impl RObject for BotMediaPreview {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "botMediaPreview"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BotMediaPreview {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBotMediaPreviewBuilder {
    let inner = BotMediaPreview {
      td_name: "botMediaPreview".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBotMediaPreviewBuilder { inner }
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn content(&self) -> &StoryContent {
    &self.content
  }
}

#[doc(hidden)]
pub struct RTDBotMediaPreviewBuilder {
  inner: BotMediaPreview,
}

impl RTDBotMediaPreviewBuilder {
  pub fn build(&self) -> BotMediaPreview {
    self.inner.clone()
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

  pub fn content<T: AsRef<StoryContent>>(&mut self, content: T) -> &mut Self {
    self.inner.content = content.as_ref().clone();
    self
  }
}

impl AsRef<BotMediaPreview> for BotMediaPreview {
  fn as_ref(&self) -> &BotMediaPreview {
    self
  }
}

impl AsRef<BotMediaPreview> for RTDBotMediaPreviewBuilder {
  fn as_ref(&self) -> &BotMediaPreview {
    &self.inner
  }
}
