use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains default auto-delete timer setting for new chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageAutoDeleteTime {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Message auto-delete time, in seconds. If 0, then messages aren't deleted automatically
  time: i64,
}

impl RObject for MessageAutoDeleteTime {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageAutoDeleteTime"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl MessageAutoDeleteTime {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageAutoDeleteTimeBuilder {
    let inner = MessageAutoDeleteTime {
      td_name: "messageAutoDeleteTime".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageAutoDeleteTimeBuilder { inner }
  }

  pub fn time(&self) -> i64 {
    self.time
  }
}

#[doc(hidden)]
pub struct RTDMessageAutoDeleteTimeBuilder {
  inner: MessageAutoDeleteTime,
}

impl RTDMessageAutoDeleteTimeBuilder {
  pub fn build(&self) -> MessageAutoDeleteTime {
    self.inner.clone()
  }

  pub fn time(&mut self, time: i64) -> &mut Self {
    self.inner.time = time;
    self
  }
}

impl AsRef<MessageAutoDeleteTime> for MessageAutoDeleteTime {
  fn as_ref(&self) -> &MessageAutoDeleteTime {
    self
  }
}

impl AsRef<MessageAutoDeleteTime> for RTDMessageAutoDeleteTimeBuilder {
  fn as_ref(&self) -> &MessageAutoDeleteTime {
    &self.inner
  }
}
