use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A text fixed using fixTextWithAi
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FixedText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The resulting text
  text: FormattedText,
  /// Changes made to the original text
  diff_text: DiffText,
}

impl RObject for FixedText {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fixedText"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl FixedText {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFixedTextBuilder {
    let inner = FixedText {
      td_name: "fixedText".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDFixedTextBuilder { inner }
  }

  pub fn text(&self) -> &FormattedText {
    &self.text
  }

  pub fn diff_text(&self) -> &DiffText {
    &self.diff_text
  }
}

#[doc(hidden)]
pub struct RTDFixedTextBuilder {
  inner: FixedText,
}

impl RTDFixedTextBuilder {
  pub fn build(&self) -> FixedText {
    self.inner.clone()
  }

  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn diff_text<T: AsRef<DiffText>>(&mut self, diff_text: T) -> &mut Self {
    self.inner.diff_text = diff_text.as_ref().clone();
    self
  }
}

impl AsRef<FixedText> for FixedText {
  fn as_ref(&self) -> &FixedText {
    self
  }
}

impl AsRef<FixedText> for RTDFixedTextBuilder {
  fn as_ref(&self) -> &FixedText {
    &self.inner
  }
}
