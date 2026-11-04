use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of boosts applied to a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoundChatBoosts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Total number of boosts applied to the chat
  total_count: i64,
  /// List of boosts
  boosts: Vec<ChatBoost>,
  /// The offset for the next request. If empty, then there are no more results
  next_offset: String,
}

impl RObject for FoundChatBoosts {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "foundChatBoosts"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl FoundChatBoosts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFoundChatBoostsBuilder {
    let inner = FoundChatBoosts {
      td_name: "foundChatBoosts".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDFoundChatBoostsBuilder { inner }
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn boosts(&self) -> &Vec<ChatBoost> {
    &self.boosts
  }

  pub fn next_offset(&self) -> &String {
    &self.next_offset
  }
}

#[doc(hidden)]
pub struct RTDFoundChatBoostsBuilder {
  inner: FoundChatBoosts,
}

impl RTDFoundChatBoostsBuilder {
  pub fn build(&self) -> FoundChatBoosts {
    self.inner.clone()
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn boosts(&mut self, boosts: Vec<ChatBoost>) -> &mut Self {
    self.inner.boosts = boosts;
    self
  }

  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }
}

impl AsRef<FoundChatBoosts> for FoundChatBoosts {
  fn as_ref(&self) -> &FoundChatBoosts {
    self
  }
}

impl AsRef<FoundChatBoosts> for RTDFoundChatBoostsBuilder {
  fn as_ref(&self) -> &FoundChatBoosts {
    &self.inner
  }
}
