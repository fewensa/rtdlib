use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a shortcut that can be used for a quick reply
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuickReplyShortcut {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique shortcut identifier
  id: i64,
  /// The name of the shortcut that can be used to use the shortcut
  name: String,
  /// The first shortcut message
  first_message: QuickReplyMessage,
  /// The total number of messages in the shortcut
  message_count: i64,
}

impl RObject for QuickReplyShortcut {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "quickReplyShortcut"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl QuickReplyShortcut {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDQuickReplyShortcutBuilder {
    let inner = QuickReplyShortcut {
      td_name: "quickReplyShortcut".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDQuickReplyShortcutBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn first_message(&self) -> &QuickReplyMessage {
    &self.first_message
  }

  pub fn message_count(&self) -> i64 {
    self.message_count
  }
}

#[doc(hidden)]
pub struct RTDQuickReplyShortcutBuilder {
  inner: QuickReplyShortcut,
}

impl RTDQuickReplyShortcutBuilder {
  pub fn build(&self) -> QuickReplyShortcut {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn first_message<T: AsRef<QuickReplyMessage>>(&mut self, first_message: T) -> &mut Self {
    self.inner.first_message = first_message.as_ref().clone();
    self
  }

  pub fn message_count(&mut self, message_count: i64) -> &mut Self {
    self.inner.message_count = message_count;
    self
  }
}

impl AsRef<QuickReplyShortcut> for QuickReplyShortcut {
  fn as_ref(&self) -> &QuickReplyShortcut {
    self
  }
}

impl AsRef<QuickReplyShortcut> for RTDQuickReplyShortcutBuilder {
  fn as_ref(&self) -> &QuickReplyShortcut {
    &self.inner
  }
}
