use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a forum topic icon
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ForumTopicIcon {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Color of the topic icon in RGB format
  color: i64,
  /// Unique identifier of the custom emoji shown on the topic icon; 0 if none
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  custom_emoji_id: isize,
}

impl RObject for ForumTopicIcon {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "forumTopicIcon"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ForumTopicIcon {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDForumTopicIconBuilder {
    let inner = ForumTopicIcon {
      td_name: "forumTopicIcon".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDForumTopicIconBuilder { inner }
  }

  pub fn color(&self) -> i64 {
    self.color
  }

  pub fn custom_emoji_id(&self) -> isize {
    self.custom_emoji_id
  }
}

#[doc(hidden)]
pub struct RTDForumTopicIconBuilder {
  inner: ForumTopicIcon,
}

impl RTDForumTopicIconBuilder {
  pub fn build(&self) -> ForumTopicIcon {
    self.inner.clone()
  }

  pub fn color(&mut self, color: i64) -> &mut Self {
    self.inner.color = color;
    self
  }

  pub fn custom_emoji_id(&mut self, custom_emoji_id: isize) -> &mut Self {
    self.inner.custom_emoji_id = custom_emoji_id;
    self
  }
}

impl AsRef<ForumTopicIcon> for ForumTopicIcon {
  fn as_ref(&self) -> &ForumTopicIcon {
    self
  }
}

impl AsRef<ForumTopicIcon> for RTDForumTopicIconBuilder {
  fn as_ref(&self) -> &ForumTopicIcon {
    &self.inner
  }
}
