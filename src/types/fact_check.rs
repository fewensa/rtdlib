use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a fact-check added to the message by an independent checker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FactCheck {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text of the fact-check
  text: FormattedText,
  /// A two-letter ISO 3166-1 alpha-2 country code of the country for which the fact-check is shown
  country_code: String,
}

impl RObject for FactCheck {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "factCheck"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl FactCheck {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFactCheckBuilder {
    let inner = FactCheck {
      td_name: "factCheck".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDFactCheckBuilder { inner }
  }

  pub fn text(&self) -> &FormattedText {
    &self.text
  }

  pub fn country_code(&self) -> &String {
    &self.country_code
  }
}

#[doc(hidden)]
pub struct RTDFactCheckBuilder {
  inner: FactCheck,
}

impl RTDFactCheckBuilder {
  pub fn build(&self) -> FactCheck {
    self.inner.clone()
  }

  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn country_code<T: AsRef<str>>(&mut self, country_code: T) -> &mut Self {
    self.inner.country_code = country_code.as_ref().to_string();
    self
  }
}

impl AsRef<FactCheck> for FactCheck {
  fn as_ref(&self) -> &FactCheck {
    self
  }
}

impl AsRef<FactCheck> for RTDFactCheckBuilder {
  fn as_ref(&self) -> &FactCheck {
    &self.inner
  }
}
