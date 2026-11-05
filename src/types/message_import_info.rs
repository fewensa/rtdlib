use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a message created with importMessages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageImportInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the original sender
  sender_name: String,
  /// Point in time (Unix timestamp) when the message was originally sent
  date: i64,
}

impl RObject for MessageImportInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageImportInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl MessageImportInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageImportInfoBuilder {
    let inner = MessageImportInfo {
      td_name: "messageImportInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageImportInfoBuilder { inner }
  }

  pub fn sender_name(&self) -> &String {
    &self.sender_name
  }

  pub fn date(&self) -> i64 {
    self.date
  }
}

#[doc(hidden)]
pub struct RTDMessageImportInfoBuilder {
  inner: MessageImportInfo,
}

impl RTDMessageImportInfoBuilder {
  pub fn build(&self) -> MessageImportInfo {
    self.inner.clone()
  }

  pub fn sender_name<T: AsRef<str>>(&mut self, sender_name: T) -> &mut Self {
    self.inner.sender_name = sender_name.as_ref().to_string();
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }
}

impl AsRef<MessageImportInfo> for MessageImportInfo {
  fn as_ref(&self) -> &MessageImportInfo {
    self
  }
}

impl AsRef<MessageImportInfo> for RTDMessageImportInfoBuilder {
  fn as_ref(&self) -> &MessageImportInfo {
    &self.inner
  }
}
