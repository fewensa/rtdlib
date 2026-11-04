use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a business chat link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessChatLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The HTTPS link
  link: String,
  /// Message draft text that will be added to the input field
  text: FormattedText,
  /// Link title
  title: String,
  /// Number of times the link was used
  view_count: i64,
}

impl RObject for BusinessChatLink {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessChatLink"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BusinessChatLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessChatLinkBuilder {
    let inner = BusinessChatLink {
      td_name: "businessChatLink".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBusinessChatLinkBuilder { inner }
  }

  pub fn link(&self) -> &String {
    &self.link
  }

  pub fn text(&self) -> &FormattedText {
    &self.text
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn view_count(&self) -> i64 {
    self.view_count
  }
}

#[doc(hidden)]
pub struct RTDBusinessChatLinkBuilder {
  inner: BusinessChatLink,
}

impl RTDBusinessChatLinkBuilder {
  pub fn build(&self) -> BusinessChatLink {
    self.inner.clone()
  }

  pub fn link<T: AsRef<str>>(&mut self, link: T) -> &mut Self {
    self.inner.link = link.as_ref().to_string();
    self
  }

  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn view_count(&mut self, view_count: i64) -> &mut Self {
    self.inner.view_count = view_count;
    self
  }
}

impl AsRef<BusinessChatLink> for BusinessChatLink {
  fn as_ref(&self) -> &BusinessChatLink {
    self
  }
}

impl AsRef<BusinessChatLink> for RTDBusinessChatLinkBuilder {
  fn as_ref(&self) -> &BusinessChatLink {
    &self.inner
  }
}
