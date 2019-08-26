
use crate::types::*;
use crate::errors::*;




/// A text with some entities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormattedText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// The text
  text: String,
  /// Entities contained in the text
  entities: Vec<TextEntity>,
  
}

impl RObject for FormattedText {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "formattedText" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl FormattedText {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDFormattedTextBuilder {
    let mut inner = FormattedText::default();
    inner.td_name = "formattedText".to_string();
    RTDFormattedTextBuilder { inner }
  }

  pub fn text(&self) -> &String { &self.text }

  pub fn entities(&self) -> &Vec<TextEntity> { &self.entities }

}

#[doc(hidden)]
pub struct RTDFormattedTextBuilder {
  inner: FormattedText
}

impl RTDFormattedTextBuilder {
  pub fn build(&self) -> FormattedText { self.inner.clone() }

   
  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }

   
  pub fn entities(&mut self, entities: Vec<TextEntity>) -> &mut Self {
    self.inner.entities = entities;
    self
  }

}

impl AsRef<FormattedText> for FormattedText {
  fn as_ref(&self) -> &FormattedText { self }
}

impl AsRef<FormattedText> for RTDFormattedTextBuilder {
  fn as_ref(&self) -> &FormattedText { &self.inner }
}



