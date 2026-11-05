use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents an available reaction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AvailableReaction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of the reaction
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: ReactionType,
  /// True, if Telegram Premium is needed to send the reaction
  needs_premium: bool,
}

impl RObject for AvailableReaction {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "availableReaction"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AvailableReaction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAvailableReactionBuilder {
    let inner = AvailableReaction {
      td_name: "availableReaction".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAvailableReactionBuilder { inner }
  }

  pub fn type_(&self) -> &ReactionType {
    &self.type_
  }

  pub fn needs_premium(&self) -> bool {
    self.needs_premium
  }
}

#[doc(hidden)]
pub struct RTDAvailableReactionBuilder {
  inner: AvailableReaction,
}

impl RTDAvailableReactionBuilder {
  pub fn build(&self) -> AvailableReaction {
    self.inner.clone()
  }

  pub fn type_<T: AsRef<ReactionType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

  pub fn needs_premium(&mut self, needs_premium: bool) -> &mut Self {
    self.inner.needs_premium = needs_premium;
    self
  }
}

impl AsRef<AvailableReaction> for AvailableReaction {
  fn as_ref(&self) -> &AvailableReaction {
    self
  }
}

impl AsRef<AvailableReaction> for RTDAvailableReactionBuilder {
  fn as_ref(&self) -> &AvailableReaction {
    &self.inner
  }
}
