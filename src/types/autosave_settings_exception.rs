use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains autosave settings for a chat, which overrides default settings for the corresponding scope
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutosaveSettingsException {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Autosave settings for the chat
  settings: ScopeAutosaveSettings,
}

impl RObject for AutosaveSettingsException {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "autosaveSettingsException"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AutosaveSettingsException {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAutosaveSettingsExceptionBuilder {
    let inner = AutosaveSettingsException {
      td_name: "autosaveSettingsException".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAutosaveSettingsExceptionBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn settings(&self) -> &ScopeAutosaveSettings {
    &self.settings
  }
}

#[doc(hidden)]
pub struct RTDAutosaveSettingsExceptionBuilder {
  inner: AutosaveSettingsException,
}

impl RTDAutosaveSettingsExceptionBuilder {
  pub fn build(&self) -> AutosaveSettingsException {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn settings<T: AsRef<ScopeAutosaveSettings>>(&mut self, settings: T) -> &mut Self {
    self.inner.settings = settings.as_ref().clone();
    self
  }
}

impl AsRef<AutosaveSettingsException> for AutosaveSettingsException {
  fn as_ref(&self) -> &AutosaveSettingsException {
    self
  }
}

impl AsRef<AutosaveSettingsException> for RTDAutosaveSettingsExceptionBuilder {
  fn as_ref(&self) -> &AutosaveSettingsException {
    &self.inner
  }
}
