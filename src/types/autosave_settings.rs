use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes autosave settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutosaveSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Default autosave settings for private chats
  private_chat_settings: ScopeAutosaveSettings,
  /// Default autosave settings for basic group and supergroup chats
  group_settings: ScopeAutosaveSettings,
  /// Default autosave settings for channel chats
  channel_settings: ScopeAutosaveSettings,
  /// Autosave settings for specific chats
  exceptions: Vec<AutosaveSettingsException>,
}

impl RObject for AutosaveSettings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "autosaveSettings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AutosaveSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAutosaveSettingsBuilder {
    let inner = AutosaveSettings {
      td_name: "autosaveSettings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAutosaveSettingsBuilder { inner }
  }

  pub fn private_chat_settings(&self) -> &ScopeAutosaveSettings {
    &self.private_chat_settings
  }

  pub fn group_settings(&self) -> &ScopeAutosaveSettings {
    &self.group_settings
  }

  pub fn channel_settings(&self) -> &ScopeAutosaveSettings {
    &self.channel_settings
  }

  pub fn exceptions(&self) -> &Vec<AutosaveSettingsException> {
    &self.exceptions
  }
}

#[doc(hidden)]
pub struct RTDAutosaveSettingsBuilder {
  inner: AutosaveSettings,
}

impl RTDAutosaveSettingsBuilder {
  pub fn build(&self) -> AutosaveSettings {
    self.inner.clone()
  }

  pub fn private_chat_settings<T: AsRef<ScopeAutosaveSettings>>(
    &mut self,
    private_chat_settings: T,
  ) -> &mut Self {
    self.inner.private_chat_settings = private_chat_settings.as_ref().clone();
    self
  }

  pub fn group_settings<T: AsRef<ScopeAutosaveSettings>>(
    &mut self,
    group_settings: T,
  ) -> &mut Self {
    self.inner.group_settings = group_settings.as_ref().clone();
    self
  }

  pub fn channel_settings<T: AsRef<ScopeAutosaveSettings>>(
    &mut self,
    channel_settings: T,
  ) -> &mut Self {
    self.inner.channel_settings = channel_settings.as_ref().clone();
    self
  }

  pub fn exceptions(&mut self, exceptions: Vec<AutosaveSettingsException>) -> &mut Self {
    self.inner.exceptions = exceptions;
    self
  }
}

impl AsRef<AutosaveSettings> for AutosaveSettings {
  fn as_ref(&self) -> &AutosaveSettings {
    self
  }
}

impl AsRef<AutosaveSettings> for RTDAutosaveSettingsBuilder {
  fn as_ref(&self) -> &AutosaveSettings {
    &self.inner
  }
}
