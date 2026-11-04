use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a tag used in Saved Messages or a Saved Messages topic
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SavedMessagesTag {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The tag
  tag: ReactionType,
  /// Label of the tag; 0-12 characters. Always empty if the tag is returned for a Saved Messages topic
  label: String,
  /// Number of times the tag was used; may be 0 if the tag has non-empty label
  count: i64,
}

impl RObject for SavedMessagesTag {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "savedMessagesTag"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SavedMessagesTag {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSavedMessagesTagBuilder {
    let inner = SavedMessagesTag {
      td_name: "savedMessagesTag".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSavedMessagesTagBuilder { inner }
  }

  pub fn tag(&self) -> &ReactionType {
    &self.tag
  }

  pub fn label(&self) -> &String {
    &self.label
  }

  pub fn count(&self) -> i64 {
    self.count
  }
}

#[doc(hidden)]
pub struct RTDSavedMessagesTagBuilder {
  inner: SavedMessagesTag,
}

impl RTDSavedMessagesTagBuilder {
  pub fn build(&self) -> SavedMessagesTag {
    self.inner.clone()
  }

  pub fn tag<T: AsRef<ReactionType>>(&mut self, tag: T) -> &mut Self {
    self.inner.tag = tag.as_ref().clone();
    self
  }

  pub fn label<T: AsRef<str>>(&mut self, label: T) -> &mut Self {
    self.inner.label = label.as_ref().to_string();
    self
  }

  pub fn count(&mut self, count: i64) -> &mut Self {
    self.inner.count = count;
    self
  }
}

impl AsRef<SavedMessagesTag> for SavedMessagesTag {
  fn as_ref(&self) -> &SavedMessagesTag {
    self
  }
}

impl AsRef<SavedMessagesTag> for RTDSavedMessagesTagBuilder {
  fn as_ref(&self) -> &SavedMessagesTag {
    &self.inner
  }
}
