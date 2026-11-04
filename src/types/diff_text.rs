use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A text with some changes highlighted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiffText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The text
  text: String,
  /// Entities describing changes in the text. Entities don't mutually intersect with each other
  entities: Vec<DiffEntity>,
}

impl RObject for DiffText {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "diffText"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl DiffText {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDiffTextBuilder {
    let inner = DiffText {
      td_name: "diffText".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDiffTextBuilder { inner }
  }

  pub fn text(&self) -> &String {
    &self.text
  }

  pub fn entities(&self) -> &Vec<DiffEntity> {
    &self.entities
  }
}

#[doc(hidden)]
pub struct RTDDiffTextBuilder {
  inner: DiffText,
}

impl RTDDiffTextBuilder {
  pub fn build(&self) -> DiffText {
    self.inner.clone()
  }

  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }

  pub fn entities(&mut self, entities: Vec<DiffEntity>) -> &mut Self {
    self.inner.entities = entities;
    self
  }
}

impl AsRef<DiffText> for DiffText {
  fn as_ref(&self) -> &DiffText {
    self
  }
}

impl AsRef<DiffText> for RTDDiffTextBuilder {
  fn as_ref(&self) -> &DiffText {
    &self.inner
  }
}
