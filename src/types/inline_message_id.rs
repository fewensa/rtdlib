use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains identifier of a sent guest message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineMessageId {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier for the message
  id: String,
}

impl RObject for InlineMessageId {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inlineMessageId"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InlineMessageId {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInlineMessageIdBuilder {
    let inner = InlineMessageId {
      td_name: "inlineMessageId".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInlineMessageIdBuilder { inner }
  }

  pub fn id(&self) -> &String {
    &self.id
  }
}

#[doc(hidden)]
pub struct RTDInlineMessageIdBuilder {
  inner: InlineMessageId,
}

impl RTDInlineMessageIdBuilder {
  pub fn build(&self) -> InlineMessageId {
    self.inner.clone()
  }

  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }
}

impl AsRef<InlineMessageId> for InlineMessageId {
  fn as_ref(&self) -> &InlineMessageId {
    self
  }
}

impl AsRef<InlineMessageId> for RTDInlineMessageIdBuilder {
  fn as_ref(&self) -> &InlineMessageId {
    &self.inner
  }
}
