use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains an example of text composition style usage
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextCompositionStyleExample {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Source text
  source_text: FormattedText,
  /// The text after the style was applied to the source text
  result_text: FormattedText,
}

impl RObject for TextCompositionStyleExample {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "textCompositionStyleExample"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TextCompositionStyleExample {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTextCompositionStyleExampleBuilder {
    let inner = TextCompositionStyleExample {
      td_name: "textCompositionStyleExample".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTextCompositionStyleExampleBuilder { inner }
  }

  pub fn source_text(&self) -> &FormattedText {
    &self.source_text
  }

  pub fn result_text(&self) -> &FormattedText {
    &self.result_text
  }
}

#[doc(hidden)]
pub struct RTDTextCompositionStyleExampleBuilder {
  inner: TextCompositionStyleExample,
}

impl RTDTextCompositionStyleExampleBuilder {
  pub fn build(&self) -> TextCompositionStyleExample {
    self.inner.clone()
  }

  pub fn source_text<T: AsRef<FormattedText>>(&mut self, source_text: T) -> &mut Self {
    self.inner.source_text = source_text.as_ref().clone();
    self
  }

  pub fn result_text<T: AsRef<FormattedText>>(&mut self, result_text: T) -> &mut Self {
    self.inner.result_text = result_text.as_ref().clone();
    self
  }
}

impl AsRef<TextCompositionStyleExample> for TextCompositionStyleExample {
  fn as_ref(&self) -> &TextCompositionStyleExample {
    self
  }
}

impl AsRef<TextCompositionStyleExample> for RTDTextCompositionStyleExampleBuilder {
  fn as_ref(&self) -> &TextCompositionStyleExample {
    &self.inner
  }
}
