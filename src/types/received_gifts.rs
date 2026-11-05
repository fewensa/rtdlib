use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of gifts received by a user or a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReceivedGifts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The total number of received gifts
  total_count: i64,
  /// The list of gifts
  gifts: Vec<ReceivedGift>,
  /// True, if notifications about new gifts of the owner are enabled
  are_notifications_enabled: bool,
  /// The offset for the next request. If empty, then there are no more results
  next_offset: String,
}

impl RObject for ReceivedGifts {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "receivedGifts"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ReceivedGifts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReceivedGiftsBuilder {
    let inner = ReceivedGifts {
      td_name: "receivedGifts".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDReceivedGiftsBuilder { inner }
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn gifts(&self) -> &Vec<ReceivedGift> {
    &self.gifts
  }

  pub fn are_notifications_enabled(&self) -> bool {
    self.are_notifications_enabled
  }

  pub fn next_offset(&self) -> &String {
    &self.next_offset
  }
}

#[doc(hidden)]
pub struct RTDReceivedGiftsBuilder {
  inner: ReceivedGifts,
}

impl RTDReceivedGiftsBuilder {
  pub fn build(&self) -> ReceivedGifts {
    self.inner.clone()
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn gifts(&mut self, gifts: Vec<ReceivedGift>) -> &mut Self {
    self.inner.gifts = gifts;
    self
  }

  pub fn are_notifications_enabled(&mut self, are_notifications_enabled: bool) -> &mut Self {
    self.inner.are_notifications_enabled = are_notifications_enabled;
    self
  }

  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }
}

impl AsRef<ReceivedGifts> for ReceivedGifts {
  fn as_ref(&self) -> &ReceivedGifts {
    self
  }
}

impl AsRef<ReceivedGifts> for RTDReceivedGiftsBuilder {
  fn as_ref(&self) -> &ReceivedGifts {
    &self.inner
  }
}
