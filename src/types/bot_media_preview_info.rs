use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of media previews of a bot for the given language and the list of languages for which the bot has dedicated previews
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotMediaPreviewInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of media previews
  previews: Vec<BotMediaPreview>,
  /// List of language codes for which the bot has dedicated previews
  language_codes: Vec<String>,
}

impl RObject for BotMediaPreviewInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "botMediaPreviewInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BotMediaPreviewInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBotMediaPreviewInfoBuilder {
    let inner = BotMediaPreviewInfo {
      td_name: "botMediaPreviewInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBotMediaPreviewInfoBuilder { inner }
  }

  pub fn previews(&self) -> &Vec<BotMediaPreview> {
    &self.previews
  }

  pub fn language_codes(&self) -> &Vec<String> {
    &self.language_codes
  }
}

#[doc(hidden)]
pub struct RTDBotMediaPreviewInfoBuilder {
  inner: BotMediaPreviewInfo,
}

impl RTDBotMediaPreviewInfoBuilder {
  pub fn build(&self) -> BotMediaPreviewInfo {
    self.inner.clone()
  }

  pub fn previews(&mut self, previews: Vec<BotMediaPreview>) -> &mut Self {
    self.inner.previews = previews;
    self
  }

  pub fn language_codes(&mut self, language_codes: Vec<String>) -> &mut Self {
    self.inner.language_codes = language_codes;
    self
  }
}

impl AsRef<BotMediaPreviewInfo> for BotMediaPreviewInfo {
  fn as_ref(&self) -> &BotMediaPreviewInfo {
    self
  }
}

impl AsRef<BotMediaPreviewInfo> for RTDBotMediaPreviewInfoBuilder {
  fn as_ref(&self) -> &BotMediaPreviewInfo {
    &self.inner
  }
}
