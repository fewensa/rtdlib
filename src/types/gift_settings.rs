use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains settings for gift receiving for a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if a button for sending a gift to the user or by the user must always be shown in the input field
  show_gift_button: bool,
  /// Types of gifts accepted by the user; for Telegram Premium users only
  accepted_gift_types: AcceptedGiftTypes,
}

impl RObject for GiftSettings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftSettings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiftSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftSettingsBuilder {
    let inner = GiftSettings {
      td_name: "giftSettings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftSettingsBuilder { inner }
  }

  pub fn show_gift_button(&self) -> bool {
    self.show_gift_button
  }

  pub fn accepted_gift_types(&self) -> &AcceptedGiftTypes {
    &self.accepted_gift_types
  }
}

#[doc(hidden)]
pub struct RTDGiftSettingsBuilder {
  inner: GiftSettings,
}

impl RTDGiftSettingsBuilder {
  pub fn build(&self) -> GiftSettings {
    self.inner.clone()
  }

  pub fn show_gift_button(&mut self, show_gift_button: bool) -> &mut Self {
    self.inner.show_gift_button = show_gift_button;
    self
  }

  pub fn accepted_gift_types<T: AsRef<AcceptedGiftTypes>>(
    &mut self,
    accepted_gift_types: T,
  ) -> &mut Self {
    self.inner.accepted_gift_types = accepted_gift_types.as_ref().clone();
    self
  }
}

impl AsRef<GiftSettings> for GiftSettings {
  fn as_ref(&self) -> &GiftSettings {
    self
  }
}

impl AsRef<GiftSettings> for RTDGiftSettingsBuilder {
  fn as_ref(&self) -> &GiftSettings {
    &self.inner
  }
}
