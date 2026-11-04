use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes verification status provided by a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotVerification {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the bot that provided the verification
  bot_user_id: i64,
  /// Identifier of the custom emoji that is used as the verification sign
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  icon_custom_emoji_id: isize,
  /// Custom description of verification reason set by the bot. Can contain only Mention, Hashtag, Cashtag, PhoneNumber, BankCardNumber, TonAddress, Url, and EmailAddress entities
  custom_description: FormattedText,
}

impl RObject for BotVerification {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "botVerification"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BotVerification {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBotVerificationBuilder {
    let inner = BotVerification {
      td_name: "botVerification".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBotVerificationBuilder { inner }
  }

  pub fn bot_user_id(&self) -> i64 {
    self.bot_user_id
  }

  pub fn icon_custom_emoji_id(&self) -> isize {
    self.icon_custom_emoji_id
  }

  pub fn custom_description(&self) -> &FormattedText {
    &self.custom_description
  }
}

#[doc(hidden)]
pub struct RTDBotVerificationBuilder {
  inner: BotVerification,
}

impl RTDBotVerificationBuilder {
  pub fn build(&self) -> BotVerification {
    self.inner.clone()
  }

  pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
    self.inner.bot_user_id = bot_user_id;
    self
  }

  pub fn icon_custom_emoji_id(&mut self, icon_custom_emoji_id: isize) -> &mut Self {
    self.inner.icon_custom_emoji_id = icon_custom_emoji_id;
    self
  }

  pub fn custom_description<T: AsRef<FormattedText>>(
    &mut self,
    custom_description: T,
  ) -> &mut Self {
    self.inner.custom_description = custom_description.as_ref().clone();
    self
  }
}

impl AsRef<BotVerification> for BotVerification {
  fn as_ref(&self) -> &BotVerification {
    self
  }
}

impl AsRef<BotVerification> for RTDBotVerificationBuilder {
  fn as_ref(&self) -> &BotVerification {
    &self.inner
  }
}
