use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of gifts that can be sent to another user or channel chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AvailableGifts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of gifts
  gifts: Vec<AvailableGift>,
}

impl RObject for AvailableGifts {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "availableGifts"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AvailableGifts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAvailableGiftsBuilder {
    let inner = AvailableGifts {
      td_name: "availableGifts".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAvailableGiftsBuilder { inner }
  }

  pub fn gifts(&self) -> &Vec<AvailableGift> {
    &self.gifts
  }
}

#[doc(hidden)]
pub struct RTDAvailableGiftsBuilder {
  inner: AvailableGifts,
}

impl RTDAvailableGiftsBuilder {
  pub fn build(&self) -> AvailableGifts {
    self.inner.clone()
  }

  pub fn gifts(&mut self, gifts: Vec<AvailableGift>) -> &mut Self {
    self.inner.gifts = gifts;
    self
  }
}

impl AsRef<AvailableGifts> for AvailableGifts {
  fn as_ref(&self) -> &AvailableGifts {
    self
  }
}

impl AsRef<AvailableGifts> for RTDAvailableGiftsBuilder {
  fn as_ref(&self) -> &AvailableGifts {
    &self.inner
  }
}
