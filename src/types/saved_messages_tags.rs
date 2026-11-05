use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of tags used in Saved Messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SavedMessagesTags {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of tags
  tags: Vec<SavedMessagesTag>,
}

impl RObject for SavedMessagesTags {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "savedMessagesTags"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SavedMessagesTags {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSavedMessagesTagsBuilder {
    let inner = SavedMessagesTags {
      td_name: "savedMessagesTags".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSavedMessagesTagsBuilder { inner }
  }

  pub fn tags(&self) -> &Vec<SavedMessagesTag> {
    &self.tags
  }
}

#[doc(hidden)]
pub struct RTDSavedMessagesTagsBuilder {
  inner: SavedMessagesTags,
}

impl RTDSavedMessagesTagsBuilder {
  pub fn build(&self) -> SavedMessagesTags {
    self.inner.clone()
  }

  pub fn tags(&mut self, tags: Vec<SavedMessagesTag>) -> &mut Self {
    self.inner.tags = tags;
    self
  }
}

impl AsRef<SavedMessagesTags> for SavedMessagesTags {
  fn as_ref(&self) -> &SavedMessagesTags {
    self
  }
}

impl AsRef<SavedMessagesTags> for RTDSavedMessagesTagsBuilder {
  fn as_ref(&self) -> &SavedMessagesTags {
    &self.inner
  }
}
