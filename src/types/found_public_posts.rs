use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of messages found by a public post search
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoundPublicPosts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of found public posts
  messages: Vec<Message>,
  /// The offset for the next request. If empty, then there are no more results
  next_offset: String,
  /// Updated public post search limits after the query; repeated requests with the same query will be free; may be null if they didn't change
  search_limits: Option<PublicPostSearchLimits>,
  /// True, if the query has failed because search limits are exceeded. In this case search_limits.daily_free_query_count will be equal to 0
  are_limits_exceeded: bool,
}

impl RObject for FoundPublicPosts {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "foundPublicPosts"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl FoundPublicPosts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFoundPublicPostsBuilder {
    let inner = FoundPublicPosts {
      td_name: "foundPublicPosts".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDFoundPublicPostsBuilder { inner }
  }

  pub fn messages(&self) -> &Vec<Message> {
    &self.messages
  }

  pub fn next_offset(&self) -> &String {
    &self.next_offset
  }

  pub fn search_limits(&self) -> &Option<PublicPostSearchLimits> {
    &self.search_limits
  }

  pub fn are_limits_exceeded(&self) -> bool {
    self.are_limits_exceeded
  }
}

#[doc(hidden)]
pub struct RTDFoundPublicPostsBuilder {
  inner: FoundPublicPosts,
}

impl RTDFoundPublicPostsBuilder {
  pub fn build(&self) -> FoundPublicPosts {
    self.inner.clone()
  }

  pub fn messages(&mut self, messages: Vec<Message>) -> &mut Self {
    self.inner.messages = messages;
    self
  }

  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }

  pub fn search_limits<T: AsRef<PublicPostSearchLimits>>(&mut self, search_limits: T) -> &mut Self {
    self.inner.search_limits = Some(search_limits.as_ref().clone());
    self
  }

  pub fn are_limits_exceeded(&mut self, are_limits_exceeded: bool) -> &mut Self {
    self.inner.are_limits_exceeded = are_limits_exceeded;
    self
  }
}

impl AsRef<FoundPublicPosts> for FoundPublicPosts {
  fn as_ref(&self) -> &FoundPublicPosts {
    self
  }
}

impl AsRef<FoundPublicPosts> for RTDFoundPublicPostsBuilder {
  fn as_ref(&self) -> &FoundPublicPosts {
    &self.inner
  }
}
