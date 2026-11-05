use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of reactions added to a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddedReactions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The total number of found reactions
  total_count: i64,
  /// The list of added reactions
  reactions: Vec<AddedReaction>,
  /// The offset for the next request. If empty, then there are no more results
  next_offset: String,
}

impl RObject for AddedReactions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "addedReactions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AddedReactions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAddedReactionsBuilder {
    let inner = AddedReactions {
      td_name: "addedReactions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAddedReactionsBuilder { inner }
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn reactions(&self) -> &Vec<AddedReaction> {
    &self.reactions
  }

  pub fn next_offset(&self) -> &String {
    &self.next_offset
  }
}

#[doc(hidden)]
pub struct RTDAddedReactionsBuilder {
  inner: AddedReactions,
}

impl RTDAddedReactionsBuilder {
  pub fn build(&self) -> AddedReactions {
    self.inner.clone()
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn reactions(&mut self, reactions: Vec<AddedReaction>) -> &mut Self {
    self.inner.reactions = reactions;
    self
  }

  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }
}

impl AsRef<AddedReactions> for AddedReactions {
  fn as_ref(&self) -> &AddedReactions {
    self
  }
}

impl AsRef<AddedReactions> for RTDAddedReactionsBuilder {
  fn as_ref(&self) -> &AddedReactions {
    &self.inner
  }
}
