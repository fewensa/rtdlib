use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a business chat link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessChatLinkInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the private chat that created the link
  chat_id: i64,
  /// Message draft text that must be added to the input field
  text: FormattedText,
}

impl RObject for BusinessChatLinkInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessChatLinkInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BusinessChatLinkInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessChatLinkInfoBuilder {
    let inner = BusinessChatLinkInfo {
      td_name: "businessChatLinkInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBusinessChatLinkInfoBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn text(&self) -> &FormattedText {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDBusinessChatLinkInfoBuilder {
  inner: BusinessChatLinkInfo,
}

impl RTDBusinessChatLinkInfoBuilder {
  pub fn build(&self) -> BusinessChatLinkInfo {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }
}

impl AsRef<BusinessChatLinkInfo> for BusinessChatLinkInfo {
  fn as_ref(&self) -> &BusinessChatLinkInfo {
    self
  }
}

impl AsRef<BusinessChatLinkInfo> for RTDBusinessChatLinkInfoBuilder {
  fn as_ref(&self) -> &BusinessChatLinkInfo {
    &self.inner
  }
}
