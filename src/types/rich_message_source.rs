use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes source of a rich message
pub trait TDRichMessageSource: Debug + RObject {}

/// Describes source of a rich message
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum RichMessageSource {
  #[doc(hidden)]
  _Default(()),
  /// A rich message defined by blocks
  Blocks(RichMessageSourceBlocks),
  /// An HTML-formatted rich message; for bots only
  Html(RichMessageSourceHtml),
  /// A Markdown-formatted rich message; for bots only
  Markdown(RichMessageSourceMarkdown),
}

impl Default for RichMessageSource {
  fn default() -> Self {
    RichMessageSource::_Default(())
  }
}

impl<'de> Deserialize<'de> for RichMessageSource {
  fn deserialize<D>(deserializer: D) -> Result<RichMessageSource, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      RichMessageSource,
      (richMessageSourceBlocks, Blocks);
      (richMessageSourceHtml, Html);
      (richMessageSourceMarkdown, Markdown);

    )(deserializer)
  }
}

impl RObject for RichMessageSource {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      RichMessageSource::Blocks(t) => t.td_name(),
      RichMessageSource::Html(t) => t.td_name(),
      RichMessageSource::Markdown(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      RichMessageSource::Blocks(t) => t.extra(),
      RichMessageSource::Html(t) => t.extra(),
      RichMessageSource::Markdown(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl RichMessageSource {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, RichMessageSource::_Default(_))
  }

  pub fn is_blocks(&self) -> bool {
    matches!(self, RichMessageSource::Blocks(_))
  }
  pub fn is_html(&self) -> bool {
    matches!(self, RichMessageSource::Html(_))
  }
  pub fn is_markdown(&self) -> bool {
    matches!(self, RichMessageSource::Markdown(_))
  }

  pub fn on_blocks<F: FnOnce(&RichMessageSourceBlocks)>(&self, fnc: F) -> &Self {
    if let RichMessageSource::Blocks(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_html<F: FnOnce(&RichMessageSourceHtml)>(&self, fnc: F) -> &Self {
    if let RichMessageSource::Html(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_markdown<F: FnOnce(&RichMessageSourceMarkdown)>(&self, fnc: F) -> &Self {
    if let RichMessageSource::Markdown(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_blocks(&self) -> Option<&RichMessageSourceBlocks> {
    if let RichMessageSource::Blocks(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_html(&self) -> Option<&RichMessageSourceHtml> {
    if let RichMessageSource::Html(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_markdown(&self) -> Option<&RichMessageSourceMarkdown> {
    if let RichMessageSource::Markdown(t) = self {
      return Some(t);
    }
    None
  }

  pub fn blocks<T: AsRef<RichMessageSourceBlocks>>(t: T) -> Self {
    RichMessageSource::Blocks(t.as_ref().clone())
  }

  pub fn html<T: AsRef<RichMessageSourceHtml>>(t: T) -> Self {
    RichMessageSource::Html(t.as_ref().clone())
  }

  pub fn markdown<T: AsRef<RichMessageSourceMarkdown>>(t: T) -> Self {
    RichMessageSource::Markdown(t.as_ref().clone())
  }
}

impl AsRef<RichMessageSource> for RichMessageSource {
  fn as_ref(&self) -> &RichMessageSource {
    self
  }
}

/// A rich message defined by blocks
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichMessageSourceBlocks {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Content of the message
  blocks: Vec<InputPageBlock>,
}

impl RObject for RichMessageSourceBlocks {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richMessageSourceBlocks"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichMessageSource for RichMessageSourceBlocks {}

impl RichMessageSourceBlocks {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichMessageSourceBlocksBuilder {
    let inner = RichMessageSourceBlocks {
      td_name: "richMessageSourceBlocks".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichMessageSourceBlocksBuilder { inner }
  }

  pub fn blocks(&self) -> &Vec<InputPageBlock> {
    &self.blocks
  }
}

#[doc(hidden)]
pub struct RTDRichMessageSourceBlocksBuilder {
  inner: RichMessageSourceBlocks,
}

impl RTDRichMessageSourceBlocksBuilder {
  pub fn build(&self) -> RichMessageSourceBlocks {
    self.inner.clone()
  }

  pub fn blocks(&mut self, blocks: Vec<InputPageBlock>) -> &mut Self {
    self.inner.blocks = blocks;
    self
  }
}

impl AsRef<RichMessageSourceBlocks> for RichMessageSourceBlocks {
  fn as_ref(&self) -> &RichMessageSourceBlocks {
    self
  }
}

impl AsRef<RichMessageSourceBlocks> for RTDRichMessageSourceBlocksBuilder {
  fn as_ref(&self) -> &RichMessageSourceBlocks {
    &self.inner
  }
}

/// An HTML-formatted rich message; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichMessageSourceHtml {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// HTML-formatted text of the message
  text: String,
  /// Media used in the message
  media: Vec<InputRichMessageMedia>,
}

impl RObject for RichMessageSourceHtml {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richMessageSourceHtml"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichMessageSource for RichMessageSourceHtml {}

impl RichMessageSourceHtml {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichMessageSourceHtmlBuilder {
    let inner = RichMessageSourceHtml {
      td_name: "richMessageSourceHtml".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichMessageSourceHtmlBuilder { inner }
  }

  pub fn text(&self) -> &String {
    &self.text
  }

  pub fn media(&self) -> &Vec<InputRichMessageMedia> {
    &self.media
  }
}

#[doc(hidden)]
pub struct RTDRichMessageSourceHtmlBuilder {
  inner: RichMessageSourceHtml,
}

impl RTDRichMessageSourceHtmlBuilder {
  pub fn build(&self) -> RichMessageSourceHtml {
    self.inner.clone()
  }

  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }

  pub fn media(&mut self, media: Vec<InputRichMessageMedia>) -> &mut Self {
    self.inner.media = media;
    self
  }
}

impl AsRef<RichMessageSourceHtml> for RichMessageSourceHtml {
  fn as_ref(&self) -> &RichMessageSourceHtml {
    self
  }
}

impl AsRef<RichMessageSourceHtml> for RTDRichMessageSourceHtmlBuilder {
  fn as_ref(&self) -> &RichMessageSourceHtml {
    &self.inner
  }
}

/// A Markdown-formatted rich message; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichMessageSourceMarkdown {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Markdown-formatted text of the message
  text: String,
  /// Media used in the message
  media: Vec<InputRichMessageMedia>,
}

impl RObject for RichMessageSourceMarkdown {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richMessageSourceMarkdown"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichMessageSource for RichMessageSourceMarkdown {}

impl RichMessageSourceMarkdown {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichMessageSourceMarkdownBuilder {
    let inner = RichMessageSourceMarkdown {
      td_name: "richMessageSourceMarkdown".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichMessageSourceMarkdownBuilder { inner }
  }

  pub fn text(&self) -> &String {
    &self.text
  }

  pub fn media(&self) -> &Vec<InputRichMessageMedia> {
    &self.media
  }
}

#[doc(hidden)]
pub struct RTDRichMessageSourceMarkdownBuilder {
  inner: RichMessageSourceMarkdown,
}

impl RTDRichMessageSourceMarkdownBuilder {
  pub fn build(&self) -> RichMessageSourceMarkdown {
    self.inner.clone()
  }

  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }

  pub fn media(&mut self, media: Vec<InputRichMessageMedia>) -> &mut Self {
    self.inner.media = media;
    self
  }
}

impl AsRef<RichMessageSourceMarkdown> for RichMessageSourceMarkdown {
  fn as_ref(&self) -> &RichMessageSourceMarkdown {
    self
  }
}

impl AsRef<RichMessageSourceMarkdown> for RTDRichMessageSourceMarkdownBuilder {
  fn as_ref(&self) -> &RichMessageSourceMarkdown {
    &self.inner
  }
}
