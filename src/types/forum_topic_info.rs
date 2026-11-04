use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains basic information about a forum topic
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ForumTopicInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of a forum supergroup chat or a chat with a bot to which the topic belongs
  chat_id: i64,
  /// Forum topic identifier of the topic
  forum_topic_id: i64,
  /// Name of the topic
  name: String,
  /// Icon of the topic
  icon: ForumTopicIcon,
  /// Point in time (Unix timestamp) when the topic was created
  creation_date: i64,
  /// Identifier of the creator of the topic
  creator_id: MessageSender,
  /// True, if the topic is the General topic
  is_general: bool,
  /// True, if the topic was created by the current user
  is_outgoing: bool,
  /// True, if the topic is closed. If the topic is closed, then the user must have can_manage_topics administrator right in the supergroup or must be the creator of the topic to send messages there
  is_closed: bool,
  /// True, if the topic is hidden above the topic list and closed; for General topic only
  is_hidden: bool,
  /// True, if the name of the topic wasn't added explicitly
  is_name_implicit: bool,
}

impl RObject for ForumTopicInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "forumTopicInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ForumTopicInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDForumTopicInfoBuilder {
    let inner = ForumTopicInfo {
      td_name: "forumTopicInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDForumTopicInfoBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn forum_topic_id(&self) -> i64 {
    self.forum_topic_id
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn icon(&self) -> &ForumTopicIcon {
    &self.icon
  }

  pub fn creation_date(&self) -> i64 {
    self.creation_date
  }

  pub fn creator_id(&self) -> &MessageSender {
    &self.creator_id
  }

  pub fn is_general(&self) -> bool {
    self.is_general
  }

  pub fn is_outgoing(&self) -> bool {
    self.is_outgoing
  }

  pub fn is_closed(&self) -> bool {
    self.is_closed
  }

  pub fn is_hidden(&self) -> bool {
    self.is_hidden
  }

  pub fn is_name_implicit(&self) -> bool {
    self.is_name_implicit
  }
}

#[doc(hidden)]
pub struct RTDForumTopicInfoBuilder {
  inner: ForumTopicInfo,
}

impl RTDForumTopicInfoBuilder {
  pub fn build(&self) -> ForumTopicInfo {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn forum_topic_id(&mut self, forum_topic_id: i64) -> &mut Self {
    self.inner.forum_topic_id = forum_topic_id;
    self
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn icon<T: AsRef<ForumTopicIcon>>(&mut self, icon: T) -> &mut Self {
    self.inner.icon = icon.as_ref().clone();
    self
  }

  pub fn creation_date(&mut self, creation_date: i64) -> &mut Self {
    self.inner.creation_date = creation_date;
    self
  }

  pub fn creator_id<T: AsRef<MessageSender>>(&mut self, creator_id: T) -> &mut Self {
    self.inner.creator_id = creator_id.as_ref().clone();
    self
  }

  pub fn is_general(&mut self, is_general: bool) -> &mut Self {
    self.inner.is_general = is_general;
    self
  }

  pub fn is_outgoing(&mut self, is_outgoing: bool) -> &mut Self {
    self.inner.is_outgoing = is_outgoing;
    self
  }

  pub fn is_closed(&mut self, is_closed: bool) -> &mut Self {
    self.inner.is_closed = is_closed;
    self
  }

  pub fn is_hidden(&mut self, is_hidden: bool) -> &mut Self {
    self.inner.is_hidden = is_hidden;
    self
  }

  pub fn is_name_implicit(&mut self, is_name_implicit: bool) -> &mut Self {
    self.inner.is_name_implicit = is_name_implicit;
    self
  }
}

impl AsRef<ForumTopicInfo> for ForumTopicInfo {
  fn as_ref(&self) -> &ForumTopicInfo {
    self
  }
}

impl AsRef<ForumTopicInfo> for RTDForumTopicInfoBuilder {
  fn as_ref(&self) -> &ForumTopicInfo {
    &self.inner
  }
}
