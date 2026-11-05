use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about verification status of a chat or a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VerificationStatus {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the chat or the user is verified by Telegram
  is_verified: bool,
  /// True, if the chat or the user is marked as scam by Telegram
  is_scam: bool,
  /// True, if the chat or the user is marked as fake by Telegram
  is_fake: bool,
  /// Identifier of the custom emoji to be shown as verification sign provided by a bot for the user; 0 if none
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  bot_verification_icon_custom_emoji_id: isize,
}

impl RObject for VerificationStatus {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "verificationStatus"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl VerificationStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDVerificationStatusBuilder {
    let inner = VerificationStatus {
      td_name: "verificationStatus".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDVerificationStatusBuilder { inner }
  }

  pub fn is_verified(&self) -> bool {
    self.is_verified
  }

  pub fn is_scam(&self) -> bool {
    self.is_scam
  }

  pub fn is_fake(&self) -> bool {
    self.is_fake
  }

  pub fn bot_verification_icon_custom_emoji_id(&self) -> isize {
    self.bot_verification_icon_custom_emoji_id
  }
}

#[doc(hidden)]
pub struct RTDVerificationStatusBuilder {
  inner: VerificationStatus,
}

impl RTDVerificationStatusBuilder {
  pub fn build(&self) -> VerificationStatus {
    self.inner.clone()
  }

  pub fn is_verified(&mut self, is_verified: bool) -> &mut Self {
    self.inner.is_verified = is_verified;
    self
  }

  pub fn is_scam(&mut self, is_scam: bool) -> &mut Self {
    self.inner.is_scam = is_scam;
    self
  }

  pub fn is_fake(&mut self, is_fake: bool) -> &mut Self {
    self.inner.is_fake = is_fake;
    self
  }

  pub fn bot_verification_icon_custom_emoji_id(
    &mut self,
    bot_verification_icon_custom_emoji_id: isize,
  ) -> &mut Self {
    self.inner.bot_verification_icon_custom_emoji_id = bot_verification_icon_custom_emoji_id;
    self
  }
}

impl AsRef<VerificationStatus> for VerificationStatus {
  fn as_ref(&self) -> &VerificationStatus {
    self
  }
}

impl AsRef<VerificationStatus> for RTDVerificationStatusBuilder {
  fn as_ref(&self) -> &VerificationStatus {
    &self.inner
  }
}
