use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes manually chosen quote from another message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputTextQuote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text of the quote; 0-getOption("message_reply_quote_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities are allowed to be kept and must be kept in the quote. Ignored if rich_texts are non-empty
  text: FormattedText,
  /// Quote from a rich message; pass an empty array if the quote isn't chosen from a rich message. Must contain one element for each quoted text of the message in the order of their appearance. The first and the last elements can contain only a part of the corresponding text; all other elements must contain the whole text. Formatting of the quoted parts must be kept. Mathematical expressions must be quoted entirely. Buttons in pageBlockButtonRow, anchors and dividers can't be quoted. There can be up to 32 quoted texts with total length up to getOption("message_reply_quote_length_max") characters
  rich_texts: Vec<RichText>,
  /// Quote position in the original message in UTF-16 code units. For quotes from rich messages, it is the position in the text formed by joining plain texts of all quotable texts of the message, separated by line feeds
  position: i64,
}

impl RObject for InputTextQuote {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputTextQuote"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputTextQuote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputTextQuoteBuilder {
    let inner = InputTextQuote {
      td_name: "inputTextQuote".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputTextQuoteBuilder { inner }
  }

  pub fn text(&self) -> &FormattedText {
    &self.text
  }

  pub fn rich_texts(&self) -> &Vec<RichText> {
    &self.rich_texts
  }

  pub fn position(&self) -> i64 {
    self.position
  }
}

#[doc(hidden)]
pub struct RTDInputTextQuoteBuilder {
  inner: InputTextQuote,
}

impl RTDInputTextQuoteBuilder {
  pub fn build(&self) -> InputTextQuote {
    self.inner.clone()
  }

  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn rich_texts(&mut self, rich_texts: Vec<RichText>) -> &mut Self {
    self.inner.rich_texts = rich_texts;
    self
  }

  pub fn position(&mut self, position: i64) -> &mut Self {
    self.inner.position = position;
    self
  }
}

impl AsRef<InputTextQuote> for InputTextQuote {
  fn as_ref(&self) -> &InputTextQuote {
    self
  }
}

impl AsRef<InputTextQuote> for RTDInputTextQuoteBuilder {
  fn as_ref(&self) -> &InputTextQuote {
    &self.inner
  }
}
