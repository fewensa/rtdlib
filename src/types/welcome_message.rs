use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a set up welcome message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WelcomeMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Welcome message identifier; unique for the chat to which the welcome message belongs
  id: i64,
  /// Content of the welcome message
  content: MessageContent,
}

impl RObject for WelcomeMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "welcomeMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl WelcomeMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDWelcomeMessageBuilder {
    let inner = WelcomeMessage {
      td_name: "welcomeMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDWelcomeMessageBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn content(&self) -> &MessageContent {
    &self.content
  }
}

#[doc(hidden)]
pub struct RTDWelcomeMessageBuilder {
  inner: WelcomeMessage,
}

impl RTDWelcomeMessageBuilder {
  pub fn build(&self) -> WelcomeMessage {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn content<T: AsRef<MessageContent>>(&mut self, content: T) -> &mut Self {
    self.inner.content = content.as_ref().clone();
    self
  }
}

impl AsRef<WelcomeMessage> for WelcomeMessage {
  fn as_ref(&self) -> &WelcomeMessage {
    self
  }
}

impl AsRef<WelcomeMessage> for RTDWelcomeMessageBuilder {
  fn as_ref(&self) -> &WelcomeMessage {
    &self.inner
  }
}
