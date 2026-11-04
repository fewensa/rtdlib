use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes parameters of verification that is provided by a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotVerificationParameters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the custom emoji that is used as the verification sign
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  icon_custom_emoji_id: isize,
  /// Name of the organization that provides verification
  organization_name: String,
  /// Default custom description of verification reason to be used as placeholder in setMessageSenderBotVerification; may be null if none
  default_custom_description: Option<FormattedText>,
  /// True, if the bot is allowed to provide custom description for verified entities
  can_set_custom_description: bool,
}

impl RObject for BotVerificationParameters {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "botVerificationParameters"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BotVerificationParameters {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBotVerificationParametersBuilder {
    let inner = BotVerificationParameters {
      td_name: "botVerificationParameters".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBotVerificationParametersBuilder { inner }
  }

  pub fn icon_custom_emoji_id(&self) -> isize {
    self.icon_custom_emoji_id
  }

  pub fn organization_name(&self) -> &String {
    &self.organization_name
  }

  pub fn default_custom_description(&self) -> &Option<FormattedText> {
    &self.default_custom_description
  }

  pub fn can_set_custom_description(&self) -> bool {
    self.can_set_custom_description
  }
}

#[doc(hidden)]
pub struct RTDBotVerificationParametersBuilder {
  inner: BotVerificationParameters,
}

impl RTDBotVerificationParametersBuilder {
  pub fn build(&self) -> BotVerificationParameters {
    self.inner.clone()
  }

  pub fn icon_custom_emoji_id(&mut self, icon_custom_emoji_id: isize) -> &mut Self {
    self.inner.icon_custom_emoji_id = icon_custom_emoji_id;
    self
  }

  pub fn organization_name<T: AsRef<str>>(&mut self, organization_name: T) -> &mut Self {
    self.inner.organization_name = organization_name.as_ref().to_string();
    self
  }

  pub fn default_custom_description<T: AsRef<FormattedText>>(
    &mut self,
    default_custom_description: T,
  ) -> &mut Self {
    self.inner.default_custom_description = Some(default_custom_description.as_ref().clone());
    self
  }

  pub fn can_set_custom_description(&mut self, can_set_custom_description: bool) -> &mut Self {
    self.inner.can_set_custom_description = can_set_custom_description;
    self
  }
}

impl AsRef<BotVerificationParameters> for BotVerificationParameters {
  fn as_ref(&self) -> &BotVerificationParameters {
    self
  }
}

impl AsRef<BotVerificationParameters> for RTDBotVerificationParametersBuilder {
  fn as_ref(&self) -> &BotVerificationParameters {
    &self.inner
  }
}
