use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a business chat link to create or edit
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputBusinessChatLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Message draft text that will be added to the input field
  text: FormattedText,
  /// Link title
  title: String,
}

impl RObject for InputBusinessChatLink {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputBusinessChatLink"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputBusinessChatLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputBusinessChatLinkBuilder {
    let inner = InputBusinessChatLink {
      td_name: "inputBusinessChatLink".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputBusinessChatLinkBuilder { inner }
  }

  pub fn text(&self) -> &FormattedText {
    &self.text
  }

  pub fn title(&self) -> &String {
    &self.title
  }
}

#[doc(hidden)]
pub struct RTDInputBusinessChatLinkBuilder {
  inner: InputBusinessChatLink,
}

impl RTDInputBusinessChatLinkBuilder {
  pub fn build(&self) -> InputBusinessChatLink {
    self.inner.clone()
  }

  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }
}

impl AsRef<InputBusinessChatLink> for InputBusinessChatLink {
  fn as_ref(&self) -> &InputBusinessChatLink {
    self
  }
}

impl AsRef<InputBusinessChatLink> for RTDInputBusinessChatLinkBuilder {
  fn as_ref(&self) -> &InputBusinessChatLink {
    &self.inner
  }
}
