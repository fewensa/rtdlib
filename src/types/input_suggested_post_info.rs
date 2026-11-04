use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a post to suggest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputSuggestedPostInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Price of the suggested post; pass null to suggest a post without payment. If the current user isn't an administrator of the channel direct messages chat and doesn't have enough funds to pay for the post, then the error "BALANCE_TOO_LOW" will be returned immediately
  price: SuggestedPostPrice,
  /// Point in time (Unix timestamp) when the post is expected to be published; pass 0 if the date isn't restricted. If specified, then the date must be getOption("suggested_post_send_delay_min")-getOption("suggested_post_send_delay_max") seconds in the future
  send_date: i64,
}

impl RObject for InputSuggestedPostInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputSuggestedPostInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputSuggestedPostInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputSuggestedPostInfoBuilder {
    let inner = InputSuggestedPostInfo {
      td_name: "inputSuggestedPostInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputSuggestedPostInfoBuilder { inner }
  }

  pub fn price(&self) -> &SuggestedPostPrice {
    &self.price
  }

  pub fn send_date(&self) -> i64 {
    self.send_date
  }
}

#[doc(hidden)]
pub struct RTDInputSuggestedPostInfoBuilder {
  inner: InputSuggestedPostInfo,
}

impl RTDInputSuggestedPostInfoBuilder {
  pub fn build(&self) -> InputSuggestedPostInfo {
    self.inner.clone()
  }

  pub fn price<T: AsRef<SuggestedPostPrice>>(&mut self, price: T) -> &mut Self {
    self.inner.price = price.as_ref().clone();
    self
  }

  pub fn send_date(&mut self, send_date: i64) -> &mut Self {
    self.inner.send_date = send_date;
    self
  }
}

impl AsRef<InputSuggestedPostInfo> for InputSuggestedPostInfo {
  fn as_ref(&self) -> &InputSuggestedPostInfo {
    self
  }
}

impl AsRef<InputSuggestedPostInfo> for RTDInputSuggestedPostInfoBuilder {
  fn as_ref(&self) -> &InputSuggestedPostInfo {
    &self.inner
  }
}
