use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of options for creating of Telegram Star giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarGiveawayPaymentOptions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of options
  options: Vec<StarGiveawayPaymentOption>,
}

impl RObject for StarGiveawayPaymentOptions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starGiveawayPaymentOptions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StarGiveawayPaymentOptions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarGiveawayPaymentOptionsBuilder {
    let inner = StarGiveawayPaymentOptions {
      td_name: "starGiveawayPaymentOptions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarGiveawayPaymentOptionsBuilder { inner }
  }

  pub fn options(&self) -> &Vec<StarGiveawayPaymentOption> {
    &self.options
  }
}

#[doc(hidden)]
pub struct RTDStarGiveawayPaymentOptionsBuilder {
  inner: StarGiveawayPaymentOptions,
}

impl RTDStarGiveawayPaymentOptionsBuilder {
  pub fn build(&self) -> StarGiveawayPaymentOptions {
    self.inner.clone()
  }

  pub fn options(&mut self, options: Vec<StarGiveawayPaymentOption>) -> &mut Self {
    self.inner.options = options;
    self
  }
}

impl AsRef<StarGiveawayPaymentOptions> for StarGiveawayPaymentOptions {
  fn as_ref(&self) -> &StarGiveawayPaymentOptions {
    self
  }
}

impl AsRef<StarGiveawayPaymentOptions> for RTDStarGiveawayPaymentOptionsBuilder {
  fn as_ref(&self) -> &StarGiveawayPaymentOptions {
    &self.inner
  }
}
