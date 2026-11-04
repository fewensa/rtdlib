use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a Saved Messages topic
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SavedMessagesTopic {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique topic identifier
  id: i64,
  /// Type of the topic
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: SavedMessagesTopicType,
  /// True, if the topic is pinned
  is_pinned: bool,
  /// A parameter used to determine order of the topic in the topic list. Topics must be sorted by the order in descending order
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  order: isize,
  /// Last message in the topic; may be null if none or unknown
  last_message: Option<Message>,
  /// A draft of a message in the topic; may be null if none
  draft_message: Option<DraftMessage>,
}

impl RObject for SavedMessagesTopic {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "savedMessagesTopic"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SavedMessagesTopic {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSavedMessagesTopicBuilder {
    let inner = SavedMessagesTopic {
      td_name: "savedMessagesTopic".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSavedMessagesTopicBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn type_(&self) -> &SavedMessagesTopicType {
    &self.type_
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }

  pub fn order(&self) -> isize {
    self.order
  }

  pub fn last_message(&self) -> &Option<Message> {
    &self.last_message
  }

  pub fn draft_message(&self) -> &Option<DraftMessage> {
    &self.draft_message
  }
}

#[doc(hidden)]
pub struct RTDSavedMessagesTopicBuilder {
  inner: SavedMessagesTopic,
}

impl RTDSavedMessagesTopicBuilder {
  pub fn build(&self) -> SavedMessagesTopic {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn type_<T: AsRef<SavedMessagesTopicType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }

  pub fn order(&mut self, order: isize) -> &mut Self {
    self.inner.order = order;
    self
  }

  pub fn last_message<T: AsRef<Message>>(&mut self, last_message: T) -> &mut Self {
    self.inner.last_message = Some(last_message.as_ref().clone());
    self
  }

  pub fn draft_message<T: AsRef<DraftMessage>>(&mut self, draft_message: T) -> &mut Self {
    self.inner.draft_message = Some(draft_message.as_ref().clone());
    self
  }
}

impl AsRef<SavedMessagesTopic> for SavedMessagesTopic {
  fn as_ref(&self) -> &SavedMessagesTopic {
    self
  }
}

impl AsRef<SavedMessagesTopic> for RTDSavedMessagesTopicBuilder {
  fn as_ref(&self) -> &SavedMessagesTopic {
    &self.inner
  }
}
