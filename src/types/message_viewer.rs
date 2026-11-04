use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a viewer of a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageViewer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier of the viewer
  user_id: i64,
  /// Approximate point in time (Unix timestamp) when the message was viewed
  view_date: i64,
}

impl RObject for MessageViewer {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageViewer"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl MessageViewer {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageViewerBuilder {
    let inner = MessageViewer {
      td_name: "messageViewer".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageViewerBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn view_date(&self) -> i64 {
    self.view_date
  }
}

#[doc(hidden)]
pub struct RTDMessageViewerBuilder {
  inner: MessageViewer,
}

impl RTDMessageViewerBuilder {
  pub fn build(&self) -> MessageViewer {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn view_date(&mut self, view_date: i64) -> &mut Self {
    self.inner.view_date = view_date;
    self
  }
}

impl AsRef<MessageViewer> for MessageViewer {
  fn as_ref(&self) -> &MessageViewer {
    self
  }
}

impl AsRef<MessageViewer> for RTDMessageViewerBuilder {
  fn as_ref(&self) -> &MessageViewer {
    &self.inner
  }
}
