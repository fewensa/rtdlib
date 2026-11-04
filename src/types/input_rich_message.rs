use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A rich message to send. Total length of all texts, including custom emoji alternative text and formula source, must not exceed getOption("rich_message_text_length_max"). The total number of all blocks, list items and table rows must not exceed getOption("rich_message_block_count_max"). The maximum allowed depth of nested blocks and rich texts is getOption("rich_message_depth_max"). The total number of media in all blocks must not exceed getOption("rich_message_media_count_max"). The maximum allowed number of table columns is getOption("rich_message_table_column_count_max")
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputRichMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Source of the rich message
  source: RichMessageSource,
  /// Pass true if the message must be shown from right to left
  is_rtl: bool,
  /// Pass true to enable detection of URLs, email addresses and other automatic blocks
  detect_automatic_blocks: bool,
}

impl RObject for InputRichMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputRichMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputRichMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputRichMessageBuilder {
    let inner = InputRichMessage {
      td_name: "inputRichMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputRichMessageBuilder { inner }
  }

  pub fn source(&self) -> &RichMessageSource {
    &self.source
  }

  pub fn is_rtl(&self) -> bool {
    self.is_rtl
  }

  pub fn detect_automatic_blocks(&self) -> bool {
    self.detect_automatic_blocks
  }
}

#[doc(hidden)]
pub struct RTDInputRichMessageBuilder {
  inner: InputRichMessage,
}

impl RTDInputRichMessageBuilder {
  pub fn build(&self) -> InputRichMessage {
    self.inner.clone()
  }

  pub fn source<T: AsRef<RichMessageSource>>(&mut self, source: T) -> &mut Self {
    self.inner.source = source.as_ref().clone();
    self
  }

  pub fn is_rtl(&mut self, is_rtl: bool) -> &mut Self {
    self.inner.is_rtl = is_rtl;
    self
  }

  pub fn detect_automatic_blocks(&mut self, detect_automatic_blocks: bool) -> &mut Self {
    self.inner.detect_automatic_blocks = detect_automatic_blocks;
    self
  }
}

impl AsRef<InputRichMessage> for InputRichMessage {
  fn as_ref(&self) -> &InputRichMessage {
    self
  }
}

impl AsRef<InputRichMessage> for RTDInputRichMessageBuilder {
  fn as_ref(&self) -> &InputRichMessage {
    &self.inner
  }
}
