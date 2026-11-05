use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of media previews of a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotMediaPreviews {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of media previews
  previews: Vec<BotMediaPreview>,
}

impl RObject for BotMediaPreviews {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "botMediaPreviews"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BotMediaPreviews {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBotMediaPreviewsBuilder {
    let inner = BotMediaPreviews {
      td_name: "botMediaPreviews".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBotMediaPreviewsBuilder { inner }
  }

  pub fn previews(&self) -> &Vec<BotMediaPreview> {
    &self.previews
  }
}

#[doc(hidden)]
pub struct RTDBotMediaPreviewsBuilder {
  inner: BotMediaPreviews,
}

impl RTDBotMediaPreviewsBuilder {
  pub fn build(&self) -> BotMediaPreviews {
    self.inner.clone()
  }

  pub fn previews(&mut self, previews: Vec<BotMediaPreview>) -> &mut Self {
    self.inner.previews = previews;
    self
  }
}

impl AsRef<BotMediaPreviews> for BotMediaPreviews {
  fn as_ref(&self) -> &BotMediaPreviews {
    self
  }
}

impl AsRef<BotMediaPreviews> for RTDBotMediaPreviewsBuilder {
  fn as_ref(&self) -> &BotMediaPreviews {
    &self.inner
  }
}
