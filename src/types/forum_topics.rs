use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a list of forum topics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ForumTopics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Approximate total number of forum topics found
  total_count: i64,
  /// List of forum topics
  topics: Vec<ForumTopic>,
  /// Offset date for the next getForumTopics request
  next_offset_date: i64,
  /// Offset message identifier for the next getForumTopics request
  next_offset_message_id: i64,
  /// Offset forum topic identifier for the next getForumTopics request
  next_offset_forum_topic_id: i64,
}

impl RObject for ForumTopics {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "forumTopics"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ForumTopics {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDForumTopicsBuilder {
    let inner = ForumTopics {
      td_name: "forumTopics".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDForumTopicsBuilder { inner }
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn topics(&self) -> &Vec<ForumTopic> {
    &self.topics
  }

  pub fn next_offset_date(&self) -> i64 {
    self.next_offset_date
  }

  pub fn next_offset_message_id(&self) -> i64 {
    self.next_offset_message_id
  }

  pub fn next_offset_forum_topic_id(&self) -> i64 {
    self.next_offset_forum_topic_id
  }
}

#[doc(hidden)]
pub struct RTDForumTopicsBuilder {
  inner: ForumTopics,
}

impl RTDForumTopicsBuilder {
  pub fn build(&self) -> ForumTopics {
    self.inner.clone()
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn topics(&mut self, topics: Vec<ForumTopic>) -> &mut Self {
    self.inner.topics = topics;
    self
  }

  pub fn next_offset_date(&mut self, next_offset_date: i64) -> &mut Self {
    self.inner.next_offset_date = next_offset_date;
    self
  }

  pub fn next_offset_message_id(&mut self, next_offset_message_id: i64) -> &mut Self {
    self.inner.next_offset_message_id = next_offset_message_id;
    self
  }

  pub fn next_offset_forum_topic_id(&mut self, next_offset_forum_topic_id: i64) -> &mut Self {
    self.inner.next_offset_forum_topic_id = next_offset_forum_topic_id;
    self
  }
}

impl AsRef<ForumTopics> for ForumTopics {
  fn as_ref(&self) -> &ForumTopics {
    self
  }
}

impl AsRef<ForumTopics> for RTDForumTopicsBuilder {
  fn as_ref(&self) -> &ForumTopics {
    &self.inner
  }
}
