use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes manually or automatically chosen quote from another message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextQuote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text of the quote. Only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities can be present in the text. May be empty if the quote was chosen from a rich message
  text: FormattedText,
  /// Rich text of the quote if it was chosen from a rich message; empty if the quote wasn't chosen from a rich message. Only richTextPlain, richTextBold, richTextItalic, richTextUnderline, richTextStrikethrough, richTextSpoiler, richTextSubscript, richTextSuperscript, richTextDateTime, richTextCustomEmoji, richTextMathematicalExpression, and richTexts can be present in the texts
  rich_texts: Vec<RichText>,
  /// Approximate quote position in the original message in UTF-16 code units as specified by the message sender. For quotes from rich messages, it is the position in the text formed by joining plain texts of all quotable texts of the message, separated by line feeds
  position: i64,
  /// True, if the quote was manually chosen by the message sender
  is_manual: bool,
}

impl RObject for TextQuote {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "textQuote"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TextQuote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTextQuoteBuilder {
    let inner = TextQuote {
      td_name: "textQuote".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTextQuoteBuilder { inner }
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

  pub fn is_manual(&self) -> bool {
    self.is_manual
  }
}

#[doc(hidden)]
pub struct RTDTextQuoteBuilder {
  inner: TextQuote,
}

impl RTDTextQuoteBuilder {
  pub fn build(&self) -> TextQuote {
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

  pub fn is_manual(&mut self, is_manual: bool) -> &mut Self {
    self.inner.is_manual = is_manual;
    self
  }
}

impl AsRef<TextQuote> for TextQuote {
  fn as_ref(&self) -> &TextQuote {
    self
  }
}

impl AsRef<TextQuote> for RTDTextQuoteBuilder {
  fn as_ref(&self) -> &TextQuote {
    &self.inner
  }
}
