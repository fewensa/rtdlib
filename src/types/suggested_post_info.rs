use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a suggested post. If the post can be approved or declined, then changes to the post can be also suggested. Use sendMessage with reply to the message and suggested post information to suggest message changes. Use addOffer to suggest price or time changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedPostInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Price of the suggested post; may be null if the post is non-paid
  price: Option<SuggestedPostPrice>,
  /// Point in time (Unix timestamp) when the post is expected to be published; 0 if the specific date isn't set yet
  send_date: i64,
  /// State of the post
  state: SuggestedPostState,
  /// True, if the suggested post can be approved by the current user using approveSuggestedPost; updates aren't sent when value of this field changes
  can_be_approved: bool,
  /// True, if the suggested post can be declined by the current user using declineSuggestedPost; updates aren't sent when value of this field changes
  can_be_declined: bool,
}

impl RObject for SuggestedPostInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedPostInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SuggestedPostInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedPostInfoBuilder {
    let inner = SuggestedPostInfo {
      td_name: "suggestedPostInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSuggestedPostInfoBuilder { inner }
  }

  pub fn price(&self) -> &Option<SuggestedPostPrice> {
    &self.price
  }

  pub fn send_date(&self) -> i64 {
    self.send_date
  }

  pub fn state(&self) -> &SuggestedPostState {
    &self.state
  }

  pub fn can_be_approved(&self) -> bool {
    self.can_be_approved
  }

  pub fn can_be_declined(&self) -> bool {
    self.can_be_declined
  }
}

#[doc(hidden)]
pub struct RTDSuggestedPostInfoBuilder {
  inner: SuggestedPostInfo,
}

impl RTDSuggestedPostInfoBuilder {
  pub fn build(&self) -> SuggestedPostInfo {
    self.inner.clone()
  }

  pub fn price<T: AsRef<SuggestedPostPrice>>(&mut self, price: T) -> &mut Self {
    self.inner.price = Some(price.as_ref().clone());
    self
  }

  pub fn send_date(&mut self, send_date: i64) -> &mut Self {
    self.inner.send_date = send_date;
    self
  }

  pub fn state<T: AsRef<SuggestedPostState>>(&mut self, state: T) -> &mut Self {
    self.inner.state = state.as_ref().clone();
    self
  }

  pub fn can_be_approved(&mut self, can_be_approved: bool) -> &mut Self {
    self.inner.can_be_approved = can_be_approved;
    self
  }

  pub fn can_be_declined(&mut self, can_be_declined: bool) -> &mut Self {
    self.inner.can_be_declined = can_be_declined;
    self
  }
}

impl AsRef<SuggestedPostInfo> for SuggestedPostInfo {
  fn as_ref(&self) -> &SuggestedPostInfo {
    self
  }
}

impl AsRef<SuggestedPostInfo> for RTDSuggestedPostInfoBuilder {
  fn as_ref(&self) -> &SuggestedPostInfo {
    &self.inner
  }
}
