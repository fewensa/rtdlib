use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about revenue earned from sponsored messages in a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatRevenueAmount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Cryptocurrency in which revenue is calculated
  cryptocurrency: String,
  /// Total amount of the cryptocurrency earned, in the smallest units of the cryptocurrency
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  total_amount: isize,
  /// Amount of the cryptocurrency that isn't withdrawn yet, in the smallest units of the cryptocurrency
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  balance_amount: isize,
  /// Amount of the cryptocurrency available for withdrawal, in the smallest units of the cryptocurrency
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  available_amount: isize,
  /// True, if the cryptocurrency can be withdrawn now or later
  withdrawal_enabled: bool,
}

impl RObject for ChatRevenueAmount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatRevenueAmount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatRevenueAmount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatRevenueAmountBuilder {
    let inner = ChatRevenueAmount {
      td_name: "chatRevenueAmount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatRevenueAmountBuilder { inner }
  }

  pub fn cryptocurrency(&self) -> &String {
    &self.cryptocurrency
  }

  pub fn total_amount(&self) -> isize {
    self.total_amount
  }

  pub fn balance_amount(&self) -> isize {
    self.balance_amount
  }

  pub fn available_amount(&self) -> isize {
    self.available_amount
  }

  pub fn withdrawal_enabled(&self) -> bool {
    self.withdrawal_enabled
  }
}

#[doc(hidden)]
pub struct RTDChatRevenueAmountBuilder {
  inner: ChatRevenueAmount,
}

impl RTDChatRevenueAmountBuilder {
  pub fn build(&self) -> ChatRevenueAmount {
    self.inner.clone()
  }

  pub fn cryptocurrency<T: AsRef<str>>(&mut self, cryptocurrency: T) -> &mut Self {
    self.inner.cryptocurrency = cryptocurrency.as_ref().to_string();
    self
  }

  pub fn total_amount(&mut self, total_amount: isize) -> &mut Self {
    self.inner.total_amount = total_amount;
    self
  }

  pub fn balance_amount(&mut self, balance_amount: isize) -> &mut Self {
    self.inner.balance_amount = balance_amount;
    self
  }

  pub fn available_amount(&mut self, available_amount: isize) -> &mut Self {
    self.inner.available_amount = available_amount;
    self
  }

  pub fn withdrawal_enabled(&mut self, withdrawal_enabled: bool) -> &mut Self {
    self.inner.withdrawal_enabled = withdrawal_enabled;
    self
  }
}

impl AsRef<ChatRevenueAmount> for ChatRevenueAmount {
  fn as_ref(&self) -> &ChatRevenueAmount {
    self
  }
}

impl AsRef<ChatRevenueAmount> for RTDChatRevenueAmountBuilder {
  fn as_ref(&self) -> &ChatRevenueAmount {
    &self.inner
  }
}
