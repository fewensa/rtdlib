use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a button to be shown above inline query results
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultsButton {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The text of the button
  text: String,
  /// Type of the button
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: InlineQueryResultsButtonType,
}

impl RObject for InlineQueryResultsButton {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inlineQueryResultsButton"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InlineQueryResultsButton {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInlineQueryResultsButtonBuilder {
    let inner = InlineQueryResultsButton {
      td_name: "inlineQueryResultsButton".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInlineQueryResultsButtonBuilder { inner }
  }

  pub fn text(&self) -> &String {
    &self.text
  }

  pub fn type_(&self) -> &InlineQueryResultsButtonType {
    &self.type_
  }
}

#[doc(hidden)]
pub struct RTDInlineQueryResultsButtonBuilder {
  inner: InlineQueryResultsButton,
}

impl RTDInlineQueryResultsButtonBuilder {
  pub fn build(&self) -> InlineQueryResultsButton {
    self.inner.clone()
  }

  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }

  pub fn type_<T: AsRef<InlineQueryResultsButtonType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }
}

impl AsRef<InlineQueryResultsButton> for InlineQueryResultsButton {
  fn as_ref(&self) -> &InlineQueryResultsButton {
    self
  }
}

impl AsRef<InlineQueryResultsButton> for RTDInlineQueryResultsButtonBuilder {
  fn as_ref(&self) -> &InlineQueryResultsButton {
    &self.inner
  }
}
