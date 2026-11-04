use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a chat shared with a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SharedChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Title of the chat; for bots only
  title: String,
  /// Username of the chat; for bots only
  username: String,
  /// Photo of the chat; for bots only; may be null
  photo: Option<Photo>,
}

impl RObject for SharedChat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sharedChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SharedChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSharedChatBuilder {
    let inner = SharedChat {
      td_name: "sharedChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSharedChatBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn username(&self) -> &String {
    &self.username
  }

  pub fn photo(&self) -> &Option<Photo> {
    &self.photo
  }
}

#[doc(hidden)]
pub struct RTDSharedChatBuilder {
  inner: SharedChat,
}

impl RTDSharedChatBuilder {
  pub fn build(&self) -> SharedChat {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
    self.inner.username = username.as_ref().to_string();
    self
  }

  pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }
}

impl AsRef<SharedChat> for SharedChat {
  fn as_ref(&self) -> &SharedChat {
    self
  }
}

impl AsRef<SharedChat> for RTDSharedChatBuilder {
  fn as_ref(&self) -> &SharedChat {
    &self.inner
  }
}
