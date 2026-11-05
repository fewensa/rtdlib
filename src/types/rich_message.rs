use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a message with rich formatting
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Content of the message
  blocks: Vec<PageBlock>,
  /// True, if the message must be shown from right to left
  is_rtl: bool,
  /// True, if the object contains the full message. Otherwise, getFullRichMessage must be used to get the full message
  is_full: bool,
}

impl RObject for RichMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl RichMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichMessageBuilder {
    let inner = RichMessage {
      td_name: "richMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichMessageBuilder { inner }
  }

  pub fn blocks(&self) -> &Vec<PageBlock> {
    &self.blocks
  }

  pub fn is_rtl(&self) -> bool {
    self.is_rtl
  }

  pub fn is_full(&self) -> bool {
    self.is_full
  }
}

#[doc(hidden)]
pub struct RTDRichMessageBuilder {
  inner: RichMessage,
}

impl RTDRichMessageBuilder {
  pub fn build(&self) -> RichMessage {
    self.inner.clone()
  }

  pub fn blocks(&mut self, blocks: Vec<PageBlock>) -> &mut Self {
    self.inner.blocks = blocks;
    self
  }

  pub fn is_rtl(&mut self, is_rtl: bool) -> &mut Self {
    self.inner.is_rtl = is_rtl;
    self
  }

  pub fn is_full(&mut self, is_full: bool) -> &mut Self {
    self.inner.is_full = is_full;
    self
  }
}

impl AsRef<RichMessage> for RichMessage {
  fn as_ref(&self) -> &RichMessage {
    self
  }
}

impl AsRef<RichMessage> for RTDRichMessageBuilder {
  fn as_ref(&self) -> &RichMessage {
    &self.inner
  }
}
