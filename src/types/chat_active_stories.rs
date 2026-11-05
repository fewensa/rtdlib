use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes active stories posted by a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActiveStories {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat that posted the stories
  chat_id: i64,
  /// Identifier of the story list in which the stories are shown; may be null if the stories aren't shown in a story list
  list: Option<StoryList>,
  /// A parameter used to determine order of the stories in the story list; 0 if the stories don't need to be shown in the story list. Stories must be sorted by the pair (order, story_poster_chat_id) in descending order
  order: i64,
  /// True, if the stories are shown in the main story list and can be archived; otherwise, the stories can be hidden from the main story list only by calling removeTopChat with topChatCategoryUsers and the chat_id. Stories of the current user can't be archived nor hidden using removeTopChat
  can_be_archived: bool,
  /// Identifier of the last read active story
  max_read_story_id: i64,
  /// Basic information about the stories; use getStory to get full information about the stories. The stories are in chronological order (i.e., in order of increasing story identifiers)
  stories: Vec<StoryInfo>,
}

impl RObject for ChatActiveStories {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatActiveStories"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatActiveStories {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatActiveStoriesBuilder {
    let inner = ChatActiveStories {
      td_name: "chatActiveStories".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatActiveStoriesBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn list(&self) -> &Option<StoryList> {
    &self.list
  }

  pub fn order(&self) -> i64 {
    self.order
  }

  pub fn can_be_archived(&self) -> bool {
    self.can_be_archived
  }

  pub fn max_read_story_id(&self) -> i64 {
    self.max_read_story_id
  }

  pub fn stories(&self) -> &Vec<StoryInfo> {
    &self.stories
  }
}

#[doc(hidden)]
pub struct RTDChatActiveStoriesBuilder {
  inner: ChatActiveStories,
}

impl RTDChatActiveStoriesBuilder {
  pub fn build(&self) -> ChatActiveStories {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn list<T: AsRef<StoryList>>(&mut self, list: T) -> &mut Self {
    self.inner.list = Some(list.as_ref().clone());
    self
  }

  pub fn order(&mut self, order: i64) -> &mut Self {
    self.inner.order = order;
    self
  }

  pub fn can_be_archived(&mut self, can_be_archived: bool) -> &mut Self {
    self.inner.can_be_archived = can_be_archived;
    self
  }

  pub fn max_read_story_id(&mut self, max_read_story_id: i64) -> &mut Self {
    self.inner.max_read_story_id = max_read_story_id;
    self
  }

  pub fn stories(&mut self, stories: Vec<StoryInfo>) -> &mut Self {
    self.inner.stories = stories;
    self
  }
}

impl AsRef<ChatActiveStories> for ChatActiveStories {
  fn as_ref(&self) -> &ChatActiveStories {
    self
  }
}

impl AsRef<ChatActiveStories> for RTDChatActiveStoriesBuilder {
  fn as_ref(&self) -> &ChatActiveStories {
    &self.inner
  }
}
